use crate::frames::forums::ForumListItem;
use crate::util::{stdin_line, ureq_result_to_reader};
use crate::{ActionOutput, Frame, HOST};
use serde::{Deserialize, Serialize};
use serde_json;
use std::io::{BufRead, BufReader, Read};
use std::thread;
use ureq::Agent;

use super::home::User;

pub mod owned;
pub mod permitted;

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub user: String,
    pub forum_hex_id: String,
    pub value: String,
}

fn get_stream(
    agent: &Agent,
    item: &mut ForumListItem,
) -> Result<BufReader<Box<dyn Read + Send>>, ActionOutput> {
    let url = format!("{}{}{}", HOST, "forum/listen?f=", item._id.to_hex());
    let result = agent.get(&url).call();
    match ureq_result_to_reader(result) {
        Ok(r) => Ok(BufReader::new(r)),
        Err((code, s)) => match code {
            500 | 401 => Err(ActionOutput::new(s, Frame::Forums)),
            _ => Err(ActionOutput::response(s)),
        },
    }
}

fn listener<T: Read>(user_name: String, reader: BufReader<T>) {
    println!("Listening...");
    for line in reader.lines().flatten() {
        if let Some(data) = line.strip_prefix("data:") {
            let message: Message = match serde_json::from_str(data) {
                Ok(m) => m,
                Err(_) => continue,
            };
            if message.user != user_name {
                println!("{}: {}", message.user, message.value);
            }
        }
    }
}

pub fn listen(agent: &Agent, user: &User, item: &mut ForumListItem) -> Result<(), ActionOutput> {
    let reader = get_stream(agent, item)?;
    let name = user.get_name()?;
    listener(name, reader);
    Ok(())
}

pub fn message(agent: &Agent, user: &User, item: &mut ForumListItem) -> Result<(),ActionOutput> {
    let name = &user.get_name()?;

    println!("Ready to send messages...");
    loop {
        let message = stdin_line();
        if message == "exit" {
            return Err(ActionOutput::new("Exiting chat".to_string(), Frame::Forums));
        }
        let url = format!("{}{}", HOST, "forum/message");
        let result = agent.post(&url).send_form(&[
            ("forum_hex_id", &item._id.to_hex()),
            ("user", name),
            ("value", &message),
        ]);
        if result.is_err() {
            return Err(ActionOutput::new("Exiting chat".to_string(), Frame::Home));
        }
    }
}

pub fn listen_and_message(
    agent: &Agent,
    user: &User,
    item: &mut ForumListItem,
) -> Result<(), ActionOutput> {
    let reader = get_stream(agent, item)?;
    let name = user.get_name()?;
    thread::spawn(|| listener(name, reader));
    message(agent, user, item)?;
    Ok(())
}
