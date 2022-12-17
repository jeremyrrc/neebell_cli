use crate::frames::forum::{listen, message, listen_and_message};
use crate::frames::forums::ForumListItem;
use crate::frames::home::User;
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

pub fn run(agent: &Agent, user: &User, forum_item: &mut ForumListItem) -> Result<(), ActionOutput> {
    println!(
        "Forum '{}':\n(1) 'view permitted users'\n(2) 'update permitted users'\n(3) 'listen'\n(4) 'message'\n(5) 'listen and message'",
        forum_item.name,
    );
    let input: u16 = proccess_input()?;
    match input {
        1 => Err(view_permitted_users(forum_item)),
        2 => Err(update_users(agent, forum_item)),
        3 => listen(agent, user, forum_item),
        4 => message(agent, user, forum_item),
        5 => listen_and_message(agent, user, forum_item),
        _ => Err(ActionOutput::response(format!("Action '{input}' not available"))),
    }
}
