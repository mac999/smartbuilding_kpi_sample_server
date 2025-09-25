use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use chrono::{DateTime, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};

// Smart Building KPI data structure
#[derive(Serialize, Deserialize, Clone)]
pub struct SmartBuildingKPI {
    pub timestamp: DateTime<Utc>,     // Timestamp
    pub energy_efficiency: f64,       // Energy efficiency (%)
    pub temperature: f64,             // Indoor temperature (°C)
    pub humidity: f64,               // Indoor humidity (%)
    pub air_quality: f64,            // Air quality index (AQI)
    pub lighting_usage: f64,         // Lighting usage rate (%)
    pub occupancy_rate: f64,         // Space occupancy rate (%)
    pub water_consumption: f64,      // Water consumption (L)
    pub co2_level: f64,             // Carbon dioxide concentration (ppm)
}

// Dataset response structure
#[derive(Serialize)]
pub struct DatasetResponse {
    pub data: Vec<SmartBuildingKPI>,
}

// Random KPI data generation function
fn generate_random_kpi_data(count: usize) -> Vec<SmartBuildingKPI> {
    let mut rng = rand::thread_rng();
    let mut data = Vec::new();
    
    for i in 0..count {
        let kpi = SmartBuildingKPI {
            timestamp: Utc::now() - chrono::Duration::hours(count as i64 - i as i64),
            energy_efficiency: rng.gen_range(75.0..95.0),     // 75-95%
            temperature: rng.gen_range(20.0..28.0),           // 20-28°C
            humidity: rng.gen_range(40.0..60.0),              // 40-60%
            air_quality: rng.gen_range(20.0..100.0),          // 20-100 AQI
            lighting_usage: rng.gen_range(60.0..100.0),       // 60-100%
            occupancy_rate: rng.gen_range(30.0..90.0),        // 30-90%
            water_consumption: rng.gen_range(100.0..500.0),   // 100-500L
            co2_level: rng.gen_range(350.0..800.0),           // 350-800 ppm
        };
        data.push(kpi);
    }
    
    data
}

// API endpoint: return random data
async fn get_kpi_data() -> Result<HttpResponse> {
    let data = generate_random_kpi_data(10);
    let response = DatasetResponse { data };
    Ok(HttpResponse::Ok().json(response))
}

// Serve main page
async fn index() -> Result<HttpResponse> {
    let html = include_str!("../static/index.html");
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Smart Building KPI Dashboard Starting...");
    println!("Server running at http://localhost:8080");
    println!("Open your browser and visit the dashboard!");
    
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/api/kpi-data", web::get().to(get_kpi_data))
            .service(Files::new("/static", "./static").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
