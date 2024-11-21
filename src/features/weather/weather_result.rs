pub(crate) struct WeatherResult {
    temperature: String,
    feels_like: String,
    humidity: String,
    wind_speed: String,
}

impl WeatherResult {
    pub(crate) fn new(temperature: String, feels_like: String, humidity: String, wind_speed: String) -> WeatherResult {
        WeatherResult {
            temperature,
            feels_like,
            humidity,
            wind_speed,
        }
    }
}