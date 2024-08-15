//! An app that gets the daily forecast for 10 Belgian cities
//! Uses this API : https://www.prevision-meteo.ch/
//! Documentation of the API : https://www.prevision-meteo.ch/uploads/pdf/recuperation-donnees-meteo.pdf
use clap::{arg, Parser, Subcommand};
use reqwest::{Client, Response, StatusCode};
use weather_types::WeatherData;
mod weather_types;

#[derive(Debug, Parser)]
#[command(
    name = "weather",
    version = "1.0",
    about = "A CLI to get the weather in Belgium"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Gets the weather forecast")]
    Get {
        #[arg(
            help = "The city to get the weather from",
            required = false,
            default_value = "favorite"
        )]
        city: String,
        #[arg(
            help = "The day of the forecast to get (0 is today, 4 is the limit)", 
            required = false,
            short,
            long,
            value_parser = clap::value_parser!(u8).range(0..4),
            default_value_t = 1
        )]
        day: u8,
    },
    #[command(about = "Sets your favorite city", arg_required_else_help(true))]
    Set { city_name: String },
}

const CITIES: &[&str] = &[
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
