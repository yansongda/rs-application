use std::str::FromStr;

use crate::config::G_CONFIG;
use tracing::level_filters::LevelFilter;
use tracing::{Event, Subscriber};
use tracing_appender::non_blocking::{NonBlockingBuilder, WorkerGuard};
use tracing_subscriber::filter;
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields, FormattedFields, format};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::{LookupSpan, Scope};
use tracing_subscriber::util::SubscriberInitExt;

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
        let level = if G_CONFIG.bin.get(identifier).unwrap().debug {
            "debug"
        } else {
            "info"
        };

        filter::Targets::new().with_default(LevelFilter::from_str(level).unwrap())
    }
}

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
            if let Some(fields) = span.extensions().get::<FormattedFields<N>>() {
                if !fields.is_empty() {
                    let c = &fields.fields;

                    if c.starts_with("request_id") {
                        write!(writer, "{}|", &c[12..c.len() - 1])?;
                        break;
                    }
                }
            }
        }

        ctx.format_fields(writer.by_ref(), event)?;

        writeln!(writer)
    }
}
