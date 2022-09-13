mod frames;
mod util;

pub const HOST: &'static str = "http://127.0.0.1:8000/";

pub enum Frame {
    Home,
    Forums,
    OwnedForums(Vec<frames::forums::ForumListItem>),
    OwnedForum(frames::forums::ForumListItem),
    PermittedForums(Vec<frames::forums::ForumListItem>),
    PermittedForum(frames::forums::ForumListItem),
}

pub struct ActionOutput {
    response: Option<String>,
    frame_redirect: Option<Frame>,
}

impl ActionOutput {
    fn new(response: String, frame: Frame) -> Self {
        Self {
            response: Some(response),
            frame_redirect: Some(frame),
        }
    }

    fn response(response: String) -> Self {
        Self {
            response: Some(response),
            frame_redirect: None,
        }
    }

    fn redirect(frame: Frame) -> Self {
        Self {
            response: None,
            frame_redirect: Some(frame),
        }
    }
}

fn respond_redirect(output: ActionOutput, mut frame: Frame) -> Frame {
    if let Some(r) = output.response {
        println!("{r}");
    }
    if let Some(f) = output.frame_redirect {
        frame = f;
    }
    frame
}

fn main() {
    use crate::Frame::*;
    let agent = ureq::agent();
    let mut frame = Frame::Home;

    loop {
        match &mut frame {
            Home => {
                let output = frames::home::run(&agent);
                frame = respond_redirect(output, frame);
            }
            Forums => {
                let output = frames::forums::run(&agent);
                frame = respond_redirect(output, frame);
            }
            OwnedForums(forum_list) => match frames::forums::run_get_item(forum_list) {
                Ok(item) => frame = OwnedForum(item),
                Err(output) => frame = respond_redirect(output, frame),
            },
            OwnedForum(forum_item) => {
                let output = frames::forum::owned::run(&agent, forum_item);
                frame = respond_redirect(output, frame);
            }
            PermittedForums(forum_list) => match frames::forums::run_get_item(forum_list) {
                Ok(item) => frame = PermittedForum(item),
                Err(output) => frame = respond_redirect(output, frame),
            },
            PermittedForum(forum_item) => {
                let output = frames::forum::permitted::run(&agent, forum_item);
                frame = respond_redirect(output, frame);
            }
        }
    }
}
