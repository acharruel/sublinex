use std::env;

use clap::Parser;
use srtlib::{Subtitles, Timestamp};

#[derive(Parser, Debug)]
pub struct Cli {
    /// Input subtitles file
    input: String,

    /// Output subtitles file
    output: String,

    /// First timestamp formatted as hh:mm:ss:ms
    #[arg(short, long)]
    first: String,

    /// Last timestamp formatted as hh:mm:ss:ms
    #[arg(short, long)]
    last: String,
}

trait TimestampExt {
    fn to_ms(&self) -> u64;
    fn from_ms(ms: u64) -> Timestamp;
}

impl TimestampExt for Timestamp {
    fn to_ms(&self) -> u64 {
        let (hours, minutes, seconds, milliseconds) = self.get();
        hours as u64 * 3600 * 1000
            + minutes as u64 * 60 * 1000
            + seconds as u64 * 1000
            + milliseconds as u64
    }

    fn from_ms(ms: u64) -> Timestamp {
        let hours = (ms / 3600000) as u8;
        let minutes = ((ms % 3600000) / 60000) as u8;
        let seconds = ((ms % 60000) / 1000) as u8;
        let milliseconds = (ms % 1000) as u16;
        Timestamp::new(hours, minutes, seconds, milliseconds)
    }
}

fn ts_arg_to_ms(ts: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let parts: Vec<&str> = ts.split(':').collect();
    if parts.len() != 4 {
        return Err("Invalid timestamp format".into());
    }
    let hours = parts[0].parse::<u8>()?;
    let minutes = parts[1].parse::<u8>()?;
    let seconds = parts[2].parse::<u8>()?;
    let milliseconds = parts[3].parse::<u16>()?;
    Ok(Timestamp::new(hours, minutes, seconds, milliseconds).to_ms())
}

fn apply_offset(subs: &mut Subtitles, offset: i32) {
    subs.into_iter()
        .for_each(|sub| sub.add_milliseconds(offset));
}

fn apply_ratio(subs: &mut Subtitles, ratio_num: u64, ratio_den: u64) {
    subs.into_iter().for_each(|sub| {
        sub.start_time = Timestamp::from_ms(sub.start_time.to_ms() * ratio_num / ratio_den);
        sub.end_time = Timestamp::from_ms(sub.end_time.to_ms() * ratio_num / ratio_den);
    });
}

pub fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{}: extrapoling subtiles '{}' -> '{}'",
        env::args().next().unwrap_or_default(),
        cli.input,
        cli.output
    );

    let mut subs = Subtitles::parse_from_file(&cli.input, None)?;
    let duration = subs[subs.len() - 1].start_time.to_ms() - subs[0].start_time.to_ms();
    let first = subs[0].start_time.to_ms() as i32;

    let target_first = ts_arg_to_ms(&cli.first)? as i32;
    let target_last = ts_arg_to_ms(&cli.last)? as i32;

    println!("Applying offset: {}ms", target_first - first);
    println!(
        "Applying ratio: {}",
        (target_last - target_first) as f64 / duration as f64
    );

    apply_offset(&mut subs, target_first - first);
    apply_ratio(&mut subs, (target_last - target_first) as u64, duration);

    println!("Writing to file: {}", cli.output);
    subs.write_to_file(&cli.output, None)?;

    Ok(())
}

#[cfg(test)]
mod test {
    use srtlib::Subtitle;

    use super::*;

    #[test]
    fn test_timestamp_to_ms() {
        let ts = Timestamp::new(0, 0, 0, 0);
        assert_eq!(ts.to_ms(), 0);
        let ts = Timestamp::new(0, 0, 0, 1);
        assert_eq!(ts.to_ms(), 1);
        let ts = Timestamp::new(0, 0, 1, 0);
        assert_eq!(ts.to_ms(), 1000);
        let ts = Timestamp::new(0, 1, 0, 0);
        assert_eq!(ts.to_ms(), 60000);
        let ts = Timestamp::new(1, 0, 0, 0);
        assert_eq!(ts.to_ms(), 3600000);
        let ts = Timestamp::new(1, 1, 1, 1);
        assert_eq!(ts.to_ms(), 3661001);
    }

