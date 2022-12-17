use crate::frames::forum::{listen, message, listen_and_message};
use crate::frames::forums::ForumListItem;
use crate::frames::home::User;
use crate::util::proccess_input;
use crate::ActionOutput;

use ureq::Agent;

pub fn run(agent: &Agent, user: &User, forum_item: &mut ForumListItem) -> Result<(), ActionOutput> {
    println!("Forum '{}':\n(1) 'listen'\n(2) 'message'\n(3) 'listen and message'", forum_item.name,);
    let input: u16 = proccess_input()?;
    match input {
        1 => listen(agent, user, forum_item),
        2 => message(agent, user, forum_item),
        3 => listen_and_message(agent, user, forum_item),
        _ => Err(ActionOutput::response(format!("Action '{input}' not available"))),
    }
}
