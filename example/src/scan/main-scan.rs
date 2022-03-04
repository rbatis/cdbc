use std::fs::File;
use fast_log::config::Config;
use log::Level;
use cdbc::{Either, Executor, query};
use cdbc::crud::{CRUD, Table};
use cdbc::database::Database;
use cdbc_sqlite::{Sqlite, SqlitePool};
use cdbc::Scan;
use cdbc::scan::Scan;

/// or use this example
/// #[derive(Debug,cdbc::ScanSqlite,cdbc::ScanMssql,cdbc::ScanMysql,cdbc::ScanPg)]
#[derive(Debug, Clone, cdbc::Scan)]
pub struct BizActivity {
    pub id: Option<String>,
    pub name: Option<String>,
    pub age: Option<i32>,
    pub delete_flag: Option<i32>,
}


impl Table for BizActivity {
    fn table() -> &'static str {
        "biz_activity"
    }

    fn columns() -> &'static [&'static str] {
        &["id", "name", "age", "delete_flag"]
    }
}

impl CRUD<BizActivity> for SqlitePool {
    fn inserts(&mut self, arg: Vec<BizActivity>) -> cdbc::Result<u64> where BizActivity: Sized {
        if arg.len() == 0 {
            return Ok(0);
        }
        let mut arg_idx = 1;
        let mut sql = format!("insert into {} ({}) values ", BizActivity::table(), BizActivity::columns_str());
        let mut value_num = 0;
        for x in &arg {
            if value_num != 0 {
                sql.push_str(",");
            }
            sql.push_str("(");
            sql.push_str(&BizActivity::values_str("?", &mut arg_idx));
            sql.push_str(")");
            value_num += 1;
        }
        log::info!("sql=> {}",sql);
        let mut q = query(sql.as_str());
        for arg in arg {
            log::info!("arg=> {:?},{:?},{:?},{:?}",arg.id,arg.name,arg.age,arg.delete_flag);
            q = q.bind(arg.id)
                .bind(arg.name)
                .bind(arg.age)
                .bind(arg.delete_flag);
        }
        self.execute(q).map(|r| {
            r.rows_affected()
        })
    }

    fn updates(&mut self, args: Vec<BizActivity>, r#where: &str) -> cdbc::Result<u64> where BizActivity: Sized {
        let mut num = 0;
        for arg in args {
            let mut q = query("");
            let mut arg_idx = 1;
            let mut sets = String::new();

            if arg.id.is_some() {
                sets.push_str("id = ");
                sets.push_str(&BizActivity::p("?", &mut arg_idx));
                sets.push_str(",");
                q = q.bind(arg.id);
            }
            if arg.name.is_some() {
                sets.push_str("name = ");
                sets.push_str(&BizActivity::p("?", &mut arg_idx));
                sets.push_str(",");
                q = q.bind(arg.name);
            }
            if arg.age.is_some() {
                sets.push_str("age = ");
                sets.push_str(&BizActivity::p("?", &mut arg_idx));
                sets.push_str(",");
                q = q.bind(arg.age);
            }
            if arg.delete_flag.is_some() {
                sets.push_str("delete_flag = ");
                sets.push_str(&BizActivity::p("?", &mut arg_idx));
                sets.push_str(",");
                q = q.bind(arg.delete_flag);
            }
            if sets.ends_with(",") {
                sets.pop();
            }
            let mut w = r#where.to_string();
            if !w.trim().is_empty() {
                w.insert_str(0, "where ");
            }
            let mut sql = format!("update {} set {} {}", BizActivity::table(), sets, w);
            log::info!("sql=> {}",sql);
            q.statement = Either::Left(&sql);
            self.execute(q).map(|r| {
                num += r.rows_affected();
            })?;
        }
        return Ok(num);
    }

    fn find(&mut self, r#where: &str) -> cdbc::Result<BizActivity> where BizActivity: Sized {
        let mut w = r#where.to_string();
        if !w.trim().is_empty() {
            w.insert_str(0, "where ");
        }
        let mut sql = format!("select * from {} {} ", BizActivity::table(), w);
        let q = query(&sql);
        self.fetch_one(q)?.scan()
    }

    fn finds(&mut self, r#where: &str) -> cdbc::Result<Vec<BizActivity>> where BizActivity: Sized {
        let mut w = r#where.to_string();
        if !w.trim().is_empty() {
            w.insert_str(0, "where ");
        }
        let mut sql = format!("select * from {} {} ", BizActivity::table(), w);
        let q = query(&sql);
        self.fetch_all(q)?.scan()
    }

    fn delete(&mut self, r#where: &str) -> cdbc::Result<u64> where {
        let mut w = r#where.to_string();
        if !w.trim().is_empty() {
            w.insert_str(0, "where ");
        }
        let mut sql = format!("delete from {} {} ", BizActivity::table(), w);
        let q = query(&sql);
        self.execute(q).map(|r| {
            r.rows_affected()
        })
    }
}

fn main() -> cdbc::Result<()> {
    fast_log::init(Config::new().console().level(Level::Trace));
    let pool = make_sqlite()?;

    let arg = BizActivity {
        id: Some("2".to_string()),
        name: Some("2".to_string()),
        age: Some(2),
        delete_flag: Some(1),
    };
    // BizActivity::insert(&pool,arg).unwrap();
    let r = pool.clone().insert(arg.clone());
    println!("insert = {:?}", r);

    let r = pool.clone().update(arg.clone(), "id = 1");
    println!("insert = {:?}", r);

    let data = query!("select * from biz_activity limit 1")
        .fetch_one(pool.clone())
        .scan();
    println!("{:?}", data);

    let data = query!("select * from biz_activity")
        .fetch_all(pool.clone())
        .scan();
    println!("{:?}", data);
    Ok(())
}

fn make_sqlite() -> cdbc::Result<SqlitePool> {
    //first. create sqlite dir/file
    std::fs::create_dir_all("target/db/");
    File::create("target/db/sqlite.db");
    //next create table and query result
    let pool = SqlitePool::connect("sqlite://target/db/sqlite.db")?;
    let mut conn = pool.acquire()?;
    conn.execute("CREATE TABLE biz_activity(  id string, name string,age int, delete_flag int) ");
    conn.execute("INSERT INTO biz_activity (id,name,age,delete_flag) values (\"1\",\"1\",1,0)");
    Ok(pool)
}
