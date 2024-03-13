use clap::Parser;
use srtlib::{Subtitles, Timestamp};

#[derive(Parser, Debug)]
pub struct Cli {
    /// Input subtitles file
    input: String,

    /// Output subtitles file
    output: String,

    /// First timestamp
    #[arg(short, long)]
    first: String,

    /// Last timestamp
    #[arg(short, long)]
    last: String,
}

trait TimestampExt {
    fn to_ms(&self) -> u64;
}

impl TimestampExt for Timestamp {
    fn to_ms(&self) -> u64 {
        let (hours, minutes, seconds, milliseconds) = self.get();
        hours as u64 * 3600 * 1000
            + minutes as u64 * 60 * 1000
            + seconds as u64 * 1000
            + milliseconds as u64
    }
}

pub fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    println!("Input file: {}", cli.input);
    println!("Output file: {}", cli.output);

    let subs = Subtitles::parse_from_file(&cli.input, None)?;

    for s in subs {
        println!("ts: {} --> {} ms", s.start_time, s.start_time.to_ms(),);
    }

    Ok(())
}

#[cfg(test)]
mod test {
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
}
