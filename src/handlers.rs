use actix_web::{web, HttpResponse, Responder};
use std::sync::Mutex;
use rand::seq::SliceRandom;
use serde::{Serialize};

lazy_static::lazy_static! {
    static ref STATUSES: Mutex<Vec<String>> = Mutex::new(vec![
        "Feeling great!".to_string(),
        "Just another day...".to_string(),
        "Living the dream.".to_string(),
    ]);
}

// Define a struct for structured JSON responses
#[derive(Serialize)]
struct ApiResponse<T> {
    status: String,
    message: String,
    data: Option<T>,
}

// Handler to fetch a random Facebook status
pub async fn get_random_status() -> impl Responder {
    let statuses = STATUSES.lock().unwrap(); // Acquire lock on the statuses
    if let Some(random_status) = statuses.choose(&mut rand::thread_rng()) {
        let response = ApiResponse {
            status: "success".to_string(),
            message: "Random status fetched successfully.".to_string(),
            data: Some(random_status.clone()),
        };
        HttpResponse::Ok().json(response) // Return a structured JSON response
    } else {
        let response = ApiResponse::<String> {
            status: "error".to_string(),
            message: "No statuses available.".to_string(),
            data: None,
        };
        HttpResponse::Ok().json(response) // Return a structured JSON response
    }
}

// Handler to add a new Facebook status
pub async fn add_status(status: web::Json<String>) -> impl Responder {
    let mut statuses = STATUSES.lock().unwrap(); // Acquire lock on the statuses
    statuses.push(status.into_inner());

    let response = ApiResponse::<()>{ 
        status: "success".to_string(),
        message: "Status added successfully!".to_string(),
        data: None,
    };
    HttpResponse::Ok().json(response) // Return a structured JSON response
}

// Handler to fetch all Facebook statuses
pub async fn get_all_statuses() -> impl Responder {
    let statuses = STATUSES.lock().unwrap(); // Acquire lock on the statuses
    let all_statuses = statuses.join(", ");

    let response = ApiResponse {
        status: "success".to_string(),
        message: "All statuses fetched successfully.".to_string(),
        data: Some(all_statuses),
    };
    HttpResponse::Ok().json(response) // Return a structured JSON response
}
