//! An app that gets the daily forecast for 10 Belgian cities
//! Uses this API : https://www.prevision-meteo.ch/
//! Documentation of the API : https://www.prevision-meteo.ch/uploads/pdf/recuperation-donnees-meteo.pdf
use clap::Parser;
use cli::{Cli, Commands};
use reqwest::{Client, Response, StatusCode};
use weather_types::WeatherData;

mod cli;
mod weather_types;

const CITIES: [&str; 10] = [
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

#[tokio::main]
async fn main() -> () {
    let client: Client = Client::new();

    let parsed = Cli::parse();
    println!("parsed : {:?}", parsed);
    match parsed.command {
        Commands::Get { city, day } => print_daily_weather_forecast(&client, &city).await,
        Commands::Set { city_name } => {
            println!("Setting the favorite city")
        }
    };
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
