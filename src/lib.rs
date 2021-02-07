use std::{thread, time};

use std::io::stdin;

use rand::thread_rng;
use rand::Rng;

use chrono::prelude::*;

pub fn sleep(millis: u64) {
    let duration = time::Duration::from_millis(millis);
    thread::sleep(duration);
}

pub fn input() -> String {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string)
    	.ok()
        .expect("Failed to read line");
        return input_string.trim().to_string();
}

pub fn random(number_1: i32, number_2: i32) -> i32 {
    let mut rng = thread_rng();
    let final: i32 = rng.gen_range(number_1..number_2);
    return final;
}

pub fn date() {
    let utc: DateTime<Utc> = Utc::now();
    return utc;
}
