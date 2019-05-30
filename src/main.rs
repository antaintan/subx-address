use chrono::Utc;
use cron::Schedule;

use std::collections::BTreeMap;
use std::str::FromStr;
use std::time::SystemTime;

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

    // launch web
    rocket::ignite().mount("/", routes![index]).launch();
}

fn do_job() {
    println!("{:?}: It's time!", SystemTime::now());
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!, rust web"
}