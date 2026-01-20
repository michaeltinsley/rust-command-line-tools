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

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = Args::parse();
    let adjusted_time = adjust_time(SystemTime::now(), args.seconds)?;
    let output = format_timestamp(adjusted_time, args.format)?;
    println!("{}", output);
    Ok(())
}

/// Adjust a SystemTime by adding or subtracting seconds
fn adjust_time(time: SystemTime, seconds: i64) -> Result<SystemTime> {
    if seconds >= 0 {
        Ok(time + std::time::Duration::from_secs(seconds as u64))
    } else {
        time.checked_sub(std::time::Duration::from_secs((-seconds) as u64))
            .ok_or_else(|| "Resulting timestamp would be before UNIX epoch".into())
    }
}

/// Format a SystemTime according to the specified format
fn format_timestamp(time: SystemTime, format: TimestampFormat) -> Result<String> {
    match format {
        TimestampFormat::Unix => {
            let duration = time.duration_since(UNIX_EPOCH)?;
            Ok(duration.as_secs().to_string())
        }
        TimestampFormat::Rfc3339 => {
            let datetime: DateTime<Utc> = time.into();
            Ok(datetime.to_rfc3339())
        }
        TimestampFormat::Default => {
            let datetime: DateTime<Utc> = time.into();
            Ok(datetime.format("%Y-%m-%d %H:%M:%S").to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjust_time_positive() {
        let result = adjust_time(UNIX_EPOCH, 3600).unwrap();
        let duration = result.duration_since(UNIX_EPOCH).unwrap();
        assert_eq!(duration.as_secs(), 3600);
    }

    #[test]
    fn test_adjust_time_negative() {
        let time = UNIX_EPOCH + std::time::Duration::from_secs(7200);
        let result = adjust_time(time, -3600).unwrap();
        let duration = result.duration_since(UNIX_EPOCH).unwrap();
        assert_eq!(duration.as_secs(), 3600);
    }

    #[test]
    fn test_format_unix() {
        let time = UNIX_EPOCH + std::time::Duration::from_secs(1704067200);
        let result = format_timestamp(time, TimestampFormat::Unix).unwrap();
        assert_eq!(result, "1704067200");
    }

    #[test]
    fn test_format_default() {
        let time = UNIX_EPOCH + std::time::Duration::from_secs(1704067200);
        let result = format_timestamp(time, TimestampFormat::Default).unwrap();
        assert_eq!(result, "2024-01-01 00:00:00");
    }

    #[test]
    fn test_format_rfc3339() {
        let time = UNIX_EPOCH + std::time::Duration::from_secs(1704067200);
        let result = format_timestamp(time, TimestampFormat::Rfc3339).unwrap();
        assert!(result.starts_with("2024-01-01T00:00:00"));
    }

    #[test]
    fn test_systemtime_to_datetime_conversion() {
        let time = UNIX_EPOCH + std::time::Duration::from_secs(1704067200);
        let datetime: DateTime<Utc> = time.into();
        assert_eq!(
            datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
            "2024-01-01 00:00:00"
        );
    }
}
