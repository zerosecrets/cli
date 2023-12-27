use crate::common::print_formatted_error::print_formatted_error;
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use termimad::crossterm::style::{StyledContent, Stylize};

/// # Date Formatting Module
///
/// Provides a utility for formatting date strings into human-readable relative time strings.
/// The module attempts to handle two common date string formats, RFC3339 and a custom data like string from hasura
/// format specific to this application. By using this module, date strings can be transformed
/// into more user-friendly representations indicating the time that has passed since a given date.
///
/// # Utility Function
///
/// * `format_relative_time`: This function is the primary utility provided by the module,
/// designed to format a date string into its relative time string representation.
/// Formats a provided date string into its relative time representation.
///
/// The function endeavors to interpret the provided date string, considering both RFC 3339
/// and the custom format. After successful interpretation, it calculates the time duration
/// between the given date and the current moment. The result is a string suggesting how much
/// time has passed since the input date (e.g., "used 3 days ago", "used just now").
///
/// # Arguments
///
/// * `date_str` - The date string to format, adhering to either RFC 3339 or the custom
/// "YYYY-MM-DD HH:MM:SS.SSSSSS UTC" format.
///
/// # Returns
///
/// A `Result<String, chrono::ParseError>` where a successful result (`Ok`) provides the
/// relative time string, while an error (`Err`) indicates issues with date string parsing.
///
/// # Examples
///
/// ```
/// let date_str = "2023-09-29T14:10:14.449216+00:00";
/// let relative_str = format_relative_time(&date_str);
/// assert_eq!(relative_str.unwrap(), "used just now");
/// ```
pub fn format_relative_time(date_str: &str) -> Result<StyledContent<String>, chrono::ParseError> {
    let date = match DateTime::parse_from_rfc3339(date_str)
        .map(|dt| dt.naive_utc())
        .or_else(|_| {
            let format = "%Y-%m-%d %H:%M:%S%.6f UTC";
            NaiveDateTime::parse_from_str(date_str, format)
        }) {
        Ok(formatted_data) => formatted_data,
        Err(_) => {
            print_formatted_error("Service error. Please try again.");
            std::process::exit(1);
        }
    };

    let now = Utc::now().naive_utc();
    let duration = now.signed_duration_since(date);

    if duration.num_seconds() < 60 {
        return Ok("just now".to_string().green());
    }

    if duration.num_minutes() < 60 {
        return Ok(format!(
            "{} minute{} ago",
            duration.num_minutes(),
            if duration.num_minutes() == 1 { "" } else { "s" }
        )
        .green());
    }

    if duration.num_hours() < 24 {
        return Ok(format!(
            "{} hour{} ago",
            duration.num_hours(),
            if duration.num_hours() == 1 { "" } else { "s" }
        )
        .green());
    }

    if duration.num_days() <= 7 {
        return Ok(format!(
            "{} day{} ago",
            duration.num_days(),
            if duration.num_days() == 1 { "" } else { "s" }
        )
        .green());
    }

    if duration.num_days() < 30 {
        return Ok(format!(
            "{} day{} ago",
            duration.num_days(),
            if duration.num_days() == 1 { "" } else { "s" }
        )
        .yellow());
    }

    if duration < Duration::days(365) {
        let months = duration.num_days() / 30;
        return Ok(format!("{} month{} ago", months, if months == 1 { "" } else { "s" }).red());
    }

    let years = duration.num_days() / 365;

    Ok(format!("{} year{} ago", years, if years == 1 { "" } else { "s" }).red())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relative_time_format() {
        let now = Utc::now().to_rfc3339();
        assert_eq!(
            format_relative_time(&now).unwrap(),
            "just now".to_string().green()
        );

        let an_hour_ago = (Utc::now() - Duration::hours(1)).to_rfc3339();

        assert_eq!(
            format_relative_time(&an_hour_ago).unwrap(),
            "1 hour ago".to_string().green()
        );

        let three_days_ago = (Utc::now() - Duration::days(3)).to_rfc3339();

        assert_eq!(
            format_relative_time(&three_days_ago).unwrap(),
            "3 days ago".to_string().green()
        );
    }
}
