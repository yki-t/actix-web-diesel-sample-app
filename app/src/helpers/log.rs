#![allow(unused)]

use crate::DbConn;
use std::borrow::Cow;

pub fn trace(file: &str, line: u32, message: Cow<'_, str>) {
    if *super::consts::LOG_LEVEL < 1 {
        println!(
            "[{}]\t<TRACE>\tAPI\t@{file}:{line}\t{message}",
            super::util::ymdhmsf()
        );
    }
}

pub fn debug(file: &str, line: u32, message: Cow<'_, str>) {
    if *super::consts::LOG_LEVEL < 2 {
        println!(
            "[{}]\t<DEBUG>\tAPI\t@{file}:{line}\t{message}",
            super::util::ymdhmsf()
        );
    }
}

pub fn info(file: &str, line: u32, message: Cow<'_, str>) {
    if *super::consts::LOG_LEVEL < 3 {
        println!(
            "[{}]\t<INFO>\tAPI\t@{file}:{line}\t{message}",
            super::util::ymdhmsf()
        );
    }
}

pub fn warn(file: &str, line: u32, message: Cow<'_, str>) {
    if *super::consts::LOG_LEVEL < 4 {
        println!(
            "[{}]\t<WARN>\tAPI\t@{file}:{line}\t{message}",
            super::util::ymdhmsf()
        );
    }
}

pub fn error(file: &str, line: u32, message: Cow<'_, str>) {
    if *super::consts::LOG_LEVEL < 5 {
        println!(
            "[{}]\t<ERROR>\tAPI\t@{file}:{line}\t{message}",
            super::util::ymdhmsf()
        );
    }
}

pub fn critical(file: &str, line: u32, message: Cow<'_, str>) {
    if *super::consts::LOG_LEVEL < 6 {
        println!(
            "[{}]\t<CRITICAL>\tAPI\t@{file}:{line}\t{message}",
            super::util::ymdhmsf()
        );
    }
}
