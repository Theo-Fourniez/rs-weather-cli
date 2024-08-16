use std::{
    fs::{self, File, OpenOptions},
    io::Write,
};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
struct FavoriteCity {
    city: String,
}

pub fn get_favorite_city() -> String {
    if let Some(proj_dirs) = ProjectDirs::from("com", "Decathlon", "Weather App") {
        let config_dir = proj_dirs.config_dir();
        fs::create_dir_all(&config_dir).expect("Failed to create config directory");
        let file = OpenOptions::new()
            .read(true) // Open the file for reading
            .write(true) // Open the file for writing
            .create(true) // Create the file if it doesn't exist
            .open(&config_dir.join("favorite.json"))
            .expect("Could not open the favorite configuration file");
        let favorite: FavoriteCity =
            serde_json::from_reader(file).unwrap_or(FavoriteCity::default());
        favorite.city
    } else {
        panic!("Could not get a config directory for this app")
    }
}

pub fn set_favorite_city(city_name: String) {
    if let Some(proj_dirs) = ProjectDirs::from("com", "Decathlon", "Weather App") {
        let config_dir = proj_dirs.config_dir();
        fs::create_dir_all(&config_dir).expect("Failed to create config directory");
        let mut file = File::create(&config_dir.join("favorite.json"))
            .expect("Could not open the favorite configuration file");
        let favorite = FavoriteCity { city: city_name };
        let to_write = serde_json::to_vec_pretty(&favorite).unwrap();
        file.write_all(&to_write)
            .expect("Could not write the favorite city to the config file");
    } else {
        panic!("Could not get a config directory for this app")
    }
}

mod utils {
    use std::fs;

    use directories::ProjectDirs;

    pub(crate) fn delete_favorite_config_file_and_dir() -> () {
        if let Some(proj_dirs) = ProjectDirs::from("com", "Decathlon", "Weather App") {
            let config_dir = proj_dirs.config_dir();
            let _ = fs::remove_dir_all(config_dir);
        }
    }

    pub(crate) fn create_favorite_config_folder() -> () {
        if let Some(proj_dirs) = ProjectDirs::from("com", "Decathlon", "Weather App") {
            let config_dir = proj_dirs.config_dir();
            fs::create_dir_all(&config_dir).expect("Failed to create config directory");
        }
    }
}
#[cfg(test)]
mod test {
    use crate::favorite_city::{
        get_favorite_city, set_favorite_city, utils::delete_favorite_config_file_and_dir,
    };

    #[test]
    pub fn test_setting_getting_favorite_city() {
        delete_favorite_config_file_and_dir();
        set_favorite_city("Gent".to_string());
        assert!(
            get_favorite_city() == "Gent",
            "The set favorite city name did not match the expected name"
        )
    }

    #[test]
    pub fn test_getting_default_favorite_city() {
        delete_favorite_config_file_and_dir();
        assert!(
            get_favorite_city() == "",
            "Expected an empty default city name"
        )
    }
}
