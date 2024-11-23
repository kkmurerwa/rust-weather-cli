mod features;

use std::{io, process};
use crate::features::weather::weather_controller;

pub fn run() {
    let city = get_user_city();
    weather_controller::get_weather_for(city);
}

fn get_user_city() -> String {
    println!("Please enter the full name of your city");
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap_or_else(|_| {
        println!("Invalid input entered. Please try again");
        process::exit(1);
    });
    input.trim().to_string()
}