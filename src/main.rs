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
        Commands::Get { city, day } => match city {
            cli::CityNameOrFavorite::CityName(city) => {
                print_daily_weather_forecast(&client, &city, day.into()).await
            }
            cli::CityNameOrFavorite::Favorite => {
                println!("Getting the favorite city")
            }
        },
        Commands::Set { city_name } => {
            println!("Setting the favorite city as {}", city_name)
        }
    };
}

/// Prints the daily forecast of a city
async fn print_daily_weather_forecast(client: &Client, city: &str, day: usize) -> () {
    let url = format!("https://www.prevision-meteo.ch/services/json/{}", city);
    let response: Response = client.get(&url).send().await.unwrap();
    assert!(
        &response.status() == &StatusCode::OK,
        "Response from weather API was not OK, was {}",
        &response.status()
    );
    let weather_data: WeatherData = response.json().await.unwrap();
    let forecasts = [
        weather_data.fcst_day_0,
        weather_data.fcst_day_1,
        weather_data.fcst_day_2,
        weather_data.fcst_day_3,
        weather_data.fcst_day_4,
    ];

    let forecast = forecasts.get(day).expect("Forecast day out of range O..5");

    println!(
        "Weather in {} will be {:?} tomorrow",
        weather_data.city_info.name, forecast.condition
    )
}
