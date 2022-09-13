use crate::frames::forum::{listen, message};
use crate::frames::forums::ForumListItem;
use crate::util::{proccess_input, stdin_line, ureq_result_to_string};
use crate::{ActionOutput, Frame, HOST};

use serde::Serialize;
use ureq::Agent;

fn view_permitted_users(i: &mut ForumListItem) -> ActionOutput {
    let response = format!(
        "Forum '{}' permitted users:\n{:#?}",
        i.name, i.permitted_users
    );
    ActionOutput::response(response)
}

#[derive(Serialize)]
struct PermittedUsers(Vec<String>);

#[derive(Serialize)]
struct UpdateUsers {
    id: String,
    permitted_users: PermittedUsers,
}

fn update_users(agent: &Agent, item: &mut ForumListItem) -> ActionOutput {
    println!("Enter a comma separated list of users:");
    let users: Vec<String> = stdin_line()
        .split(',')
        .map(|s| s.trim().to_owned())
        .collect();
    let update = UpdateUsers {
        id: item._id.to_hex(),
        permitted_users: PermittedUsers(users.to_owned()),
    };
    let url = format!("{}{}", HOST, "forum/update-users");
    let result = agent.post(&url).send_json(update);
    match ureq_result_to_string(result) {
        Ok(s) => {
            let new_forum_item = ForumListItem {
                _id: item._id,
                name: item.name.to_owned(),
                permitted_users: users,
            };
            ActionOutput::new(s, Frame::OwnedForum(new_forum_item))
        }
        Err((code, s)) => match code {
            500 | 401 => ActionOutput::new(s, Frame::Home),
            _ => ActionOutput::response(s),
        },
    }
}

pub fn run(agent: &Agent, forum_item: &mut ForumListItem) -> ActionOutput {
    println!(
        "Forum '{}':\n(1) 'view permitted users'\n(2) 'update permitted users'\n(3) 'listen'\n(4) 'message'",
        forum_item.name,
    );
    let input: u16 = match proccess_input() {
        Ok(n) => n,
        Err(a) => return a,
    };
    match input {
        1 => view_permitted_users(forum_item),
        2 => update_users(agent, forum_item),
        3 => listen(agent, forum_item),
        4 => message(agent, forum_item),
        _ => ActionOutput::response(format!("Action '{}' not available", input)),
    }
}
