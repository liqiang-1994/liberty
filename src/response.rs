use std::borrow::Cow;
use serde::Serialize;
use actix_web::{HttpRequest, HttpResponse, Error, ResponseError, Responder};
use std::fmt::{self, Debug, Display};
use serde::export::Formatter;
use actix_web::http::StatusCode;
use actix_web::body::Body;
use futures::future::{ready, Ready};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseEntity<T = ()> {
    pub code: i32,
    pub msg: Option<Cow<'static, str>>,
    pub data: Option<T>,
}

impl <T: Serialize> ResponseEntity<T> {
    pub fn new() -> Self {
        Self{
            code: 200,
            msg: None,
            data: None,
        }
    }

    pub fn code(mut self, code: i32) -> Self {
        self.code = code;
        self
    }

    pub fn with_msg<S: Into<Cow<'static, str>>>(mut self, msg: S) -> Self {
        self.msg = Some(msg.into());
        self
    }

    pub fn msg_as_str(&self) -> &str {
        self.msg.as_ref().map(|s|s.as_ref()).unwrap_or_default()
    }

    pub fn with_data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }

    pub fn log(&self, req: &HttpRequest) {
        info!(
            "{}\"{} {} {:?}\" {}",
            req.peer_addr().unwrap(),
            req.method(),
            req.uri(),
            req.version(),
            self.code
        );
    }

    pub fn to_resp(&self) -> HttpResponse {
        let resp = match serde_json::to_string(self) {
            Ok(json) => HttpResponse::Ok()
                .content_type("application/json")
                .body(json),
            Err(e) => Error::from(e).into(),
        };

        resp
    }

    pub fn log_to_resp(&self, req: &HttpRequest) -> HttpResponse {
        self.log(req);
        self.to_resp()
    }
}

impl <T: Debug + Serialize> Display for ResponseEntity<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
       write!(f, "{:?}", self)
    }
}

pub type ResponseErr = ResponseEntity<()>;
impl <T: Debug + Serialize> ResponseError for ResponseEntity<T> {
    fn status_code(&self) -> StatusCode {
        StatusCode::OK
    }

    fn error_response(&self) -> HttpResponse {
        self.to_resp()
    }
}

pub enum ApiRt<L, R> {
    Ref(L),
    T(R)
}

impl <T,R> Responder for ApiRt<R, ResponseEntity<T>>
where
  T: Serialize,
  R: AsRef<ResponseEntity<T>>,
{
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;
    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        match self {
            ApiRt::Ref(a) => a.as_ref().respond_to(req),
            ApiRt::T(b) => b.respond_to(req),
        }
    }
}

impl <T: Serialize> Responder for ResponseEntity<T> {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;
    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        (&self).respond_to(req)
    }
}

impl <T: Serialize> Responder for &ResponseEntity<T> {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        ready(Ok(self.log_to_resp(req)))
    }
}