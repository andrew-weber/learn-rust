use diesel_demo::*;
use std::io::{stdin, Read};

fn main() {
    let conn = &mut establish_connection();

    let mut title = String::new();
    let mut body = String::new();

    println!("Title:\n");
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end();

    println!("Write a message for {} then press {}\n", title, EOF);
    stdin().read_to_string(&mut body).unwrap();

    let post = create_post(conn, title, &body);

    println!("\nSaved {} with id {}", title, post.id);
}

#[cfg(not(windows))]
const EOF: &str = "ctrl + d";

#[cfg(windows)]
const EOF: &str = "ctrl + z";
