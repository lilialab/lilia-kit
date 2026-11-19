//! 二进制入口：转调 lib。

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    lilia_kit_lib::run()
}
