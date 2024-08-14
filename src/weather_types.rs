use serde::Deserialize;

#[derive(Deserialize)]
pub struct CityInfo {
    pub name: String,
    country: String,
    latitude: String,
    longitude: String,
    elevation: String,
    sunrise: String,
    sunset: String,
}

#[derive(Deserialize)]
struct ForecastInfo {
    latitude: Option<f64>,
    longitude: Option<f64>,
    elevation: String,
}

#[derive(Deserialize)]
struct CurrentCondition {
    date: String,
    hour: String,
    tmp: i32,
    wnd_spd: i32,
    wnd_gust: i32,
    wnd_dir: String,
    pressure: f64,
    humidity: i32,
    condition: String,
    condition_key: String,
}

#[derive(Deserialize)]
pub struct FcstDay {
    date: String,
    day_short: String,
    day_long: String,
    tmin: i32,
    tmax: i32,
    pub condition: String,
    condition_key: String,
}

#[derive(Deserialize)]
pub struct WeatherData {
    pub city_info: CityInfo,
    pub forecast_info: ForecastInfo,
    pub current_condition: CurrentCondition,
    pub fcst_day_1: FcstDay,
}
