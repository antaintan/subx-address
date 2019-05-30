use chrono::Utc;
use cron::Schedule;
use std::collections::BTreeMap;
use std::str::FromStr;

fn main() {
    let mut map = BTreeMap::new();
    map.insert("TUSD", 100);
    map.insert("DOT", 500);
    map.insert("BTC", 100);

    println!("hello world!");
    println!("map: {:?}", map);

    // run job
    let expression = "*/3 * * * * *";
    let schedule = Schedule::from_str(expression).unwrap();
    println!("Upcoming fire times:");
    for _datetime in schedule.upcoming(Utc).take(10) {
        do_job();
    }
}

fn do_job() {
    println!("{:?}: It's time!", std::time::SystemTime::now());
}