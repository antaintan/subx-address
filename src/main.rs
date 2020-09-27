#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

// 导出模块
pub mod web;

fn main() {
    // 初始化web
    web::init();
}

