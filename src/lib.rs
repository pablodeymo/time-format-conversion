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

pub fn calcular_tiempo_en_millis_from_minuto_y_segundo(minutes: u32, seconds: u32) -> Millis {
    minutes * 60_000 + seconds * 1000
}

pub fn calcular_tiempo_en_millis_from_minuto_y_segundo_str(minute_and_second: &str) 
            -> Result<Millis, Box<dyn error::Error>> {
    let v: Vec<&str> = minute_and_second.split(':').collect();
    if v.len() != 2 {
        return Result::Err(Box::new(TimeUtilsError("Invalid time format".into())))
    }

    let minutes: u32 = FromStr::from_str(v[0])?;
    let seconds: u32 = FromStr::from_str(v[1])?;
    Ok(minutes * 60_000 + seconds * 1000)
}

pub fn calcular_tiempo_en_millis_from_hora_minuto(hour_minute: &str) -> Result<Millis, Box<dyn error::Error>> {
    let v: Vec<&str> = hour_minute.split(':').collect();
    if v.len() != 2 {
        return Result::Err(Box::new(TimeUtilsError("Invalid time format".into())))
    }

    let hour: u32 = FromStr::from_str(v[0])?;
    let minutes: u32 = FromStr::from_str(v[1])?;
    Ok((hour * 3600 + minutes * 60) * 1000)
}

pub fn millis_to_string(millis: Millis) -> String {
    let mut count_minutes = millis / 60000;
    let mut count_seconds = millis / 1000;
    let count_millis = millis - (count_seconds * 1000);
    count_seconds %= 60;
    count_minutes %= 60;
    let count_hours = millis / 3_600_000;
    match count_hours {
        0 => format!("{}:{:02}.{:03}", count_minutes, count_seconds, count_millis),
        _ => format!("{}:{:02}:{:02}.{:03}", count_hours, count_minutes, count_seconds, count_millis),
    }

}

pub fn millis_hora_minuto_to_string(millis: Millis) -> String {
    let count_minutes = millis / 60000;
    let hour          = count_minutes / 60;
    let minutes = count_minutes - (hour * 60);
    format!("{}:{:02}", hour, minutes)
}

pub fn convertir_tiempo_from_string_to_millis(input: &str) -> Option<Millis> {
    lazy_static! {
        static ref RE1: Regex = Regex::new(r"(\d+):(\d+):(\d{2})\.(\d{3})").unwrap();
        static ref RE2: Regex = Regex::new(r"(\d+):(\d{2})\.(\d{3})").unwrap();
        static ref RE3: Regex = Regex::new(r"(\d{1,2})\.(\d{3})").unwrap();
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