    #[test]
    fn test_ts_arg_to_ms() {
        assert_eq!(ts_arg_to_ms("00:00:00:000").unwrap(), 0);
        assert_eq!(ts_arg_to_ms("00:00:00:001").unwrap(), 1);
        assert_eq!(ts_arg_to_ms("00:00:01:000").unwrap(), 1000);
        assert_eq!(ts_arg_to_ms("00:01:00:000").unwrap(), 60000);
        assert_eq!(ts_arg_to_ms("01:00:00:000").unwrap(), 3600000);
        assert_eq!(ts_arg_to_ms("01:01:01:001").unwrap(), 3661001);
    }

    #[test]
    fn test_apply_offset() {
        let mut subs = Subtitles::new_from_vec(vec![
            Subtitle::new(
                0,
                Timestamp::new(0, 0, 0, 0),
                Timestamp::new(0, 0, 0, 500),
                String::new(),
            ),
            Subtitle::new(
                1,
                Timestamp::new(0, 0, 1, 0),
                Timestamp::new(0, 0, 1, 500),
                String::new(),
            ),
            Subtitle::new(
                2,
                Timestamp::new(0, 1, 2, 0),
                Timestamp::new(0, 1, 2, 500),
                String::new(),
            ),
            Subtitle::new(
                3,
                Timestamp::new(0, 1, 3, 0),
                Timestamp::new(0, 1, 3, 500),
                String::new(),
            ),
            Subtitle::new(
                4,
                Timestamp::new(2, 3, 4, 0),
                Timestamp::new(2, 3, 4, 500),
                String::new(),
            ),
        ]);
        apply_offset(&mut subs, 1000);
        assert_eq!(subs[0].start_time.to_ms(), 1000);
        assert_eq!(subs[0].end_time.to_ms(), 1500);
        assert_eq!(subs[1].start_time.to_ms(), 2000);
        assert_eq!(subs[1].end_time.to_ms(), 2500);
        assert_eq!(subs[2].start_time.to_ms(), 63000);
        assert_eq!(subs[2].end_time.to_ms(), 63500);
        assert_eq!(subs[3].start_time.to_ms(), 64000);
        assert_eq!(subs[3].end_time.to_ms(), 64500);
        assert_eq!(subs[4].start_time.to_ms(), 7385000);
        assert_eq!(subs[4].end_time.to_ms(), 7385500);
    }

    #[test]
    fn test_apply_ratio() {
        let mut subs = Subtitles::new_from_vec(vec![
            Subtitle::new(
                0,
                Timestamp::new(0, 0, 0, 0),
                Timestamp::new(0, 0, 0, 500),
                String::new(),
            ),
            Subtitle::new(
                1,
                Timestamp::new(0, 0, 1, 0),
                Timestamp::new(0, 0, 1, 500),
                String::new(),
            ),
            Subtitle::new(
                2,
                Timestamp::new(0, 1, 2, 0),
                Timestamp::new(0, 1, 2, 500),
                String::new(),
            ),
            Subtitle::new(
                3,
                Timestamp::new(0, 1, 3, 0),
                Timestamp::new(0, 1, 3, 500),
                String::new(),
            ),
            Subtitle::new(
                4,
                Timestamp::new(2, 3, 4, 0),
                Timestamp::new(2, 3, 4, 500),
                String::new(),
            ),
        ]);
        apply_ratio(&mut subs, 2, 1);
        assert_eq!(subs[0].start_time.to_ms(), 0);
        assert_eq!(subs[0].end_time.to_ms(), 1000);
        assert_eq!(subs[1].start_time.to_ms(), 2000);
        assert_eq!(subs[1].end_time.to_ms(), 3000);
        assert_eq!(subs[2].start_time.to_ms(), 6000);
        assert_eq!(subs[2].end_time.to_ms(), 7000);
        assert_eq!(subs[3].start_time.to_ms(), 8000);
        assert_eq!(subs[3].end_time.to_ms(), 9000);
        assert_eq!(subs[4].start_time.to_ms(), 18000);
        assert_eq!(subs[4].end_time.to_ms(), 19000);
    }
}
