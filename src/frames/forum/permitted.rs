use crate::frames::forum::{listen, message};
use crate::frames::forums::ForumListItem;
use crate::util::proccess_input;
use crate::ActionOutput;

use ureq::Agent;

pub fn run(agent: &Agent, forum_item: &mut ForumListItem) -> ActionOutput {
    println!("Forum '{}':\n(1) 'listen'\n(2) 'message'", forum_item.name,);
    let input: u16 = match proccess_input() {
        Ok(n) => n,
        Err(a) => return a,
    };
    match input {
        1 => listen(agent, forum_item),
        2 => message(agent, forum_item),
        _ => ActionOutput::response(format!("Action '{}' not available", input)),
    }
}
