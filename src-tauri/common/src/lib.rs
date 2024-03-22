use serde::{Deserialize, Serialize};

pub trait SqlExecute {
    fn sql_query(sql: &str) -> Response<std::string::String>;
}

#[derive(Debug, thiserror::Error)]
pub enum PalmError {
    #[error("读取字符串失败")]
    ReadError(#[from] std::io::Error),

    #[error("数据库连接失败")]
    MySqlConnectError(#[from] mysql::Error),

    #[error("json解析失败")]
    JsonParseError(#[from] serde_json::Error),

    // "{\"code\":".to_owned() + &self.code.to_string() + ",\"msg\":\"" + &self.msg + "\"}"
    #[error("{0}")]
    FailedWithMsg(String),
}

// 封装统一请求头
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request<T> {
    pub page_index: u16,
    pub page_size: u16,
    pub params: T,
}

// 统一返回枚举类
#[allow(dead_code)]
pub enum Resp<T, E> {
    Data(T),
    Err(E),
}

// 数据统一返回封装
#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {
    pub code: u8,
    pub msg: String,
    pub data: T,
}

// 返回数据结果处理
impl<T> Response<T> {
    pub fn new(code: u8, msg: String, data: T) -> Self {
        Response {
            msg: msg,
            code: code,
            data,
        }
    }

    // 实现转为json string
    #[allow(dead_code)]
    pub fn to_json_string(&self) -> String {
        let s =
            "{\"code\":".to_owned() + &self.code.to_string() + ",\"msg\":\"" + &self.msg + "\"}";
        format!("{}", s)
    }
}
