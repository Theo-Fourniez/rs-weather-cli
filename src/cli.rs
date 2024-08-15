use clap::{command, Parser, Subcommand};

use crate::CITIES;

pub fn city_in_list_or_favorite(s: &str) -> Result<CityNameOrFavorite, String> {
    if s == "favorite" {
        return Ok(CityNameOrFavorite::Favorite);
    }

    return CITIES
        .iter()
        .find(|x| **x == s)
        .map(|&_x| CityNameOrFavorite::CityName(String::from(s)))
        .ok_or_else(|| {
            format!(
                "City {} not in supported cities. Supported cities are : {} or favorite",
                s,
                CITIES.join(" ")
            )
        });
}
#[derive(Debug, Parser)]
#[command(
    name = "weather",
    version = "1.0",
    about = "A CLI to get the weather in Belgium"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Clone)]
pub enum CityNameOrFavorite {
    CityName(String),
    Favorite,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "Gets the weather forecast of a city")]
    Get {
        #[arg(
            help = "The city to get the weather from",
            required = false,
            default_value = "favorite",
            value_parser = city_in_list_or_favorite
        )]
        city: CityNameOrFavorite,
        #[arg(
            help = "The day of the forecast to get (0 is today, 4 is the limit)", 
            required = false,
            short,
            long,
            value_parser = clap::value_parser!(u8).range(0..5),
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
