use actix_web::{web, HttpResponse, Responder};
use std::sync::Mutex;
use rand::seq::SliceRandom;

lazy_static::lazy_static! {
    static ref STATUSES: Mutex<Vec<String>> = Mutex::new(vec![
        "Feeling great!".to_string(),
        "Just another day...".to_string(),
        "Living the dream.".to_string(),
    ]);
}

// Handler to fetch a random Facebook status
pub async fn get_random_status() -> impl Responder {
    let statuses = STATUSES.lock().unwrap(); // Acquire lock on the statuses
    if let Some(random_status) = statuses.choose(&mut rand::thread_rng()) {
        HttpResponse::Ok().body(random_status.clone())
    } else {
        HttpResponse::Ok().body("No statuses available.".to_string())
    }
}

// Handler to add a new Facebook status
pub async fn add_status(status: web::Json<String>) -> impl Responder {
    let mut statuses = STATUSES.lock().unwrap(); // Acquire lock on the statuses
    statuses.push(status.into_inner());
    HttpResponse::Ok().body("Status added successfully!")
}

// Handler to fetch all Facebook statuses
pub async fn get_all_statuses() -> impl Responder {
    let statuses = STATUSES.lock().unwrap(); // Acquire lock on the statuses
    let all_statuses = statuses.join(", ");
    HttpResponse::Ok().body(all_statuses)
}
