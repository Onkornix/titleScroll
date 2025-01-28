use clap::Parser;
use std::fmt::Write;
use std::process::Command;
use std::thread::sleep;
use std::time;
use unicode_segmentation::UnicodeSegmentation;
// Songs with problems:
// Venom - Die Hard (12" Version)
//

enum Direction {
    Left,
    Right,
}
#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 30)]
    max_length: usize,
}

fn main() {
    let args = Args::parse();

    let max_length = args.max_length;

    loop {
        let title = get_title();

        let length = title.chars().count();

        if length > max_length {
            scroll_loop(Direction::Right, &title, max_length);

            if title_change_check(&title) {
                continue;
            }

            sleep(time::Duration::from_millis(3000));

            if title_change_check(&title) {
                continue;
            }

            scroll_loop(Direction::Left, &title, max_length);

            if title_change_check(&title) {
                continue;
            }

            sleep(time::Duration::from_millis(3000));
        } else {
            println!("{}", title);
            sleep(time::Duration::from_millis(1000));
        }
    }
}

fn scroll_loop(direction: Direction, title: &String, max_length: usize) {
    let length = title.chars().count();

    let title_chars: Vec<char> = title.chars().collect();

    let mut current_section: (usize, usize) = match direction {
        Direction::Left => (length - max_length, length),
        Direction::Right => (0, max_length),
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
                    break;
                }
            }
            Direction::Right => {
                if current_section.1 == length || title_change_check(&title) {
                    break;
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
        let mut i = 0;
        let len = chunk.valid().graphemes(true).count() as i32;

        for ch in chunk.valid().graphemes(true) {
            if i == 0 {
                i += 1;
                continue;
            }
            if i >= len - 2 {
                continue;
            }
            i += 1;

            if ch == "\"" || ch == "\'" {
                write!(&mut title, "{}", ch.trim_start()).expect("failed to write to title buffer");
                continue;
            }
            write!(&mut title, "{}", ch.escape_debug()).expect("failed to write to title buffer");

        }
        for _ in chunk.invalid() {
            write!(&mut title, "?").expect("failed to write to title buffer");
        }

    }
    title
}

