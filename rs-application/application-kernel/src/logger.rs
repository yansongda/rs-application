use crate::config::G_CONFIG;
use tracing::level_filters::LevelFilter;
use tracing::{Event, Subscriber};
use tracing_appender::non_blocking::{NonBlockingBuilder, WorkerGuard};
use tracing_subscriber::filter;
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields, format};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::{LookupSpan, Scope};
use tracing_subscriber::util::SubscriberInitExt;

const MAX_LOG_LENGTH: usize = 1024;

/// Truncates a string to MAX_LOG_LENGTH bytes, adding "..." if truncated
pub fn truncate_for_log(s: &str) -> String {
    if s.len() <= MAX_LOG_LENGTH {
        s.to_string()
    } else {
        // Find a valid UTF-8 boundary at or before MAX_LOG_LENGTH
        let mut end = MAX_LOG_LENGTH;
        while end > 0 && !s.is_char_boundary(end) {
            end -= 1;
        }
        let mut result = String::with_capacity(end + 3);
        result.push_str(&s[..end]);
        result.push_str("...");
        result
    }
}

pub struct Logger {
    _guard: WorkerGuard,
}

impl Logger {
    pub fn blocking(identifier: &str) {
        tracing_subscriber::registry()
            .with(Self::get_filter_target(identifier))
            .with(tracing_subscriber::fmt::layer().event_format(TracingFormatter))
            .init();
    }

    pub fn non_blocking(identifier: &str) -> Self {
        let (non_blocking, guard) = NonBlockingBuilder::default().finish(std::io::stdout());

        tracing_subscriber::registry()
            .with(Self::get_filter_target(identifier))
            .with(
                tracing_subscriber::fmt::layer()
                    .event_format(TracingFormatter)
                    .with_writer(non_blocking),
            )
            .init();

        Logger { _guard: guard }
    }

    fn get_filter_target(identifier: &str) -> filter::Targets {
        if let Some(bin) = G_CONFIG.bin.get(identifier)
            && bin.debug
        {
            return filter::Targets::new().with_default(LevelFilter::DEBUG);
        }

        filter::Targets::new().with_default(LevelFilter::INFO)
    }
}

pub struct TracingId(pub String);

#[derive(Debug, Clone)]
struct TracingFormatter;

impl<S, N> FormatEvent<S, N> for TracingFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: format::Writer<'_>,
        event: &Event<'_>,
    ) -> std::fmt::Result {
        write!(
            &mut writer,
            "{}|{}|",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.6f"),
            event.metadata().level()
        )?;

        for span in ctx.event_scope().into_iter().flat_map(Scope::from_root) {
            if let Some(request_id) = span.extensions().get::<TracingId>() {
                write!(writer, "{}|", request_id.0)?;
                break;
            }
        }

        ctx.format_fields(writer.by_ref(), event)?;

        writeln!(writer)
    }
}
