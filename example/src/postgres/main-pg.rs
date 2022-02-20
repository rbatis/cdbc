use mco::std::time::time::Time;
use cdbc::Executor;
use cdbc_pg::PgPool;

fn main() -> cdbc::Result<()> {
    #[derive(Debug,serde::Serialize)]
    pub struct BizActivity {
        pub id: Option<String>,
        pub name: Option<String>,
        pub delete_flag: Option<i32>,
        pub create_time: Option<Time>
    }
    let pool = PgPool::connect("postgres://postgres:123456@localhost:5432/postgres")?;
    let data = cdbc::row_scans!(
        cdbc::query("select * from biz_activity limit 1")
        .fetch_all(pool)?,
        BizActivity{id:None,name:None,delete_flag:None,create_time:None})?;
    println!("{:?}", data);
    println!("{}", serde_json::to_string(&data).unwrap());
    Ok(())
}
