/*
tokio-postgres = {version = "0.7", features = ["with-chrono-0_4","with-serde_json-1"]}
tokio = {version = "1", features = ["full"]}
futures = "0.3"
chrono = "0.4"
serde_json = "1.0"
hex = "0.4"
deadpool-postgres = "0.11"
*/

use deadpool_postgres::{Config, ManagerConfig, Pool, PoolConfig, RecyclingMethod, Runtime};
use futures::{TryStreamExt, future, pin_mut};
use std::collections::HashMap;
use tokio_postgres::{Client, Error, NoTls, SimpleQueryMessage, row::Row};

// 0.7.3'te bunsuz dene
// &[] ile
// https://github.com/sfackler/rust-postgres/pull/813
const NO_PARAMS: [i32; 0] = [];

async fn get_conn() -> Result<Client, Error> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost dbname=test user=test port=5433", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}

async fn get_pool() -> Pool {
    let mut cfg = Config::new();
    cfg.dbname = Some("test".to_string());
    cfg.pool = Some(PoolConfig::new(90));
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });
    cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut client = get_conn().await?;

    let n = client
        .execute(
            "INSERT INTO tbltest VALUES (DEFAULT,$1, $2)",
            &[&"TestAdi", &"TestSoyadi"],
        )
        .await?;
    println!("{} rows inserted", n);

    let rows = client.query("SELECT * FROM tbltest", &[]).await?;

    for row in rows {
        print_row(&row);
    }

    let n = client
        .execute(
            "UPDATE tbltest SET adi = $1, soyadi = $2 WHERE adi = $3",
            &[&"TestAdiX", &"TestSoyadiX", &"TestAdi"],
        )
        .await?;
    println!("{} rows updated", n);

    // query_opt
    if let Some(row) = client
        .query_opt("SELECT * FROM tbltest WHERE id = $1", &[&15])
        .await?
    {
        print_row(&row);
    }

    // prepare - execute
    let del_cmd = client.prepare("DELETE FROM tbltest WHERE id = $1").await?;
    let n = client.execute(&del_cmd, &[&13]).await?;
    println!("{} rows deleted", n);

    // query_opt - prepare
    let get_cmd = client
        .prepare("SELECT * FROM tbltest WHERE id = $1")
        .await?;
    if let Some(row) = client.query_opt(&get_cmd, &[&17]).await? {
        print_row(&row);
    }

    // query_one
    // ok() ile Result'u Option'a çevirdik
    // bunun yerine query_opt daha mantıklı
    if let Some(row) = client
        .query_one("SELECT * FROM tbltest WHERE id = $1", &[&5])
        .await
        .ok()
    {
        print_row(&row);
    } else {
        println!("No rows found");
    }

    // query_raw
    let it = client.query_raw("SELECT * FROM tbltest", NO_PARAMS).await?;
    //    let it = client.query_raw("SELECT * FROM tbltest", &[]).await?;
    pin_mut!(it);
    while let Some(row) = it.try_next().await? {
        print_row(&row);
    }

    // execute_raw
    let n = client
        .execute_raw("DELETE FROM tbltest WHERE id = $1", &[&18])
        .await?;
    println!("{} rows deleted", n);

    // simple_query
    let query = "CREATE TABLE tblxyz(id serial, xyz varchar(25));
	INSERT INTO tblxyz VALUES (DEFAULT,'sdfgsdfg');
	INSERT INTO tblxyz VALUES (DEFAULT,'sdfgsdfg');
	INSERT INTO tblxyz VALUES (DEFAULT,'sdfgsdfg');
	SELECT * FROM tblxyz;
	UPDATE tblxyz set xyz = 'KLMN' WHERE id = 2;
	DELETE FROM tblxyz;
	DROP TABLE tblxyz;";

    for mes in client.simple_query(query).await? {
        match mes {
            SimpleQueryMessage::Row(row) => {
                println!("{}-{}", row.get(0).unwrap(), row.get(1).unwrap())
            }
            SimpleQueryMessage::CommandComplete(n) => println!("{} rows affected", n),
            _ => (),
        }
    }

    // batch_execute
    // simple_query ile aynı fakat sonuç dönmüyor
    // CREATE vs işlemleri için uygun
    client.batch_execute(query).await?;

    // transaction - rollback - drop'ta rollback yapılıyor
    {
        let tx = client.transaction().await?;
        tx.execute("TRUNCATE TABLE tbltest", &[]).await?;
    }

    // transaction - commit
    let tx = client.transaction().await?;
    tx.execute(
        "INSERT INTO tbltest VALUES (DEFAULT,'sdfgsdfg','sdgsdgsdgd')",
        &[],
    )
    .await?;
    tx.commit().await?;

    let sql = include_str!("test.sql");
    client.batch_execute(sql).await?;

    for row in client.query("SELECT * from tblxxx", &[]).await? {
        let tbl = TblTest::from_row(&row);
        tbl.print();
    }

    // iki future'dan hangisi önce biterse
    let token = client.cancel_token();
    tokio::select! {
        _ = client.execute("SELECT pg_sleep(1);", &[]) => println!("sleep bitti"),
        _ = tokio::time::sleep(std::time::Duration::from_millis(1100)) => { // 1050'de cancel ediyor
            token.cancel_query(NoTls).await?;
            println!("Cancel edildi");
        }
    }

    // future spawn
    let handler = tokio::spawn(async move {
        get_conn()
            .await
            .unwrap()
            .execute("SELECT pg_sleep(1);", &[])
            .await
            .unwrap();
        println!("pg_sleep spawn bitti");
    });

    handler.await.unwrap();

    println!("Başladı");
    // deadpool

    let pool = get_pool().await;
    let mut v = vec![];
    for _ in 0..90 {
        let pool = pool.clone();
        let handle = tokio::spawn(async move {
            for _ in 0..1_000 {
                let client = pool.get().await.unwrap();
                let stmt = client
                    .prepare_cached("SELECT * from tbltest")
                    .await
                    .unwrap();

                let rows = client.query(&stmt, &[]).await.unwrap();
                for row in rows {
                    print_row(&row);
                }
            }
        });

        v.push(handle);
    }

    future::join_all(v).await;

    /* join_all ile aynı işi yapıyor
    for handle in v {
        handle.await.unwrap();
    }
    */

    println!("Bitti");
    // bu yanlış
    /*
    for _ in 0..1000 {
        let rows = client.query(&stmt, &[]).await.unwrap();
        for row in rows {
            print_row(&row);
        }
    }
    */

    Ok(())
}

