
mod controllers;

pub fn init() {
    // launch web
    rocket::ignite()
    .mount("/", routes![controllers::index])
    // 地址
    .mount("/", routes![controllers::new_address])
    .launch();
}

