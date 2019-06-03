
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] 
extern crate rocket;

mod controller

#[get("/user")]
fn index() -> &'static str {
    "Hello, world!, rust web"
}

#[post("/user/<name>")]
fn hello(name: String) -> String {
    format!("{}, we need to talk about your coolness.", name)
}