struct TblTest {
    id: Option<i32>,
    a: Option<String>,
    b: Option<String>,
    c: Option<i32>,
    d: Option<i64>,
    e: Option<f32>,
    f: Option<f64>,
    g: Option<chrono::NaiveDateTime>,
    h: Option<chrono::DateTime<chrono::Local>>,
    i: Option<chrono::NaiveDate>,
    j: Option<chrono::NaiveTime>,
    k: Option<bool>,
    l: Option<String>,
    m: Option<i16>,
    n: Option<serde_json::Value>,
    o: Option<serde_json::Value>,
    p: Option<Vec<u8>>,
    r: Option<HashMap<String, Option<String>>>,
}

impl TblTest {
    fn from_row(row: &tokio_postgres::row::Row) -> TblTest {
        TblTest {
            id: row.get(0),
            a: row.get(1),
            b: row.get("b"),
            c: row.get(3),
            d: row.get("d"),
            e: row.get(5),
            f: row.get("f"),
            g: row.get(7),
            h: row.get("h"),
            i: row.get(9),
            j: row.get("j"),
            k: row.get(11),
            l: row.get("l"),
            m: row.get(13),
            n: row.get("n"),
            o: row.get(15),
            p: row.get("p"),
            r: row.get("r"),
        }
    }

    fn print(self) {
        let cd = chrono::NaiveDate::from_ymd_opt(2021, 7, 8).unwrap();
        let ct = chrono::NaiveTime::from_hms_opt(9, 10, 11).unwrap();
        let dt = chrono::NaiveDateTime::new(cd, ct);
        let cl = chrono::Local::now();

        println!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            self.id.unwrap(),
            self.a.unwrap_or("".into()),
            self.b.unwrap_or("".into()),
            self.c.unwrap_or(0),
            self.d.unwrap_or(0),
            self.e.unwrap_or(0.0),
            self.f.unwrap_or(0.0),
            self.g.unwrap_or(dt),
            self.h.unwrap_or(cl),
            self.i.unwrap_or(cd),
            self.j.unwrap_or(ct),
            self.k.unwrap_or(true),
            self.l.unwrap_or("".into()),
            self.m.unwrap_or(0),
            self.n.unwrap_or(serde_json::json!(null)),
            self.o.unwrap_or(serde_json::json!(null)),
            hex::encode(self.p.unwrap_or(Vec::new())),
            self.r
                .unwrap_or(HashMap::new())
                .get("c")
                .unwrap_or(&Some("X".to_string()))
                .clone()
                .unwrap(),
        );
    }
}

fn print_row(row: &Row) {
    let id: i32 = row.get(0);
    let adi: &str = row.get(1);
    let soyadi: &str = row.get(2);
    println!("id: {} - adi: {} - soyadi: {}", id, adi, soyadi);
}

/*
test.sql

DROP TABLE IF EXISTS tblxxx;

CREATE TABLE tblxxx (
    id serial primary key,
    a varchar(50),
    b text,
    c int,
    d bigint,
    e real,
    f double precision,
    g timestamp,
    h timestamptz,
    i date,
    j time,
    k bool,
    l char,
    m smallint,
    n json,
    o jsonb,
    p bytea,
    r hstore
);

INSERT INTO tblxxx VALUES(DEFAULT,'a','b',1,123456789,3.14, 3.15, '2021-08-04 14:53:28','2021-08-04 14:53:28-03','2021-08-04','14:53:28',true,'X',12,'{ "name": "KLM"}','{ "name": "KLM"}',decode('DEADBEEF','hex'), 'a=>b, c=>d');
INSERT INTO tblxxx VALUES(DEFAULT,'a','b',1,123456789,3.14, 3.15, NOW(), CURRENT_TIMESTAMP, '2021-08-04',CURRENT_TIME,false,'X',24,'{ "name": "XXX"}','{ "name": "KLM"}',decode('DEADBEEF','hex'), 'a=>b, c=>d');
INSERT INTO tblxxx VALUES(DEFAULT,'a','b',1,123456789,3.14, 3.15, CURRENT_TIMESTAMP, now(), CURRENT_DATE ,CURRENT_TIME,'t','X',25,'{ "name": "YYY"}','{ "name": "KLM"}',decode('DEADBEEF','hex'), 'a=>b, c=>d');
INSERT INTO tblxxx(id) VALUES(DEFAULT);

SELECT * FROM tblxxx;

 */
