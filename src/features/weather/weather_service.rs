use crate::features::weather::weather_result::WeatherResult;

struct WeatherService;

impl WeatherService {
    pub(crate) fn get_weather(city: &String) -> WeatherResult {
        WeatherResult::new(
            String::from("24.5"),
            String::from("24.5"),
            String::from("24.5"),
            String::from("24.5"),
        )
    }
}