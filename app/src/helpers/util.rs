#![allow(dead_code)]

use chrono::{NaiveDateTime, Utc};

use std::borrow::Cow;

/// 現在時刻
pub fn now() -> NaiveDateTime {
    Utc::now()
        .with_timezone(&*crate::helpers::consts::TIMEZONE)
        .naive_local()
}

/// 現在時刻 文字列形式
pub fn ymdhms() -> Cow<'static, str> {
    now().format("%Y-%m-%dT%H:%M:%S").to_string().into()
}

/// 現在時刻 文字列形式
pub fn ymdhmsf() -> Cow<'static, str> {
    now().format("%Y-%m-%dT%H:%M:%S.%6f").to_string().into()
}

/// 現在時刻 Timestamp
pub fn timestamp() -> i64 {
    now().timestamp()
}
