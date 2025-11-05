use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct DogImage {
    message: String,
    status: String,
}

#[derive(Debug)]
enum DogFetcherError {
    NetworkError(String),
    ApiError(String),
    JsonError(String),
    FileError(String),
}

#[derive(Debug)]
enum ApiResult {
    Success(String), // file path of the saved image
    Failure(DogFetcherError),
}

fn fetch_random_dog_image(index: usize) -> ApiResult {
    let url = "https://dog.ceo/api/breeds/image/random";

    match ureq::get(url).call() {
        Ok(response) => {
            if response.status() == 200 {
                match response.into_json::<DogImage>() {
                    Ok(dog_image) => {
                        println!("✅ Got image URL: {}", dog_image.message);
                        download_image(&dog_image.message, index)
                    }
                    Err(e) => ApiResult::Failure(DogFetcherError::JsonError(format!(
                        "Failed to parse JSON: {}",
                        e
                    ))),
                }
            } else {
                ApiResult::Failure(DogFetcherError::ApiError(format!(
                    "HTTP error: {}",
                    response.status()
                )))
            }
        }
        Err(e) => ApiResult::Failure(DogFetcherError::NetworkError(format!(
            "Request failed: {}",
            e
        ))),
    }
}

fn download_image(url: &str, index: usize) -> ApiResult {
    match ureq::get(url).call() {
        Ok(response) => {
            if response.status() == 200 {
                let mut reader = response.into_reader();
                let filename = format!("dog_{}.jpg", index);
                let path = Path::new(&filename);

                match File::create(path) {
                    Ok(mut file) => {
                        if std::io::copy(&mut reader, &mut file).is_ok() {
                            ApiResult::Success(filename)
                        } else {
                            ApiResult::Failure(DogFetcherError::FileError(
                                "Failed to write image file".into(),
                            ))
                        }
                    }
                    Err(e) => ApiResult::Failure(DogFetcherError::FileError(format!(
                        "Failed to create file: {}",
                        e
                    ))),
                }
            } else {
                ApiResult::Failure(DogFetcherError::ApiError(format!(
                    "Image download HTTP error: {}",
                    response.status()
                )))
            }
        }
        Err(e) => ApiResult::Failure(DogFetcherError::NetworkError(format!(
            "Image request failed: {}",
            e
        ))),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Dog Image Fetcher");
    println!("=================\n");

    for i in 1..=5 {
        println!("Fetching random dog image #{}", i);
        match fetch_random_dog_image(i) {
            ApiResult::Success(filename) => {
                println!("✅ Image saved as: {}", filename);
            }
            ApiResult::Failure(e) => match e {
                DogFetcherError::NetworkError(msg) => println!("🌐 Network error: {}", msg),
                DogFetcherError::ApiError(msg) => println!("❌ API error: {}", msg),
                DogFetcherError::JsonError(msg) => println!("⚠️ JSON parse error: {}", msg),
                DogFetcherError::FileError(msg) => println!("💾 File error: {}", msg),
            },
        }
        println!();
    }

    Ok(())
}
