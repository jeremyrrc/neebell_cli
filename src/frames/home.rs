use crate::util::{proccess_input, stdin_line, ureq_result_to_string};
use crate::{ActionOutput, Frame, HOST};
use ureq::Agent;

fn get_password() -> String {
    println!("Password:");
    let password = stdin_line();
    println!("Confirm password:");
    let password2 = stdin_line();
    if password != password2 {
        println!("Passwords did not match.");
        get_password();
    }
    password
}

fn create_user(agent: &Agent) -> ActionOutput {
    println!("Name:");
    let name = stdin_line();
    let password = get_password();
    let url = format!("{}{}", HOST, "user/create");
    let result = agent
        .post(&url)
        .send_form(&[("name", &name), ("password", &password)]);
    match ureq_result_to_string(result) {
        Ok(s) => ActionOutput::response(s),
        Err((code, s)) => match code {
            500 | 401 => ActionOutput::new(s, Frame::Home),
            _ => ActionOutput::response(s),
        },
    }
}

fn sign_in(agent: &Agent) -> ActionOutput {
    println!("Name:");
    let name = stdin_line();
    println!("Password:");
    let password = stdin_line();

    let url = format!("{}{}", HOST, "user/sign-in");
    let result = agent
        .post(&url)
        .send_form(&[("name", &name), ("password", &password)]);
    match ureq_result_to_string(result) {
        Ok(s) => ActionOutput::new(s, Frame::Forums),
        Err((code, s)) => match code {
            500 | 401 => ActionOutput::new(s, Frame::Home),
            _ => ActionOutput::response(s),
        },
    }
}

pub fn run(agent: &Agent) -> ActionOutput {
    println!("Home:\n(1) 'create user'\n(2) 'sign in'");
    let input: u32 = match proccess_input() {
        Ok(n) => n,
        Err(a) => return a,
    };
    match input {
        1 => create_user(agent),
        2 => sign_in(agent),
        _ => ActionOutput::response(format!("Action {} not available.", input)),
    }
}
