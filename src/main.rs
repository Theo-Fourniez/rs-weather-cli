//! An app that gets the daily forecast for 10 Belgian cities
//! Uses this API : https://www.prevision-meteo.ch/
//! Documentation of the API : https://www.prevision-meteo.ch/uploads/pdf/recuperation-donnees-meteo.pdf
use clap::{arg, Parser, Subcommand};
use reqwest::{Client, Response, StatusCode};
use weather_types::WeatherData;
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

fn city_in_list_or_favorite(s: &str) -> Result<String, String> {
    if s == "favorite" {
        return Ok(String::from(s));
    }

    CITIES
        .iter()
        .find(|x| **x == s)
        .map(|&x| String::from(x))
        .ok_or_else(|| {
            format!(
                "City {} not in supported cities. Supported cities are : {} or favorite",
                s,
                CITIES.join(" ")
            )
        })
}
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
    #[command(about = "Gets the weather forecast of a city")]
    Get {
        #[arg(
            help = "The city to get the weather from",
            required = false,
            default_value = "favorite",
            value_parser = city_in_list_or_favorite
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
    Set {
        #[arg(help = "The name of your favorite city")]
        city_name: String,
    },
}

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
