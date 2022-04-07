use std::collections::HashMap;

use crate::models::{FlightRequest, FlightResponse};

fn search_path(mut map: HashMap<String, String>, start: &String) -> String {
    let mut item = start.clone();

    while let Some(elem) = map.remove(&item) {
            
            item = elem.clone();
    }
    item
}

pub fn find_start_end(request: FlightRequest) -> FlightResponse {

    let flight = request.flights;

    let m1: HashMap<String, String> = flight.iter().cloned().map(|flight| (flight.source, flight.destination)).collect();
    let m2: HashMap<String, String> = flight.iter().cloned().map(|flight| (flight.destination, flight.source)).collect(); // we reverse the pairs, to be able to go from end to start

    
    let left = flight[0].source.clone();
    let right = flight[0].destination.clone();


    let beg = search_path(m2, &left); // find the beginning of the path, starting somewhere in the middle, we just take the first element of the flight list

    let mut end = search_path(m1, &left); // find the end of the path, starting somewhere in the middle, we just take the first element of the flight list
    if end == left {
        end = right; // in case the first element in the list of flights was the last one
    }


    let res = (beg, end);
        
    FlightResponse {
        path: res
    }
}
