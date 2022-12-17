use crate::util::{proccess_input, stdin_line, ureq_result_to_json, ureq_result_to_string};
use crate::{ActionOutput, Frame, HOST};
use bson::oid::ObjectId;
use serde::Deserialize;
use std::fmt::Debug;
use ureq::Agent;

fn create(agent: &Agent) -> ActionOutput {
    println!("Name:");
    let name = stdin_line();

    let url = format!("{}{}", HOST, "forum/create");
    let result = agent.post(&url).send_form(&[("name", &name)]);
    match ureq_result_to_string(result) {
        Ok(s) => ActionOutput::response(s),
        Err((code, s)) => match code {
            500 | 401 => ActionOutput::new(s, Frame::Home),
            _ => ActionOutput::response(s),
        },
    }
}

#[derive(Debug, Deserialize)]
pub struct ForumListItem {
    pub _id: ObjectId,
    pub name: String,
    pub permitted_users: Vec<String>,
}

fn list_owned(agent: &Agent) -> ActionOutput {
    let url = format!("{}{}", HOST, "forum/list-owned");
    let result = agent.get(&url).call();
    match ureq_result_to_json::<Vec<ForumListItem>>(result) {
        Ok(v) => ActionOutput::new("Forums owned by you:".to_string(), Frame::OwnedForums(v)),
        Err((code, s)) => match code {
            500 | 401 => ActionOutput::new(s, Frame::Home),
            _ => ActionOutput::response(s),
        },
    }
}

fn list_permitted(agent: &Agent) -> ActionOutput {
    let url = format!("{}{}", HOST, "forum/list-permitted");
    let result = agent.get(&url).call();
    match ureq_result_to_json::<Vec<ForumListItem>>(result) {
        Ok(v) => ActionOutput::new("Permitted forums:".to_string(), Frame::PermittedForums(v)),
        Err((code, s)) => match code {
            500 | 401 => ActionOutput::new(s, Frame::Home),
            _ => ActionOutput::response(s),
        },
    }
}

pub fn run(agent: &Agent) -> ActionOutput {
    println!("Forums:\n(1) 'create new forum'\n(2) 'list forums owned by you'\n(3) 'list permitted forums'");
    let input: u16 = match proccess_input() {
        Ok(n) => n,
        Err(a) => return a,
    };
    match input {
        1 => create(agent),
        2 => list_owned(agent),
        3 => list_permitted(agent),
        _ => ActionOutput::response(format!("Action '{input}' not available")),
    }
}

pub fn items_to_prompt(items: &[ForumListItem]) -> String {
    let mut s = String::new();
    for (count, item) in items.iter().enumerate() {
        let line = format!("({}) '{}'\n", count, item.name);
        s.push_str(&line);
    }
    s
}

pub fn run_get_item(forum_items: &mut Vec<ForumListItem>) -> Result<ForumListItem, ActionOutput> {
    let prompt = items_to_prompt(forum_items);
    print!("{prompt}");
    let index = proccess_input::<usize>()?;
    if index >= forum_items.len() {
        return Err(ActionOutput::response(format!(
            "Action {index} not available",
        )));
    }
    let item = forum_items.remove(index);
    Ok(item)
}
