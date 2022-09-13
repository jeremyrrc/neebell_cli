use crate::{ActionOutput, Frame};
use serde::de::DeserializeOwned;
use std::io::{self, Read};
use std::str::FromStr;
use ureq::{Error, Response};

pub fn stdin_line() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    buffer.trim().to_string()
}

pub fn proccess_input<T: FromStr>() -> Result<T, ActionOutput> {
    let input = stdin_line();
    match input.as_str() {
        "home" => Err(ActionOutput::redirect(Frame::Home)),
        "forums" => Err(ActionOutput::redirect(Frame::Forums)),
        other => match other.to_owned().parse::<T>() {
            Ok(n) => Ok(n),
            Err(_) => Err(ActionOutput::response(format!(
                "Action '{other}' not available"
            ))),
        },
    }
}

fn ureq_result_to_response(result: Result<Response, Error>) -> Result<Response, (u16, String)> {
    match result {
        Ok(r) => Ok(r),
        Err(Error::Status(code, r)) => {
            let response = r
                .into_string()
                .unwrap_or("The error response was too large.".to_string());
            Err((code, format!("{code}: {response}")))
        }
        Err(_) => Err((500, "Connection error.".to_string())),
    }
}

pub fn ureq_result_to_reader(
    result: Result<Response, Error>,
) -> Result<Box<dyn Read + Send + 'static>, (u16, String)> {
    Ok(ureq_result_to_response(result)?.into_reader())
}

pub fn ureq_result_to_json<T: DeserializeOwned>(
    result: Result<Response, Error>,
) -> Result<T, (u16, String)> {
    let r = ureq_result_to_response(result)?;
    let json: T = r.into_json().map_err(|e| (500, e.to_string()))?;
    Ok(json)
}

pub fn ureq_result_to_string(result: Result<Response, Error>) -> Result<String, (u16, String)> {
    let r = ureq_result_to_response(result)?;
    let string = r
        .into_string()
        .unwrap_or("The ok response was too large.".to_string());
    Ok(string)
}
