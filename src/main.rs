#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use chrono::Utc;
use cron::Schedule;

use std::collections::BTreeMap;
use std::str::FromStr;
use std::time::SystemTime;

//use substrate_keystore::Store;

use rocket::http::RawStr;
//use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::json::JsonValue;
use substrate_primitives::crypto::Pair as CPair;
use substrate_primitives::crypto::Ss58Codec;
use substrate_primitives::ed25519::Pair;
//use tempdir::TempDir;

use substrate_primitives::hexdisplay::HexDisplay;

fn main() {
    let mut map = BTreeMap::new();
    map.insert("TUSD", 100);
    map.insert("DOT", 500);
    map.insert("BTC", 100);

    println!("map: {:?}", map);

    // run job
    let expression = "0 30 9,12,15 1,15 May-Aug Mon, Wed, Fri 2018/2";
    let schedule = Schedule::from_str(expression).unwrap();
    println!("Upcoming fire times:");
    for _datetime in schedule.upcoming(Utc).take(10) {
        do_job();
    }

    // let temp_dir = TempDir::new("keystore").unwrap();
    // let store = Store::open(temp_dir.path().to_owned()).unwrap();
    // let key = store.generate("123123").unwrap();

    // launch web
    rocket::ignite()
        .mount("/", routes![index, new_address])
        .launch();
}

fn do_job() {
    println!("{:?}: It's time!", SystemTime::now());
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!, rust web"
}

#[post("/address/newAddress?<pwd>")]
fn new_address(pwd: &RawStr) -> JsonValue {
    // 生成kusama网络地址, address prefix不同
    let (pair, phrase, seed) = Pair::generate_with_phrase(Some(pwd.as_str()));

    json!({
        "code": 100,
        "mnemonic": phrase,
        "secret seed": format!("0x{}", HexDisplay::from(&seed.as_ref())),
        "public": format!("0x{}", HexDisplay::from(&pair.public().clone().as_ref())),
        "SS58 address": pair.public().to_ss58check().to_string()
    })
}
