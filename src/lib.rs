mod owm;
#[cfg(test)]
mod tests;
mod utils;

pub mod owm_structs {
    pub use crate::owm::geocoding::structures::*;
    pub use crate::owm::weather::structures::*;
}

pub mod owm_api {
    use crate::{owm, owm_structs};

    /// Gets the coordinates of a city by its name
    ///
    /// # Arguments
    /// * `city_name` - The city name to look for
    ///
    /// * `api_key` - Your OWM API key
    pub async fn get_city_coordinates(
        city_name: String,
        api_key: String,
    ) -> Result<owm_structs::Coordinates, reqwest::Error> {
        owm::geocoding::api::get_coordinates_by_location_name(city_name, api_key).await
    }

    /// Gets the weather data given input coordinates.
    ///
    /// # Arguments
    /// * `latitude` - The latitude of the target location
    ///
    /// * `longitude` - The longitude of the target location
    ///
    /// * `api_key` - Your API key
    pub async fn get_weather_by_coordinates(
        latitude: f32,
        longitude: f32,
        api_key: String,
    ) -> Result<owm_structs::WeatherData, reqwest::Error> {
        owm::weather::api::get_weather_for_coordinates(latitude, longitude, api_key).await
    }

    /// Combines searching for the coordinates of a city by name + Fetching the weather data for the location
    ///
    /// # Arguments
    /// * `city_name` - The city name to look for
    ///
    /// * `api_key` - Your OWM API key
    pub async fn get_weather_by_city(
        city_name: String,
        api_key: String,
    ) -> Result<owm_structs::WeatherData, reqwest::Error> {
        let coordinates: owm::geocoding::structures::Coordinates =
            owm::geocoding::api::get_coordinates_by_location_name(city_name, api_key.clone())
                .await?;
        owm::weather::api::get_weather_for_coordinates(
            coordinates.get_latitude(),
            coordinates.get_longitude(),
            api_key,
        )
        .await
    }

    pub mod blocking {
        use crate::owm_structs;
        use tokio::runtime::Runtime;

        fn get_runtime() -> Runtime {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Could not build tokio instance.")
        }

        /// Gets the coordinates of a city by its name
        ///
        /// # Arguments
        /// * `city_name` - The city name to look for
        ///
        /// * `api_key` - Your OWM API key
        pub fn get_city_coordinates(
            city_name: String,
            api_key: String,
        ) -> Result<owm_structs::Coordinates, reqwest::Error> {
            get_runtime().block_on(async move {
                crate::owm_api::get_city_coordinates(city_name, api_key).await
            })
        }

        /// Gets the weather data given input coordinates.
        ///
        /// # Arguments
        /// * `latitude` - The latitude of the target location
        ///
        /// * `longitude` - The longitude of the target location
        ///
        /// * `api_key` - Your API key
        pub fn get_weather_by_coordinates(
            latitude: f32,
            longitude: f32,
            api_key: String,
        ) -> Result<owm_structs::WeatherData, reqwest::Error> {
            get_runtime().block_on(async move {
                crate::owm_api::get_weather_by_coordinates(latitude, longitude, api_key).await
            })
        }

        /// Combines searching for the coordinates of a city by name + Fetching the weather data for the location
        ///
        /// # Arguments
        /// * `city_name` - The city name to look for
        ///
        /// * `api_key` - Your OWM API key
        pub fn get_weather_by_city(
            city_name: String,
            api_key: String,
        ) -> Result<owm_structs::WeatherData, reqwest::Error> {
            get_runtime().block_on(async move {
                crate::owm_api::get_weather_by_city(city_name, api_key).await
            })
        }
    }
}

pub mod owm_utils {
    pub mod convert {
        pub use crate::utils::convert::*;
    }
}

pub mod prelude {
    // Main
    pub use crate::owm_api::*;
    pub use crate::owm_structs::*;
    // Features
    #[cfg(feature = "utils")]
    pub use crate::owm_utils::*;
}
