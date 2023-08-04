#![allow(dead_code)]
use anyhow::anyhow;

use diesel_async::{
    pooled_connection::deadpool::Pool,
    pooled_connection::AsyncDieselConnectionManager as ConnectionManager, AsyncPgConnection as DB,
    SimpleAsyncConnection,
};

use crate::{DbPool, Result};

pub fn establish_connection() -> DbPool {
    let config = ConnectionManager::<DB>::new(&*crate::helpers::consts::DATABASE_URL);
    Pool::builder(config)
        .build()
        .expect("DB Connection cannot be established")
}

pub async fn conn_test() -> Result<()> {
    let pool = establish_connection();
    crate::helpers::log::debug(file!(), line!(), "db connection establishing..".into());
    let conn = &mut pool.get().await.map_err(|_e| {
        println!("{:?}", _e);
        anyhow!("err 1")
    })?;
    crate::helpers::log::debug(file!(), line!(), "db connection checking..".into());
    conn.batch_execute("SELECT 1")
        .await
        .map_err(|e| anyhow!(e))?;
    crate::helpers::log::debug(file!(), line!(), "db connection ok".into());
    Ok(())
}
