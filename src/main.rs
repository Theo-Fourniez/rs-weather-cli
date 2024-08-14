//! An app that gets the daily forecast for 10 Belgian cities
//! Uses this API : https://www.prevision-meteo.ch/
//! Documentation of the API : https://www.prevision-meteo.ch/uploads/pdf/recuperation-donnees-meteo.pdf
use reqwest::{Client, Response, StatusCode};
use weather_types::WeatherData;
mod weather_types;

#[tokio::main]
async fn main() -> () {
    let client: Client = Client::new();

    let cities = vec![
        "bruxelles-1",
        "aalbeke",
        "gent",
        "beaumont",
        "antwerpen-1",
        "brugge",
        "grivegnee-liege",
        "hasselt",
        "mons",
        "namur",
    ];

    println!(
        "Getting the weather forecast for the Belgian cities : {} !",
        cities.join(" ")
    );
    for city in cities {
        print_daily_weather_forecast(&client, city).await; // Could have used reqwest blocking client
    }
}

/// Prints the daily forecast of a city
async fn print_daily_weather_forecast(client: &Client, city: &str) -> () {
    let url = format!("https://www.prevision-meteo.ch/services/json/{}", city);
    let response: Response = client.get(&url).send().await.unwrap();
    assert!(
        &response.status() == &StatusCode::OK,
        "Response from weather API was not OK, was {}",
        &response.status()
    );
    let weather_data: WeatherData = response.json().await.unwrap();
    println!(
        "Weather in {} will be {:?} tomorrow",
        weather_data.city_info.name, weather_data.fcst_day_1.condition
    )
}
