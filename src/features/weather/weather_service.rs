use crate::features::weather::something::APIResponse;
use crate::features::weather::weather_result::WeatherResult;
use reqwest;

#[tokio::main]
pub(crate) async fn get_weather<'a>(city: String) -> Result<WeatherResult, &'a str> {
    let client = reqwest::Client::new();
    let url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid=17ac7bb7aa6eae5c20f2be86852d9567", city);
    let response = client.get(url).send().await.unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            return match response.json::<APIResponse>().await {
                Ok(parsed) => {
                    let weather_result = convert_parsed_response_to_weather_result(parsed);
                    match weather_result {
                        Some(result) => Ok(result),
                        _ => Err("Unable to converted the parsed result to a WeatherResult")
                    }
                },
                Err(_) => Err("Unable to converted the parsed result to a WeatherResult")
            }
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            Err("Need to grab a new token")
        }
        _ => {
            println!("Error: {:?}", response);
            Err("Uh oh! Something unexpected happened")
        }
    }
}

fn convert_parsed_response_to_weather_result(parsed: APIResponse) -> Option<WeatherResult> {
    let main = parsed.main?;
    let temp = main.temp?;
    let feels_like = main.feels_like?;
    let humidity = main.humidity?;
    let wind_speed = parsed.wind?.speed?;
    let city_name = parsed.name?;

    Some(WeatherResult {
        temperature: format!("{:.2}", temp),
        feels_like: format!("{:.2}", feels_like),
        humidity: format!("{:?}", humidity),
        wind_speed: format!("{:.2}", wind_speed),
        city_name: format!("{}", city_name),
    })
}