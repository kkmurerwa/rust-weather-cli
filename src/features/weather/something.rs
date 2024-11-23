use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Main {
	pub temp: Option<f64>,
	pub feels_like: Option<f64>,
	pub pressure: Option<i32>,
	pub humidity: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wind {
	pub speed: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct APIResponse {
	pub main: Option<Main>,
	pub visibility: Option<i32>,
	pub wind: Option<Wind>,
	pub name: Option<String>,
}

