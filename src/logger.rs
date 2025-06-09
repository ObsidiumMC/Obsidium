use std::fmt;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::fmt::FmtContext;
use tracing_subscriber::registry::LookupSpan;

struct CustomTimeFormat;

impl FormatTime for CustomTimeFormat {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> fmt::Result {
        let now = time::OffsetDateTime::now_utc();
        write!(
            w,
            "{:02}:{:02}:{:02}.{:03}",
            now.hour(),
            now.minute(),
            now.second(),
            now.millisecond()
        )
    }
}

struct CustomFormat;

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
        let now = time::OffsetDateTime::now_utc();

        // Write gray timestamp
        write!(
            writer,
            "\x1b[90m{:02}:{:02}:{:02}.{:03}\x1b[0m ",
            now.hour(),
            now.minute(),
            now.second(),
            now.millisecond()
        )?;

        // Write colored log level with brackets (padded for alignment)
        let level = event.metadata().level();
        match *level {
            tracing::Level::ERROR => write!(writer, "\x1b[31m[ERROR]\x1b[0m ")?,
            tracing::Level::WARN => write!(writer, "\x1b[33m[WARN]\x1b[0m ")?,
            tracing::Level::INFO => write!(writer, "\x1b[32m[INFO]\x1b[0m ")?,
            tracing::Level::DEBUG => write!(writer, "\x1b[34m[DEBUG]\x1b[0m ")?,
            tracing::Level::TRACE => write!(writer, "\x1b[35m[TRACE]\x1b[0m ")?,
        }

        // No additional space needed here since we handle it in the match above
        ctx.field_format().format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}

/// Initialize the logging system with custom formatting
pub fn init_logger() {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

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
