//! Custom logging implementation for Obsidium
//!
//! This module provides a beautifully formatted logger with colored output,
//! custom time formatting, and structured logging capabilities.

use std::fmt;
use tracing_subscriber::fmt::FmtContext;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::registry::LookupSpan;

/// ANSI color codes for terminal output
mod colors {
    pub const RESET: &str = "\x1b[0m";
    pub const GRAY: &str = "\x1b[90m";
    pub const RED: &str = "\x1b[31m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const GREEN: &str = "\x1b[32m";
    pub const BLUE: &str = "\x1b[34m";
    pub const MAGENTA: &str = "\x1b[35m";
}

/// Formats the current time as HH:MM:SS.mmm
fn format_current_time() -> String {
    let now =
        time::OffsetDateTime::now_utc().to_offset(time::UtcOffset::from_hms(3, 0, 0).unwrap());

    format!(
        "{:02}:{:02}:{:02}.{:03}",
        now.hour(),
        now.minute(),
        now.second(),
        now.millisecond()
    )
}

/// Custom time formatter for tracing subscriber
struct CustomTimeFormat;

impl FormatTime for CustomTimeFormat {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> fmt::Result {
        write!(w, "{}", format_current_time())
    }
}

/// Custom event formatter that provides colored, structured log output with timestamps
struct CustomFormatWithTime;

/// Custom event formatter that provides colored, structured log output without timestamps
struct CustomFormat;

/// Returns the appropriate color and formatted level string for a log level
fn format_level(level: &tracing::Level) -> String {
    match *level {
        tracing::Level::ERROR => format!("{}[ERROR]{}", colors::RED, colors::RESET),
        tracing::Level::WARN => format!("{}[WARN]{}", colors::YELLOW, colors::RESET),
        tracing::Level::INFO => format!("{}[INFO]{}", colors::GREEN, colors::RESET),
        tracing::Level::DEBUG => format!("{}[DEBUG]{}", colors::BLUE, colors::RESET),
        tracing::Level::TRACE => format!("{}[TRACE]{}", colors::MAGENTA, colors::RESET),
    }
}

impl<S, N> tracing_subscriber::fmt::FormatEvent<S, N> for CustomFormatWithTime
where
    S: tracing::Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> tracing_subscriber::fmt::FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> fmt::Result {
        let timestamp = format_current_time();

        // Write gray timestamp
        write!(writer, "{}{timestamp}{} ", colors::GRAY, colors::RESET)?;

        // Write colored log level with consistent spacing
        let level_formatted = format_level(event.metadata().level());
        write!(writer, "{level_formatted} ")?;

        // Write the actual log message and fields
        ctx.field_format().format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}

impl<S, N> tracing_subscriber::fmt::FormatEvent<S, N> for CustomFormat
where
    S: tracing::Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> tracing_subscriber::fmt::FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> fmt::Result {
        // Write colored log level
        let level_formatted = format_level(event.metadata().level());
        write!(writer, "{level_formatted} ")?;

        // Write the actual log message and fields
        ctx.field_format().format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}

/// Initialize the logging system with custom formatting
///
/// This function sets up a beautiful, colored logger with optional time formatting.
/// It respects the `RUST_LOG` environment variable for filtering, falling back to "info" level.
/// It also respects the `RUST_LOG_TIME` environment variable to enable timestamps.
/// Set `RUST_LOG_TIME=1` or `RUST_LOG_TIME=true` to enable timestamps in logs.
///
/// # Examples
///
/// ```bash
/// # Default format without timestamps
/// RUST_LOG=debug ./Obsidium
///
/// # Enable timestamps
/// RUST_LOG=debug RUST_LOG_TIME=1 ./Obsidium
/// ```
pub fn init() {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    // Check if timestamps should be enabled via environment variable
    let enable_time = std::env::var("RUST_LOG_TIME")
        .map(|v| v == "1" || v.to_lowercase() == "true")
        .unwrap_or(false);

    if enable_time {
        tracing_subscriber::fmt()
            .with_timer(CustomTimeFormat)
            .with_target(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_file(false)
            .with_line_number(false)
            .with_level(true)
            .with_ansi(true)
            .with_env_filter(env_filter)
            .fmt_fields(tracing_subscriber::fmt::format::DefaultFields::new())
            .event_format(CustomFormatWithTime)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_timer(CustomTimeFormat)
            .with_target(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_file(false)
            .with_line_number(false)
            .with_level(true)
            .with_ansi(true)
            .with_env_filter(env_filter)
            .fmt_fields(tracing_subscriber::fmt::format::DefaultFields::new())
            .event_format(CustomFormat)
            .init();
    }
}
