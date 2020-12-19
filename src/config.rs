use std::path::PathBuf;
use structopt::StructOpt;
use crate::state::{AppStateRaw, PoolOptions, State};
use std::sync::Arc;
use nonblock_logger::{JoinHandle, BaseFormater, Formater, FixedLevel, BaseFilter, NonblockLogger};
use log::{LevelFilter, Record};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Config {
    pub sql: String,
    pub redis: String
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct DbOptions {
    timeout: u64,
    #[serde(default)]
    timezone: String,
}

impl Config {
    pub fn parse_from_file(file: &PathBuf) -> Self {
        use std::fs::read_to_string;
        info!("config file:{}", file.display());
        let confstr = read_to_string(file).expect("config file find");
        json5::from_str(&confstr).expect("config file deser")
    }

    pub async fn into_state(self) -> AppStateRaw {
        info!("config {:?}", self);
        let mut pool_options = PoolOptions::new();
        if let Some(confstr) = url::Url::parse(&self.sql)
            .expect("Invalid DataSource url")
            .query()
        {
            if let Some(conf) = serde_qs::from_str::<DbOptions>(confstr)
                .map_err(|e| error!("serde_qs::from_str::<DbOptions> failed:{}", e))
                .ok()
            {
                pool_options = pool_options.connect_timeout(std::time::Duration::from_secs(conf.timeout));
                if !conf.timezone.is_empty() {
                    let key =  "@@session.time_zone =";
                    let zone = std::format!("set {} '{}'", key, conf.timezone.clone());

                    let zone_str = unsafe { std::mem::transmute::<_, &'static str>(zone.as_str()) };
                    std::mem::forget(zone);
                    pool_options = pool_options.after_connect(move |conn| {
                        Box::pin(async move {
                            use crate::sqlx::Executor;
                            conn.execute(zone_str).await.map(|_| ())
                        })
                    })
                }
            }
        }

        let sql = pool_options.connect(&self.sql).await.expect("sql open");
        Arc::new(State {
            config: self,
            sql
        })
    }
}

pub fn version_with_info() -> &'static str {
    concat!("shdsh"
    // env!("CARGO_PKG_VERSION"),
    // "",
    // env!("VERGEN_COMMIT_DATE"),
    // "",
    // env!("VERGEN_SHA_SHORT")
    )
}


#[derive(StructOpt, Debug)]
#[structopt(version = version_with_info())]
pub struct Opt {
    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,

    /// Output file
    #[structopt(
    short = "c",
    long = "config",
    parse(from_os_str),
    default_value = "template.json"
    )]
    pub config: PathBuf,
}

impl Opt {
    pub fn parse_from_args() -> (JoinHandle, Self) {
        let opt: Self = Opt::from_args();
        let level = match opt.verbose {
            0 => LevelFilter::Warn,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            _other => LevelFilter::Trace,
        };

        let formater = BaseFormater::new()
            .local(true)
            .color(true)
            .level(4)
            .formater(format);
        let filter = BaseFilter::new()
            .starts_with(true)
            .notfound(true)
            .max_level(level)
            .chain("sqlx", LevelFilter::Warn);
        let handle = NonblockLogger::new()
            .filter(filter)
            .unwrap()
            .formater(formater)
            .log_to_stdout()
            .map_err(|e|eprint!("failed to init nonblock_logger: {:?}", e))
            .unwrap();

        info!("opt: {:?}", opt);

        (handle, opt)

    }
}

pub fn format(base: &BaseFormater, record: &Record) -> String {
    let level = FixedLevel::with_color(record.level(), base.color_get())
        .length(base.level_get())
        .into_colored()
        .into_coloredfg();
    format!(
        "[{} {}#{}:{} {}] {}\n",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S.%sf"),
        level,
        record.module_path().unwrap_or("*"),
        record.line().unwrap_or(0),
        nonblock_logger::current_thread_name(),
        record.args()
    )
}