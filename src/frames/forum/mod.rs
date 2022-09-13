use crate::frames::forums::ForumListItem;
use crate::util::{stdin_line, ureq_result_to_reader, ureq_result_to_string};
use crate::{ActionOutput, Frame, HOST};
use std::io::{copy, stdout};
use ureq::Agent;

pub mod owned;
pub mod permitted;

pub fn listen(agent: &Agent, item: &mut ForumListItem) -> ActionOutput {
    let url = format!("{}{}{}", HOST, "forum/listen?f=", item._id.to_hex());
    let result = agent.get(&url).call();
    match ureq_result_to_reader(result) {
        Ok(mut r) => match copy(&mut r, &mut stdout()) {
            Ok(_) => ActionOutput::response("Server shutdown.".to_string()),
            Err(_) => ActionOutput::response("Errored".to_string()),
        },
        Err((code, s)) => match code {
            500 | 401 => ActionOutput::new(s, Frame::Forums),
            _ => ActionOutput::response(s),
        },
    }
}

pub fn message(agent: &Agent, item: &mut ForumListItem) -> ActionOutput {
    println!("Message:");
    let message = stdin_line();
    let url = format!("{}{}", HOST, "forum/message");
    let result = agent
        .post(&url)
        .send_form(&[("forum_hex_id", &item._id.to_hex()), ("value", &message)]);
    match ureq_result_to_string(result) {
        Ok(s) => ActionOutput::response(s),
        Err((code, s)) => match code {
            500 | 401 => ActionOutput::new(s, Frame::Forums),
            _ => ActionOutput::response(s),
        },
    }
}
