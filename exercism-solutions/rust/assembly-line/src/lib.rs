// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const BASE_SPEED: f64 = 221.0;
const LEVEL_0_SUCCESS_RATE: f64 = 0.0; // If speed is 0, success rate is 0, I guess?
const LEVEL_1_SUCCESS_RATE: f64 = 1.0;
const LEVEL_2_SUCCESS_RATE: f64 = 0.9;
const LEVEL_3_SUCCESS_RATE: f64 = 0.77;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let mut production_rate = BASE_SPEED * speed as f64;

    let success_rate = match speed {
        0 => LEVEL_0_SUCCESS_RATE, // This added as a separate match in order to explicitly specify success_rate for zero speed
        1..=4 => LEVEL_1_SUCCESS_RATE,
        5..=8 => LEVEL_2_SUCCESS_RATE,
        9..=10 => LEVEL_3_SUCCESS_RATE,
        _ => panic!("Incorrect speed supplied: {}!", speed),
    };

    production_rate * success_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
