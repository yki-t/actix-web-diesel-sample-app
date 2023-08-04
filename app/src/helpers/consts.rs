#![allow(unused)]

use std::{borrow::Cow, collections::HashMap, fs};

/// APIのCORSで許可されるドメイン
pub static DOMAINS: once_cell::sync::Lazy<Vec<String>> = once_cell::sync::Lazy::new(|| {
    dotenv::dotenv().ok();
    let domains = std::env::var("DOMAINS").expect("DOMAINS must be set");
    domains
        .split(',')
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
});

/// Diesel接続用のURL
pub static DATABASE_URL: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(|| {
    dotenv::dotenv().ok();
    let is_prod = match std::env::var("IS_PROD") {
        Ok(ok) => ok == "1",
        Err(_) => false,
    };
    if is_prod {
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set")
    } else {
        "postgres://username:password@db/database".to_string()
    }
});

/// セッション認証用のヘッダートークン
pub static AUTH_TOKEN_NAME: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(|| {
    dotenv::dotenv().ok();
    std::env::var("AUTH_TOKEN_NAME").expect("AUTH_TOKEN_NAME must be set")
});

/// セッション破棄時間
pub static SESSION_EXPIRE_SECS: once_cell::sync::Lazy<i64> = once_cell::sync::Lazy::new(|| {
    dotenv::dotenv().ok();
    std::env::var("SESSION_EXPIRE_SECS")
        .expect("SESSION_EXPIRE_SECS must be set")
        .parse()
        .unwrap()
});

/// タイムゾーン
pub static TIMEZONE: once_cell::sync::Lazy<chrono::FixedOffset> =
    once_cell::sync::Lazy::new(|| {
        dotenv::dotenv().ok();
        let hm_env = std::env::var("TIMEZONE").expect("TIMEZONE must be set");
        let hm = hm_env.split(':');
        let mut idx = 0;
        let offset = hm
            .map(|x| {
                let r = x.parse::<i32>().unwrap() * 3600 / 60_i32.pow(idx);
                idx += 1;
                r
            })
            .sum();
        chrono::FixedOffset::east_opt(offset).expect("TIMEZONE variable is invalid")
    });

/// ログ出力レベル
pub static LOG_LEVEL: once_cell::sync::Lazy<usize> = once_cell::sync::Lazy::new(|| {
    dotenv::dotenv().ok();
    let log_level = std::env::var("LOG_LEVEL")
        .unwrap_or("TRACE".to_string())
        .to_uppercase();
    match &*log_level {
        "TRACE" => 0,
        "DEBUG" => 1,
        "INFO" => 2,
        "WARN" => 3,
        "ERROR" => 4,
        "CRITICAL" => 5,
        _ => {
            panic!("LOG_LEVEL must be one of {{TRACE,DEBUG,INFO,WARN,ERROR,CRITICAL}}");
        }
    }
});
