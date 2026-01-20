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

    // Get the current time
    let now = SystemTime::now();

    // Calculate duration since UNIX epoch
    let duration = match now.duration_since(UNIX_EPOCH) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error: System time appears to be before UNIX epoch: {}", e);
            std::process::exit(1);
        }
    };

    // Add the specified number of seconds
    let total_seconds = duration.as_secs() as i64 + args.seconds;

    if total_seconds < 0 {
        eprintln!("Error: Resulting timestamp would be negative (before UNIX epoch)");
        std::process::exit(1);
    }

    // Convert back to SystemTime
    let adjusted_time = UNIX_EPOCH + std::time::Duration::from_secs(total_seconds as u64);

    // Format and print the timestamp based on the requested format
    match args.format {
        TimestampFormat::Rfc3339 => match format_rfc3339(adjusted_time) {
            Ok(formatted) => println!("{}", formatted),
            Err(e) => {
                eprintln!("Error formatting timestamp: {}", e);
                std::process::exit(1);
            }
        },
        TimestampFormat::Unix => {
            println!("{}", total_seconds);
        }
        TimestampFormat::Default => match format_default(adjusted_time) {
            Ok(formatted) => println!("{}", formatted),
            Err(e) => {
                eprintln!("Error formatting timestamp: {}", e);
                std::process::exit(1);
            }
        },
    }
}

/// Format timestamp in the default format: YYYY-MM-DD HH:MM:SS
fn format_default(time: SystemTime) -> Result<String, String> {
    let duration = time
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("Time calculation error: {}", e))?;

    let total_seconds = duration.as_secs();
    let (year, month, day, hour, minute, second) = seconds_to_datetime(total_seconds);

    Ok(format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        year, month, day, hour, minute, second
    ))
}

/// Format timestamp in RFC 3339 format
fn format_rfc3339(time: SystemTime) -> Result<String, String> {
    let duration = time
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("Time calculation error: {}", e))?;

    let total_seconds = duration.as_secs();
    let (year, month, day, hour, minute, second) = seconds_to_datetime(total_seconds);

    Ok(format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        year, month, day, hour, minute, second
    ))
}

/// Convert seconds since UNIX epoch to date/time components
fn seconds_to_datetime(total_seconds: u64) -> (i32, u8, u8, u8, u8, u8) {
    let seconds_per_minute = 60;
    let seconds_per_hour = 3600;
    let seconds_per_day = 86400;

    // Calculate time components
    let second = (total_seconds % seconds_per_minute) as u8;
    let total_minutes = total_seconds / seconds_per_minute;
    let minute = (total_minutes % 60) as u8;
    let total_hours = total_seconds / seconds_per_hour;
    let hour = (total_hours % 24) as u8;

    // Calculate days since epoch (1970-01-01)
    let mut days = total_seconds / seconds_per_day;

    // Calculate year
    let mut year = 1970;
    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if days >= days_in_year {
            days -= days_in_year;
            year += 1;
        } else {
            break;
        }
    }

    // Calculate month and day
    let days_in_months = [
        31,                                       // January
        if is_leap_year(year) { 29 } else { 28 }, // February
        31,                                       // March
        30,                                       // April
        31,                                       // May
        30,                                       // June
        31,                                       // July
        31,                                       // August
        30,                                       // September
        31,                                       // October
        30,                                       // November
        31,                                       // December
    ];

    let mut month = 1;
    for &days_in_month in &days_in_months {
        if days >= days_in_month as u64 {
            days -= days_in_month as u64;
            month += 1;
        } else {
            break;
        }
    }

    let day = (days + 1) as u8; // Days are 1-indexed

    (year, month, day, hour, minute, second)
}

/// Check if a year is a leap year
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_leap_year() {
        assert!(is_leap_year(2000));
        assert!(is_leap_year(2004));
        assert!(!is_leap_year(1900));
        assert!(!is_leap_year(2001));
    }

    #[test]
    fn test_seconds_to_datetime_epoch() {
        let (year, month, day, hour, minute, second) = seconds_to_datetime(0);
        assert_eq!(year, 1970);
        assert_eq!(month, 1);
        assert_eq!(day, 1);
        assert_eq!(hour, 0);
        assert_eq!(minute, 0);
        assert_eq!(second, 0);
    }

    #[test]
    fn test_seconds_to_datetime_basic() {
        // 2024-01-01 00:00:00 is approximately 1704067200 seconds since epoch
        let (year, month, day, hour, minute, second) = seconds_to_datetime(1704067200);
        assert_eq!(year, 2024);
        assert_eq!(month, 1);
        assert_eq!(day, 1);
        assert_eq!(hour, 0);
        assert_eq!(minute, 0);
        assert_eq!(second, 0);
    }
}
