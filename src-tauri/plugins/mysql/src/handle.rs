use super::common::{Response, SqlExecute};
use common::PalmError;
use mysql::{
    params,
    prelude::{BinQuery, Queryable, WithParams},
    Pool, PooledConn,
};
use once_cell::sync::OnceCell;
use serde::Serialize;
use tracing::{info, instrument};

static DB_POOL: OnceCell<Pool> = OnceCell::new();

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct Account {
    pub id: String,
    pub account: String,
    pub password: String,
    pub enabled: i32,
    pub create_time: String,
    pub modify_time: String,
}

// 初始化数据库链接池
#[instrument]
pub fn init_mysql_pool(db_url: &str) {
    // mysql://root:password@localhost:3306/MYDB
    info!("初始化数据库线程池--------开始-------");
    DB_POOL
        .set(mysql::Pool::new(db_url).expect(&format!("Error connecting to {}", &db_url)))
        .unwrap_or_else(|_| info!("try insert pool cell failure!"));
    info!("初始化数据库线程池--------结束-------");
}

// 测试连接数据库
#[instrument]
pub fn test_link(db_url: &str) -> Result<i16, PalmError> {
    // mysql://root:password@localhost:3306/MYDB
    let link = mysql::Conn::new(db_url);
    match link {
        Ok(_) => Ok(200),
        Err(_) => Err(PalmError::FailedWithMsg("连接失败".to_string())),
    }
}

// 从链接链接池里面获取链接
#[instrument]
pub fn get_connect() -> PooledConn {
    info!("从链接池获取数据库链接----------开始----------");
    let conn = DB_POOL
        .get()
        .expect("Error get pool from OneCell<Pool>")
        .get_conn()
        .expect("Error get_connect from db pool");
    info!("从链接池获取数据库链接----------结束----------");
    conn
}

// 插入数据
pub fn insert(account: &str, password: &str) -> Result<u64, PalmError> {
    let mut conn = get_connect();
    // 生成主键id 目前id写死，后续会修改
    let id = 1;
    let x = match "insert into account (id,account,password,enabled,create_time,modify_time) values (?,?,?,1,now(),now())"
          .with((id, account, password))
          .run(&mut conn) {
         Ok(res) => {
           Ok(res.affected_rows())
          }
         Err(_) => {
           Err(PalmError::FailedWithMsg("创建失败".to_string()))
          }
        };
    x
}

pub fn get_by_id(id: &str) -> Option<Account> {
    let mut conn = get_connect();
    let query_result = conn
        .exec_first(
            "select id,account,password,enabled,create_time,modify_time from account where id=:id",
            params!("id"=>id),
        )
        .map(|row| {
            row.map(
                |(id, account, password, enabled, create_time, modify_time)| Account {
                    id,
                    account,
                    password,
                    enabled,
                    create_time,
                    modify_time,
                },
            )
        });
    match query_result {
        Ok(result) => result,
        Err(_) => None,
    }
}

// 数据更新
pub fn update(account: Account) -> Result<u64, PalmError> {
    let mut conn = get_connect();
    let x = match "UPDATE account SET account=?, password=?,enabled=?, modify_time=now() where id=?"
        .with((
            &account.account,
            &account.password,
            &account.enabled,
            &account.id,
        ))
        .run(&mut conn)
    {
        Ok(res) => Ok(res.affected_rows()),
        Err(e) => Err(PalmError::FailedWithMsg("用户信息更新失败".to_string())),
    };
    x
}

// 更具id删除
pub fn delete_by_id(id: &str) -> Result<u64, PalmError> {
    let mut conn = get_connect();
    let x = match "DELETE FROM account WHERE id=?".with((id,)).run(&mut conn) {
        Ok(res) => Ok(res.affected_rows()),
        Err(e) => Err(PalmError::FailedWithMsg("删除失败".to_string())),
    };
    x
}

pub struct Qeury {}

impl SqlExecute for Qeury {
    fn sql_query(sql: &str) -> Response<std::string::String> {
        Response::new(1, "hello".to_string(), sql.to_string())
    }
}
