use std::borrow::Cow;
use chrono::Local;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response<T = ()> {
    code: i32,
    msg: Option<Cow<'static, str>>,
    data: Option<T>,
    timestamp: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Page {
    pub page_num: i64,
    pub page_size: i64,
}

impl <T:Serialize> Response<T> {
    pub fn new(code: Option<i32>) -> Self {
        Self {
            code: code.unwrap_or(200),
            msg: None,
            data: None,
            timestamp: Local::now().timestamp_millis()
        }
    }

    pub fn with_code(mut self, code: i32) -> Self {
        self.code = code;
        self
    }

    pub fn with_msg<S: Into<Cow<'static, str>>>(mut self, msg: S) -> Self {
        self.msg = Some(msg.into());
        self
    }

    pub fn with_data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }
}