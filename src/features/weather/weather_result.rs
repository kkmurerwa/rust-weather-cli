#[derive(Debug)]
pub struct WeatherResult {
    pub(crate) city_name: String,
    pub(crate) temperature: String,
    pub(crate) feels_like: String,
    pub(crate) humidity: String,
    pub(crate) wind_speed: String,
}