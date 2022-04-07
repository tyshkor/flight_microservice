use actix_web::{web::Json, HttpResponse, Responder};

use crate::{
    utils::find_start_end,    
    models::FlightRequest
};

pub async fn search_path(info: Json<FlightRequest>) -> impl Responder {

    let flight_request = info.into_inner();

    let res = find_start_end(flight_request);

    HttpResponse::Created().json(res)
}
