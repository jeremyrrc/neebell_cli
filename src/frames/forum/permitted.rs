use crate::frames::forum::listen_and_message;
use crate::frames::forums::ForumListItem;
use crate::frames::home::User;
use crate::ActionOutput;

use ureq::Agent;

pub fn run(agent: &Agent, user: &User, forum_item: &mut ForumListItem) -> Result<(), ActionOutput> {
    println!("Forum '{}'", forum_item.name,);
    listen_and_message(agent, user, forum_item)
}
