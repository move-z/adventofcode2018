use std::fs;
use std::str::FromStr;

use regex::Captures;

pub fn read_file(day: &str) -> String {
    let path = format!("input/{}.txt", day);
    fs::read_to_string(path).unwrap()
}

pub fn parse_capture<F>(cap: &Captures, idx: usize, name: &str) -> Result<F, String>
where
    F: FromStr,
    <F as std::str::FromStr>::Err: std::fmt::Debug,
{
    let ma = cap.get(idx).ok_or(format!("{} not found", name))?.as_str();
    ma.parse::<F>()
        .map_err(|e| format!("failed to parse {}, {:?}", name, e))
}
