use chrono::{DateTime, Utc};
use clap::{Parser, ValueEnum};
use std::time::{SystemTime, UNIX_EPOCH};

/// A command line tool that generates timestamps based on the current time
#[derive(Parser, Debug)]
#[command(name = "timestamp")]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of seconds to add to the current time
    #[arg(default_value_t = 0)]
    seconds: i64,

    /// Format of the timestamp
    #[arg(short, long, default_value_t = TimestampFormat::Default)]
    format: TimestampFormat,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum TimestampFormat {
    /// YYYY-MM-DD HH:MM:SS format
    Default,
    /// RFC 3339 / ISO 8601 format
    Rfc3339,
    /// Unix timestamp (seconds since epoch)
    Unix,
}

impl std::fmt::Display for TimestampFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimestampFormat::Default => write!(f, "default"),
            TimestampFormat::Rfc3339 => write!(f, "rfc3339"),
            TimestampFormat::Unix => write!(f, "unix"),
        }
    }
}

fn main() {
    let args = Args::parse();

    // Get the current time using std::time
    let now = SystemTime::now();

    // Add or subtract the specified number of seconds directly
    let adjusted_time = if args.seconds >= 0 {
        now + std::time::Duration::from_secs(args.seconds as u64)
    } else {
        match now.checked_sub(std::time::Duration::from_secs((-args.seconds) as u64)) {
            Some(time) => time,
            None => {
                eprintln!("Error: Resulting timestamp would be before UNIX epoch");
                std::process::exit(1);
            }
        }
    };

    // Format and print the timestamp based on the requested format
    match args.format {
        TimestampFormat::Unix => {
            let duration = adjusted_time.duration_since(UNIX_EPOCH).unwrap();
            println!("{}", duration.as_secs());
        }
        TimestampFormat::Rfc3339 | TimestampFormat::Default => {
            // Convert SystemTime to chrono DateTime only for formatting
            let datetime: DateTime<Utc> = adjusted_time.into();

            match args.format {
                TimestampFormat::Rfc3339 => {
                    println!("{}", datetime.to_rfc3339());
                }
                TimestampFormat::Default => {
                    println!("{}", datetime.format("%Y-%m-%d %H:%M:%S"));
                }
                _ => unreachable!(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration_arithmetic() {
        // Test that we can add seconds using std::time::Duration
        let epoch = UNIX_EPOCH;
        let one_hour_later = epoch + std::time::Duration::from_secs(3600);

        let duration = one_hour_later.duration_since(UNIX_EPOCH).unwrap();
        assert_eq!(duration.as_secs(), 3600);
    }

    #[test]
    fn test_systemtime_to_datetime_conversion() {
        // Test that SystemTime converts correctly to chrono DateTime
        let time = UNIX_EPOCH + std::time::Duration::from_secs(1704067200);
        let datetime: DateTime<Utc> = time.into();

        assert_eq!(
            datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
            "2024-01-01 00:00:00"
        );
    }

    #[test]
    fn test_epoch_formatting() {
        let datetime: DateTime<Utc> = UNIX_EPOCH.into();
        assert_eq!(
            datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
            "1970-01-01 00:00:00"
        );
    }

    #[test]
    fn test_rfc3339_formatting() {
        let time = UNIX_EPOCH + std::time::Duration::from_secs(1704067200);
        let datetime: DateTime<Utc> = time.into();
        let rfc3339 = datetime.to_rfc3339();
        assert!(rfc3339.starts_with("2024-01-01T00:00:00"));
    }
}
