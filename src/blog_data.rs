use chrono::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use std::path::Path;

pub fn get_posts() -> Result<Vec<Post>, std::io::Error> {
    let mut posts = 
        fs::read_dir("posts")?
            .filter_map(|entry| Some(entry.ok()?.path()))
            .filter(|path| path.is_file() && path.extension().unwrap_or_default() == "md")
            .filter_map(|path| get_post_data(&path))
            .collect::<Vec<Post>>();

    posts.sort_by_key(|j| j.timestamp);
    posts.reverse();
    Ok(posts)
}

fn get_post_data(path: &Path) -> Option<Post> {
    let contents = fs::read_to_string(path)
        .expect("Unable to read from file");
    let image_url = extract_first_image(&contents[..]).unwrap_or_default();
    let title = extract_post_title(&contents[..])?;
    let timestamp = extract_post_timestamp(&contents[..]).unwrap_or_default();

    Some(Post {
        name: String::from(path.file_stem()?.to_str()?),
        title: title,
        thumbnail: image_url,
        date: Utc.timestamp(timestamp as i64, 0).format("%d %B %Y").to_string(),
        timestamp: timestamp
    })
}


fn extract_first_image(markdown: &str) -> Option<String> {
    lazy_static! {
        static ref IMAGE_REGEX: Regex = Regex::new(r"!\[\]\(([^\^\n)]+)\)").unwrap();
    }
    // Return $0 from ![]($0), returns None if not found
    Some(
        to_absolute_url(
            IMAGE_REGEX.captures(markdown)?
            .get(1)?
            .as_str()
        )
    )
}

fn to_absolute_url(url: &str) -> String {
    let mut absolute_url = String::new();
    if !url.is_empty() && &url[0..1] == "/" {
        absolute_url = String::from("https://jam1.re");
    }
    absolute_url + url
}


fn extract_post_title(markdown: &str) -> Option<String> {
    lazy_static! {
        static ref TITLE_REGEX: Regex = Regex::new(r"(?m)^ *#(.+)").unwrap();
    }
    // Return the first #-level title, returns None if not found
    Some(
        TITLE_REGEX.captures(markdown)?
            .get(1)?
            .as_str()
            .to_string()
    )
}

fn extract_post_timestamp(markdown: &str) -> Option<u64> {
    lazy_static! {
        static ref TIMESTAMP_REGEX: Regex = Regex::new(r"<!-- *timestamp: *(\d+) *-->").unwrap();
    }
    // Return the first num from html comment in format timestamp:[num], returns None if not found
    TIMESTAMP_REGEX.captures(markdown)?
        .get(1)?
        .as_str()
        .parse::<u64>()
        .ok()
}


pub struct Post {
    name: String,
    title: String,
    thumbnail: String,
    date: String,
    timestamp: u64
}

impl Post {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            title: String::new(),
            thumbnail: String::new(),
            date: String::new(),
            timestamp: 0,
        }
    }
}
