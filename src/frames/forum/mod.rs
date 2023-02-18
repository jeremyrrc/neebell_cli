use crate::frames::forums::ForumListItem;
use crate::util::{stdin_line, ureq_result_to_reader, ureq_result_to_string};
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
    let url = format!(
        "{}{}{}",
        HOST,
        "forum/listen-messages?f=",
        item._id.to_hex()
    );
    let result = agent
        .get(&url)
        .set("Origin", "http://127.0.0.1:5173")
        .call();
    match ureq_result_to_reader(result) {
        Ok(r) => Ok(BufReader::new(r)),
        Err((code, s)) => match code {
            401 => Err(ActionOutput::new(s, Frame::Home)),
            _ => Err(ActionOutput::response(s)),
        },
    }
}

fn listen<T: Read>(user_name: String, reader: BufReader<T>) {
    println!("Listening...");
    for line in reader.lines().flatten() {
        if let Some(_) = line.strip_prefix("event:closed") {
            println!("--Stream closed--");
            break;
        }
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

pub fn message(agent: &Agent, user: &User, item: &mut ForumListItem) -> Result<(), ActionOutput> {
    let name = &user.get_name()?;

    println!("Ready to send messages...");
    loop {
        let message = stdin_line();
        if message == "exit" {
            return Err(ActionOutput::new(
                "--Exiting chat--".to_string(),
                Frame::Forums,
            ));
        }
        let url = format!("{}{}", HOST, "forum/message");
        let result = agent
            .post(&url)
            .set("Origin", "http://127.0.0.1:5173")
            .send_form(&[
                ("forum_hex_id", &item._id.to_hex()),
                ("user", name),
                ("value", &message),
            ]);
        return match ureq_result_to_string(result) {
            Ok(_) => continue,
            Err((code, s)) => match code {
                401 => Err(ActionOutput::new(s, Frame::Home)),
                _ => Err(ActionOutput::response(s)),
            },
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
    thread::spawn(|| listen(name, reader));
    let r = message(agent, user, item);
    let url = format!(
        "{}{}{}",
        HOST,
        "forum/unsubscribe-messages?f=",
        item._id.to_hex()
    );
    let _ = agent
        .get(&url)
        .set("Origin", "http://127.0.0.1:5173")
        .call();
    r
}
