#[macro_use]
extern crate lazy_static;

use std::error;
use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::num::ParseIntError;
use std::str::FromStr;

use chrono::prelude::*;
use regex::Captures;
use regex::Match;
use regex::Regex;
use std::collections::HashMap;
use std::thread::current;
use std::fmt::Debug;
use std::fmt::Error;
use std::collections::hash_map::Values;


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
pub struct LogEntry {
    date_time: DateTime<Utc>,
    action: Action
}

impl LogEntry {
    pub fn new(date_time: DateTime<Utc>, action: Action) -> LogEntry {
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
pub enum Action {
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

pub fn read_log(path: &str) -> ParseResult<Vec<LogEntry>> {
    let file_content = read_file(path)?;

    let mut result = file_content.iter().map(|line| LogEntry::parse_from(line)).collect::<ParseResult<Vec<LogEntry>>>()?;

    result.sort_by(|a, b| a.date_time.cmp(&b.date_time));

    Ok(result)
}

pub struct Guard {
    id: u32,
    total_minutes_asleep: u32,
    minutes_asleep: [u32; 60],
}

impl PartialEq for Guard {
    fn eq(&self, other: &Guard) -> bool {
        self.id == other.id &&
            self.total_minutes_asleep == other.total_minutes_asleep &&
            self.minutes_asleep.iter().zip(other.minutes_asleep.iter()).all(|(a, b)| a == b)
    }
}

impl Debug for Guard {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Guard {{ id: {}, total_minutes_asleep: {}, minutes_asleep: {:?} }}", self.id, self.total_minutes_asleep, &self.minutes_asleep[..])
    }
}

impl Guard {
    pub fn new(id: u32) -> Guard {
        Guard {
            id,
            total_minutes_asleep: 0,
            minutes_asleep: [0; 60],
        }
    }

    pub fn mark_asleep(&mut self, start: u32, end: u32) {
        for index in start..end {
            self.minutes_asleep[index as usize] += 1;
        }

        self.total_minutes_asleep += end - start;
    }

    pub fn get_total_minutes_asleep(&self) -> u32 {
        self.total_minutes_asleep
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn find_minute_most_often_asleep(&self) -> usize {
        let (result, _) = self.minutes_asleep.iter().enumerate().max_by(|&(_, a), &(_, b)| a.cmp(b)).unwrap();
        result
    }

    pub fn get_minute(&self, index: usize) -> u32 {
        self.minutes_asleep[index]
    }
}

#[derive(PartialEq, Debug)]
pub struct GuardOverview {
    guards: std::collections::HashMap<u32, Guard>,
}

impl GuardOverview {
    pub fn new() -> GuardOverview {
        GuardOverview {
            guards: std::collections::HashMap::with_capacity(10),
        }
    }

    pub fn get_or_insert(&mut self, id: u32) -> &mut Guard {
        self.guards.entry(id).or_insert_with(|| Guard::new(id))
    }

    pub fn iter(&self) -> Values<u32, Guard> {
        self.guards.values()
    }
}

pub fn process_log(log: &Vec<LogEntry>) -> GuardOverview {
    let mut current_guard: Option<u32> = None;
    let mut asleep_since: Option<u32> = None;
    let mut guard_overview = GuardOverview::new();

    for log_entry in log {
        match log_entry.action {
            Action::Guard(id) => {
                match (current_guard, asleep_since) {
                    (_, None) => {
                        current_guard = Some(id);
                    }
                    _ => {
                        panic!("Invalid state when new guard.");
                    }
                }
            },
            Action::FallsAsleep => {
                match (current_guard, asleep_since) {
                    (Some(id), None) => {
                        asleep_since = Some(log_entry.date_time.minute());
                    },
                    _ => {
                        panic!("Invalid state when falling asleep.");
                    }
                }
            },
            Action::WakesUp => {
                match (current_guard, asleep_since) {
                    (Some(id), Some(asleep)) => {
                        let mut guard = guard_overview.get_or_insert(id);
                        guard.mark_asleep(asleep, log_entry.date_time.minute());
                        asleep_since = None
                    }
                    _ => {
                        panic!("Invalid state when waking up.");
                    }
                }
            }
        }
    }

    guard_overview
}

pub fn strategy_1(guard_overview: &GuardOverview) -> &Guard {
    guard_overview.iter().max_by(|&a, &b| a.get_total_minutes_asleep().cmp(&b.get_total_minutes_asleep())).unwrap()
}

pub fn strategy_2(guard_overview: &GuardOverview) -> &Guard {
    guard_overview.iter().max_by(|&a, &b| {
        let a_max_index = a.find_minute_most_often_asleep();
        let b_max_index = b.find_minute_most_often_asleep();

        a.get_minute(a_max_index).cmp(&b.get_minute(b_max_index))
    }).unwrap()
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

    #[test]
    fn test_processing_log() {
        let mut guard_overview = GuardOverview::new();
        let mut guard_1 = guard_overview.get_or_insert(1);
        guard_1.mark_asleep(10, 50);

        let mut guard_2 = guard_overview.get_or_insert(2);
        guard_2.mark_asleep(35, 40);
        guard_2.mark_asleep(45, 50);

        assert_eq!(
            process_log(&vec![
                LogEntry::new(Utc.ymd(1, 1, 1).and_hms(23, 58, 0), Action::Guard(1)),
                LogEntry::new(Utc.ymd(1, 1, 1).and_hms(0, 10, 0), Action::FallsAsleep),
                LogEntry::new(Utc.ymd(1, 1, 1).and_hms(0, 50, 0), Action::WakesUp),
                LogEntry::new(Utc.ymd(1, 1, 2).and_hms(0, 10, 0), Action::Guard(2)),
                LogEntry::new(Utc.ymd(1, 1, 2).and_hms(0, 35, 0), Action::FallsAsleep),
                LogEntry::new(Utc.ymd(1, 1, 2).and_hms(0, 40, 0), Action::WakesUp),
                LogEntry::new(Utc.ymd(1, 1, 2).and_hms(0, 45, 0), Action::FallsAsleep),
                LogEntry::new(Utc.ymd(1, 1, 2).and_hms(0, 50, 0), Action::WakesUp),]),
            guard_overview);
    }
}
