//! An app that gets the daily forecast for 10 Belgian cities
//! Uses this API : https://www.prevision-meteo.ch/
//! Documentation of the API : https://www.prevision-meteo.ch/uploads/pdf/recuperation-donnees-meteo.pdf
use reqwest::{Client, Response, StatusCode};

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
    let response_json: serde_json::Value = response.json().await.unwrap();
    let forecast_tomorrow = &response_json
        .as_object()
        .unwrap()
        .get("fcst_day_1")
        .unwrap();

    let condition_tomorrow = forecast_tomorrow
        .get("condition_key")
        .unwrap()
        .as_str()
        .unwrap();
    println!(
        "Weather in {} will be {:?} tomorrow",
        city, condition_tomorrow
    )
}
