#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::str::FromStr;
use std::error;
use std::fmt;

pub type Millis = u32;

#[derive(Debug)]
struct TimeUtilsError(String);

impl error::Error for TimeUtilsError {}

impl fmt::Display for TimeUtilsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

pub fn calcular_tiempo_en_millis_from_minuto_y_segundo(minutos: u32, segundos: u32) 
            -> Millis {
    minutos * 60_000 + segundos * 1000
}

pub fn calcular_tiempo_en_millis_from_minuto_y_segundo_str(minuto_segundo: &str) 
            -> Result<Millis, Box<error::Error>> {
    let v: Vec<&str> = minuto_segundo.split(':').collect();
    if v.len() != 2 {
        return Result::Err(Box::new(TimeUtilsError("Formato de hora no valido".into())))
    }

    let minutos: u32 = FromStr::from_str(v[0])?;
    let segundos: u32 = FromStr::from_str(v[1])?;
    Ok(minutos * 60_000 + segundos * 1000)
}

pub fn calcular_tiempo_en_millis_from_hora_minuto(hour_minute: &str) 
            -> Result<Millis, Box<error::Error>> {
    let v: Vec<&str> = hour_minute.split(':').collect();
    if v.len() != 2 {
        return Result::Err(Box::new(TimeUtilsError("Format  of hour not valid".into())))
    }

    let hour: u32 = FromStr::from_str(v[0])?;
    let minutos: u32 = FromStr::from_str(v[1])?;
    Ok((hour * 3600 + minutos * 60) * 1000)
}


pub fn millis_hora_minuto_to_string(millis: Millis) -> String {
    let count_minutes = millis / 60000;
    let hour             = count_minutes / 60;
    let minutes = count_minutes - (hour * 60);
    format!("{}:{:02}", hour, minutes)
}

pub fn convertir_tiempo_from_string_to_millis(input: &str) -> Option<Millis> {
    lazy_static! {
        static ref RE1: Regex = Regex::new(r"(\d+):(\d+):(\d{2})\.(\d{3})").unwrap();
        static ref RE2: Regex = Regex::new(r"(\d+):(\d{2})\.(\d{3})").unwrap();
        static ref RE3: Regex = Regex::new(r"(\d{2})\.(\d{3})").unwrap();
    }

    let millis: u32;
    let seconds: u32;
    let mut minutes: u32 = 0;
    let mut hours: u32 = 0;
    
    if let Some(caps) = RE1.captures(input) {
        millis = caps.get(4).unwrap().as_str().parse::<u32>().unwrap();
        seconds = caps.get(3).unwrap().as_str().parse::<u32>().unwrap();
        minutes = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
        hours = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();   
    } else if let Some(caps) = RE2.captures(input) {
            millis = caps.get(3).unwrap().as_str().parse::<u32>().unwrap();
            seconds = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
            minutes = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        } else if let Some(caps) = RE3.captures(input) {
                millis = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
                seconds = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
            } else {
                // Destructure failed. Change to the failure case.
               return None;
            };
    Some(millis + seconds * 1000 + minutes * 60000 + hours * 3_600_000)
}