use std::process::Command;
use std::thread::{sleep};
use std::time;
use std::fmt::Write;

enum Direction {
    Left,
    Right
}

fn main() {

    let max_length = 30;

    loop {
        let title = get_title();

        let length = title.chars().count();

        if length > max_length {
            scroll_loop(Direction::Right, &title);

            if title_change_check(&title) { continue }

            sleep(time::Duration::from_millis(3000));

            if title_change_check(&title) { continue }

            scroll_loop(Direction::Left, &title);

            if title_change_check(&title) { continue }

            sleep(time::Duration::from_millis(3000));
        } else {
            println!("{}", title);
            sleep(time::Duration::from_millis(1000));
        }
    }
}

fn scroll_loop(direction: Direction, title: &String) {
    let length = title.chars().count();

    let title_chars: Vec<char> = title
        .chars()
        .collect();

    let mut current_section: (usize,usize) = match direction {
        Direction::Left => (length - 30,length),
        Direction::Right => (0,30)
    };

    loop {
        let mut title_section = String::new();
        let title_section_vector = Vec::from(&title_chars[current_section.0..current_section.1]);

        for ch in title_section_vector {
            title_section.push(ch);
        }

        println!("{}", title_section);

        match direction {
            Direction::Left => {
                if current_section.0 == 0 || title_change_check(&title) {
                    break
                }
            }
            Direction::Right => {
                if current_section.1 == length || title_change_check(&title) {
                    break
                }
            }
        }

        match direction {
            Direction::Left => {
                current_section.0 -= 1;
                current_section.1 -= 1;
            }
            Direction::Right => {
                current_section.0 += 1;
                current_section.1 += 1;
            }
        }

        sleep(time::Duration::from_millis(300))
    }
}
fn title_change_check(title: &String) -> bool {
    title != &get_title()
}
fn get_title() -> String {
    let command_stdout = Command::new("playerctl")
        .args(["metadata", "--format", "'{{ title }}'"])
        .output()
        .expect("failed to run command")
        .stdout;


    let utf_8 = command_stdout.utf8_chunks();
    let mut title = String::new();

    for chunk in utf_8 {
        for ch in chunk.valid().chars() {
            if ['\\','\'','\n'].contains(&ch) {
                continue
            }
            write!(&mut title, "{}", ch.escape_debug())
                .expect("failed to write to title buffer");
        }
        for _byte in chunk.invalid() {
            write!(&mut title, "?")
                .expect("failed to write to title buffer");
        }
    }
    title
}