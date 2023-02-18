use crate::frames::forum::listen_and_message;
use crate::frames::forums::ForumListItem;
use crate::frames::home::User;
use crate::util::{proccess_input, stdin_line, ureq_result_to_json, ureq_result_to_string};
use crate::{ActionOutput, Frame, HOST};

use serde::{Deserialize, Serialize};
use ureq::Agent;

#[derive(Deserialize)]
struct ForumItem {
    permitted_users: Vec<String>,
}

fn view_permitted_users(agent: &Agent, i: &mut ForumListItem) -> ActionOutput {
    let url = format!("{}{}{}", HOST, "forum/forum?f=", i._id.to_hex());
    let result = agent
        .get(&url)
        .set("Origin", "http://127.0.0.1:5173")
        .call();
    match ureq_result_to_json::<ForumItem>(result) {
        Ok(item) => {
            let response = format!(
                "Forum '{}' permitted users:\n{:#?}",
                i.name, item.permitted_users
            );
            ActionOutput::response(response)
        }
        Err((code, s)) => match code {
            500 | 401 => ActionOutput::new(s, Frame::Home),
            _ => ActionOutput::response(s),
        },
    }
}

#[derive(Serialize)]
struct PermittedUsers(Vec<String>);

#[derive(Serialize)]
struct UpdateUsers {
    forum_hex_id: String,
    permitted_users: PermittedUsers,
}

fn update_users(agent: &Agent, item: &mut ForumListItem) -> ActionOutput {
    println!("Enter a comma separated list of users:");
    let users = stdin_line();
    let url = format!("{}{}", HOST, "forum/update-users");
    let result = agent
        .post(&url)
        .set("Origin", "http://127.0.0.1:5173")
        .send_form(&[
            ("forum_hex_id", &item._id.to_hex()),
            ("permitted_users", &users),
        ]);
    match ureq_result_to_string(result) {
        Ok(s) => {
            let new_forum_item = ForumListItem {
                _id: item._id,
                name: item.name.to_owned(),
                owner: item.owner.to_owned(),
            };
            ActionOutput::new(s, Frame::OwnedForum(new_forum_item))
        }
        Err((code, s)) => match code {
            500 | 401 => ActionOutput::new(s, Frame::Home),
            _ => ActionOutput::response(s),
        },
    }
}

pub fn run(agent: &Agent, user: &User, forum_item: &mut ForumListItem) -> Result<(), ActionOutput> {
    println!(
        "Forum '{}':\n(1) 'view permitted users'\n(2) 'update permitted users'\n(3) 'chat'",
        forum_item.name,
    );
    let input: u16 = proccess_input()?;
    match input {
        1 => Err(view_permitted_users(agent, forum_item)),
        2 => Err(update_users(agent, forum_item)),
        3 => listen_and_message(agent, user, forum_item),
        _ => Err(ActionOutput::response(format!(
            "Action '{input}' not available"
        ))),
    }
}
