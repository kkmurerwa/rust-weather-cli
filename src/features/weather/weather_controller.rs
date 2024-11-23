use crate::features::weather::weather_service;

pub fn get_weather_for(city: String) {
    match weather_service::get_weather(city) {
        Ok(weather_result) => println!("{:?}", weather_result),
        Err(error_msg) => {
            println!("{}", error_msg);
        }
    }
}