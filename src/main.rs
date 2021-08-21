#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate json;

use std::env;
use std::io;
use std::io::*;
use std::process;

const ANSI_RESET: &'static str = "\x1b[0m";
const ANSI_RED: &'static str = "\x1b[31m";
const ANSI_GREEN: &'static str = "\x1b[32m";
const ANSI_YELLOW: &'static str = "\x1b[33m";
const ANSI_MAGENTA: &'static str = "\x1b[35m";
const ANSI_CYAN: &'static str = "\x1b[36m";
const ANSI_BRIGHT_WHITE: &'static str = "\x1b[97m";

mod dictionary;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let action = env::args().nth(1);

    let include_ids = arguments.contains(&"--id".to_string());
    let tag_per_line = arguments.contains(&"--tag-per-line".to_string());
    let color = arguments.contains(&"--color".to_string());

    match action.as_ref().map(String::as_ref) {
        Some("decode") => process_stdin(include_ids, tag_per_line, color),
        Some(a) => die(&format!("unsupported action '{}'", &a)),
        None => die("no action specified"),
    }
}

fn process_stdin(include_ids: bool, tag_per_line: bool, color: bool) {
    let stdin = io::stdin();
    let in_reader = BufReader::new(stdin);
    let mut stdout = io::stdout();

    for line in in_reader.lines() {
        let translation = match line.as_ref().map(String::as_ref) {
                              Ok(text) => decode_line(text, include_ids, tag_per_line, color).to_string(),
                              Err(error) => error.to_string(),
                          };

        write!(stdout, "{}\n", translation).unwrap();
    }
}

fn enquote(text: &str) -> String {
    return if text == "" || text.contains(' ') { format!("'{}'", text) } else { text.to_string() };
}

fn decode_line(line: &str, include_ids: bool, tag_per_line: bool, color: bool) -> String {
    lazy_static! {
        static ref PATTERN: regex::Regex = regex::Regex::new(r"8=FIX\.\d+\.\d+\1.*?10=\d{3}\1").unwrap();
    }

    if !PATTERN.is_match(line) { return line.to_string(); }

    let mut translation = String::new();

    for field in line.split("\x01") {
        let pair: Vec<&str> = field.splitn(2, "=").collect();
        if pair.len() < 2 { continue; }

        let tag = pair[0];
        let value = pair[1];

        let decoded_tag = decode_tag(tag);
        let decoded_value = decode_value(tag, value);

        if include_ids
        {
            translation.push_str(&colorize(tag, &ANSI_CYAN, color));
            translation.push(':');
        }

        translation.push_str(&colorize(&decoded_tag, &ANSI_BRIGHT_WHITE, color));

        translation.push_str("=");

        if include_ids && value != decoded_value
        {
            translation.push_str(&colorize(value, &ANSI_YELLOW, color));
            translation.push(':');
        }

        translation.push_str(&enquote(&colorize_by_type(value, &decoded_value, color)));

        translation.push(if tag_per_line { '\n' } else {' '});
    }

    if tag_per_line
    {
        translation.push_str("-\n");
    }

    translation.pop();

    return translation;
}

fn colorize(text: &str, ansi_color_code: &str, use_color: bool) -> String {
    let mut result = String::new();

    if use_color { result.push_str(ansi_color_code); }
    result.push_str(text);
    if use_color { result.push_str(&ANSI_RESET); }

    return result;
}

fn colorize_by_type(value: &str, decoded: &str, use_color: bool) -> String {
    lazy_static! {
        static ref DATETIME_PATTERN: regex::Regex = regex::Regex::new(r"\d{8}-\d{2}:\d{2}:\d{2}(.\d{3})?").unwrap();
    }

    let is_decoded = decoded != value;
    let is_number = value.chars().all(|c| char::is_numeric(c) || c == '.');
    let is_date = DATETIME_PATTERN.is_match(value);

    let color_code = if is_decoded { ANSI_RED }
                             else if is_number { ANSI_CYAN }
                             else if is_date { ANSI_MAGENTA }
                             else { ANSI_GREEN };

    return colorize(decoded, color_code, use_color);
}

fn decode_tag(tag: &str) -> String {
     let name = &dictionary::DICTIONARY["tags"][tag];

    return if !name.is_null() { name.to_string() } else { tag.to_string() };
}

fn decode_value<'a>(tag: &'a str, value: &'a str) -> String {
    let enumeration = &dictionary::DICTIONARY["enums"][tag];
    return if !enumeration.is_null() { enumeration[value].to_string() } else { value.to_string() };
}

fn die(message: &str) {
    println!("{}", message);
    process::exit(1);
}
