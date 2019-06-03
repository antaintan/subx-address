#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use chrono::Utc;
use cron::Schedule;

use std::collections::BTreeMap;
use std::str::FromStr;
use std::time::SystemTime;

use substrate_keystore::Store;

use substrate_primitives::crypto::Pair as CPair;
use substrate_primitives::crypto::Ss58Codec;
use substrate_primitives::ed25519::Pair;
use tempdir::TempDir;

fn main() {
    let mut map = BTreeMap::new();
    map.insert("TUSD", 100);
    map.insert("DOT", 500);
    map.insert("BTC", 100);

    println!("hello world!");
    println!("map: {:?}", map);

    // run job
    let expression = "0 30 9,12,15 1,15 May-Aug Mon, Wed, Fri 2018/2";
    let schedule = Schedule::from_str(expression).unwrap();
    println!("Upcoming fire times:");
    for _datetime in schedule.upcoming(Utc).take(10) {
        do_job();
    }

    let temp_dir = TempDir::new("keystore").unwrap();
    let store = Store::open(temp_dir.path().to_owned()).unwrap();
    let key = store.generate("123123").unwrap();

    // launch web
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/address/create", routes![create])
        .launch();
}

fn do_job() {
    println!("{:?}: It's time!", SystemTime::now());
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!, rust web"
}

#[get("/address/create")]
fn create(pwd: &str) -> JsonValue {
    let (pair, phrase) = Pair::generate_with_phrase(Some(pwd));

    // 生成公钥, 私钥(seed)
    println!("public_key = {:?}", pair.public());

    let seed = hex::encode(pair.seed());
    println!("seed = 0x{}", seed);

    // 生成地址
    let address = pair.public().to_ss58check().to_string();
    println!("address, {:?}", address);

    // 生成助记词
    println!("mnemonic = {:?}", phrase);

    json!({ "code": 100, "seed": seed, "address": address, "mnemonic": phrase })
}