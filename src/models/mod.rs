use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FlightRequest {
    pub flights: Vec<Flight>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Flight {
    pub source: String,
    pub destination: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FlightResponse {
    pub path: (String, String),
}
