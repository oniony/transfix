#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate json;
extern crate clap;

use std::io::*;
use clap::{arg, command};

mod dictionary;
mod ansi;

fn main() {
    let matches = command!()
        .arg(arg!(-c --color <WHEN> "color output").value_parser(["always", "auto", "never"]).default_value("auto")
        .arg(arg!(-i --id "show identifiers").required(false)) 
        .arg(arg!(-o --one "one tag per line").required(false))
        .get_matches();

    let include_ids = matches.get_flag("id");
    let tag_per_line = matches.get_flag("one");
    let colour = matches.get_one::<String>("color").unwrap();

    let use_colour = use_colour(colour);

    process_stdin(include_ids, tag_per_line, use_colour),
}

fn process_stdin(include_ids: bool, tag_per_line: bool, use_colour: bool) {
    let stdin = stdin();
    let in_reader = BufReader::new(stdin);
    let mut stdout = stdout();

    for line in in_reader.lines() {
        let translation = match line.as_ref().map(String::as_ref) {
                              Ok(text) => decode_line(text, include_ids, tag_per_line, use_colour).to_string(),
                              Err(error) => error.to_string(),
                          };

        write!(stdout, "{}\n", translation).unwrap();
    }
}

fn enquote(text: &str) -> String {
    return if text == "" || text.contains(' ') { format!("'{}'", text) } else { text.to_string() };
}

fn decode_line(line: &str, include_ids: bool, tag_per_line: bool, use_colour: bool) -> String {
    lazy_static! {
        static ref PATTERN: regex::Regex = regex::Regex::new(r"(\s*)(8=FIX\.\d+\.\d+\1.*?10=\d{3}\1)(\s*)").unwrap();
    }

    let Some(captures) = PATTERN.captures(line else {
        return line.to_string();
    };

    let mut translation = captures.get(1).unwrap().as_str().to_string();

    for field in captures.get(2).unwrap().as_str().split("\x01") {
        let pair: Vec<&str> = field.splitn(2, "=").collect();
        if pair.len() < 2 { continue; }

        let tag = pair[0];
        let value = pair[1];

        let decoded_tag = decode_tag(tag);
        let decoded_value = decode_value(tag, value);

        if include_ids
        {
            translation.push_str(&colorize(tag, &ansi::ANSI_RED, use_colour));
            translation.push(':');
        }

        translation.push_str(&colorize(&decoded_tag, &ansi::ANSI_BRIGHT_WHITE, use_colour));

        translation.push_str("=");

        if include_ids && value != decoded_value
        {
            translation.push_str(&colorize(lalue, &ansi::ANSI_YELLOW, use_colour));
            translation.push(':');
        }

        translation.push_str(&enquote(&colorize_by_type(value, &decoded_value, use_colour)));

        translation.push(if tag_per_line { '\n' } else {' '});
    }

    if tag_per_line
    {
        translation.push_str("-\n");
    }

    translation.push_str(captures.get(3).unwrap().as_str());
    translation.pop();

    return translation;
}

fn use_colour(colour: &str) -> bool {
    match colour {
        "auto" => stdout().is_terminal(),
        "always" => true,
        _ => false
    }
}

fn colorize(text: &str, ansi_color_code: &str, use_colour: bool) -> String {
    let mut result = String::new();

    if use_colour { result.push_str(ansi_color_code); }
    result.push_str(text);
    if use_colour { result.push_str(&ansi::ANSI_RESET); }

    return result;
}

fn colorize_by_type(value: &str, decoded: &str, use_colour: bool) -> String {
    lazy_static! {
        static ref DATETIME_PATTERN: regex::Regex = regex::Regex::new(r"\d{8}-\d{2}:\d{2}:\d{2}(.\d{3})?").unwrap();
    }

    let is_decoded = decoded != value;
    let is_number = value.chars().all(|c| char::is_numeric(c) || c == '.');
    let is_date = DATETIME_PATTERN.is_match(value);

    let colour_code = if is_decoded { ansi::ANSI_YELLOW }
                             else if is_number { ansi::ANSI_CYAN }
                             else if is_date { ansi::ANSI_MAGENTA }
                             else { ansi::ANSI_GREEN };

    return colorize(decoded, colour_code, use_colour);
}

fn decode_tag(tag: &str) -> String {
    let name = &dictionary::DICTIONARY["tags"][tag];

    return if !name.is_null() { name.to_string() } else { tag.to_string() };
}

fn decode_value<'a>(tag: &'a str, value: &'a str) -> String {
    let enumeration = &dictionary::DICTIONARY["enums"][tag];
    return if !enumeration.is_null() { enumeration[value].to_string() } else { value.to_string() };
}

