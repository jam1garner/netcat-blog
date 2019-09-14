use crate::server::ansi::Ansi;
use crate::blog_data::{Post, get_posts};
use ansi_parser::AnsiSequence as AnsiEsc;

pub struct PostHandler {
    pub index: usize,
    pub scroll: usize,
    pub posts: Vec<Post>,
}

impl PostHandler {
    pub fn new() -> Self {
        Self {
            index: 0,
            scroll: 0,
            posts: vec![Post::new(), Post::new(), Post::new()]//get_posts().unwrap(),
        }
    }

    pub fn handle_inputs(&mut self, inputs: Vec<Ansi>) {
        for input in inputs {
            match input {
                Ansi::Escape(AnsiEsc::CursorForward(_)) => {
                    self.index = (self.index + 1) % self.posts.len();
                }
                Ansi::Escape(AnsiEsc::CursorBackward(_)) => {
                    self.index = if self.index == 0 {
                                    self.posts.len() - 1
                                } else {
                                    self.index - 1
                                };
                }
                Ansi::Escape(AnsiEsc::CursorUp(_)) => {
                    if self.scroll > 0 {
                        self.scroll -= 1;
                    }
                }
                Ansi::Escape(AnsiEsc::CursorDown(_)) => {
                    self.scroll += 1;
                }
                x @ _ => {
                    println!("{:?}", x);
                }
            }
        }
    }
}
