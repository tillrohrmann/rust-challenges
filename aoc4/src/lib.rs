use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::str::FromStr;
use chrono::prelude::*;
use regex::Regex;
use regex::Captures;
use regex::Match;
use std::error;
use std::fmt;
use std::fmt::Formatter;
use std::num::ParseIntError;

#[macro_use]
extern crate lazy_static;

pub type ParseResult<T> = Result<T, Box<error::Error>>;

#[derive(Debug)]
struct ParseError {
    error_message: String,
}

impl ParseError {
    fn new(message: &str) -> ParseError {
        ParseError {
            error_message: String::from(message),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Parsing error: {}", self.error_message)
    }
}

impl error::Error for ParseError {}

#[derive(Debug, PartialEq)]
struct LogEntry {
    date_time: DateTime<Utc>,
    action: Action
}

impl LogEntry {
    fn new(date_time: DateTime<Utc>, action: Action) -> LogEntry {
        LogEntry {
            date_time,
            action
        }
    }

    pub fn parse_from(input: &str) -> ParseResult<LogEntry> {
        lazy_static! {
            static ref LOG_REGEX: Regex = Regex::new(r"\[(\d+)-(\d+)-(\d+) (\d+):(\d+)\] (.*)").unwrap();
        }

        if let Some(captures) = LOG_REGEX.captures(input) {
            let year = LogEntry::parse_capture(&captures, 1)? as i32;
            let month = LogEntry::parse_capture(&captures, 2)?;
            let day = LogEntry::parse_capture(&captures, 3)?;
            let hour = LogEntry::parse_capture(&captures, 4)?;
            let minutes = LogEntry::parse_capture(&captures, 5)?;

            let action_str = LogEntry::get_capture(&captures, 6)?;

            Ok(LogEntry::new(Utc.ymd(year, month, day).and_hms(hour, minutes, 0), Action::parse_from(&action_str)?))
        } else {
            Err(ParseError::new(&format!("Could not parse input: {}", input)).into())
        }
    }

    fn get_capture(captures: &Captures, index: usize) -> ParseResult<String> {
        captures
            .get(index)
            .map(|m| m.as_str().to_string())
            .ok_or(ParseError::new(&format!("Could not find capture group {}.", index)).into())
    }


    fn parse_capture(captures: &Captures, index: usize) -> ParseResult<u32> {
        LogEntry::get_capture(captures, index)
            .and_then(|m| m.parse::<u32>().map_err(|e| e.into()))
    }
}

#[derive(Debug, PartialEq)]
enum Action {
    Guard(u32),
    WakesUp,
    FallsAsleep,
}

static FALLS_ASLEEP: &str = "falls asleep";
static WAKES_UP: &str = "wakes up";

impl Action {
    pub fn parse_from(input: &str) -> ParseResult<Action> {
        let input = input.trim().to_lowercase();

        lazy_static! {
            static ref GUARD_REGEX: Regex = Regex::new(r"guard #(\d*) begins shift").unwrap();
        }

        if input == WAKES_UP {
            Ok(Action::WakesUp)
        } else if input == FALLS_ASLEEP {
            Ok(Action::FallsAsleep)
        } else if let Some(captures) = GUARD_REGEX.captures(&input) {
            match captures.get(1) {
                Some(m) => {
                    let guard_number = u32::from_str(m.as_str())?;
                    Ok(Action::Guard(guard_number))
                },
                None => Err(ParseError::new("Guard with no number.").into())
            }

        } else {
            Err(ParseError::new(&format!("Could not parse input: {}", input)).into())
        }
    }
}

pub fn read_file(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(&file);

    let mut result = Vec::with_capacity(32);

    for line in buf_reader.lines() {
        result.push(line?)
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_parsing_wakes_up() {
        assert_eq!(Action::parse_from("wakes up").unwrap(), Action::WakesUp);
    }

    #[test]
    fn test_action_parsing_falls_asleep() {
        assert_eq!(Action::parse_from("falls asleep").unwrap(), Action::FallsAsleep);
    }

    #[test]
    fn test_action_parsing_guard() {
        assert_eq!(Action::parse_from("Guard #42 begins shift").unwrap(), Action::Guard(42));
    }

    #[test]
    fn test_log_parsing() {
        assert_eq!(LogEntry::parse_from("[1518-07-13 23:58] Guard #2521 begins shift").unwrap(), LogEntry::new(Utc.ymd(1518, 7, 13).and_hms(23, 58, 0), Action::Guard(2521)));
    }
}
