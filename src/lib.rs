use srtlib::{Subtitles, Timestamp};

pub struct Config {
    input_file: String,
    output_file: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let input_file = args[1].clone();
        let output_file = args[2].clone();
        Ok(Config {
            input_file,
            output_file,
        })
    }
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

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    println!("Input file: {}", config.input_file);
    println!("Output file: {}", config.output_file);

    let subs = Subtitles::parse_from_file(&config.input_file, None)?;

    for s in subs {
        println!(
            "ts: {} --> {} ms",
            s.start_time, s.start_time.to_ms(),
        );
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
