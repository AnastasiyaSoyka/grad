use std::io;
use std::fmt;

use tracing::level_filters::LevelFilter;
use tracing::{Level, Subscriber, Event};
use tracing_subscriber::Layer;
use tracing_subscriber::fmt::{FmtContext, FormattedFields};
use tracing_subscriber::fmt::format::{self, FormatEvent, FormatFields};
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::filter::{self};
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::prelude::*;

/// Marker for [`Format`](tracing_subscriber::fmt::format::Format) that indicates that the bare log format should be used.
///
/// The bare format includes only an event's fields.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
struct Bare;

impl<S, N> FormatEvent<S, N> for Bare
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static
{
    fn format_event(
        &self,
        context: &FmtContext<'_, S, N>,
        mut writer: format::Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        if let Some(scope) = context.event_scope() {
            for span in scope.from_root() {
                let extensions = span.extensions();
                let fields = &extensions.get::<FormattedFields<N>>().unwrap();

                if ! fields.is_empty() {
                    write!(writer, "{}", fields)?;
                }
            }
        }

        context.field_format().format_fields(writer.by_ref(), event)?;

        writeln!(writer)
    }
}

pub fn init_tracing<V>(verbosity: V) -> anyhow::Result<()>
where
    V: Into<LevelFilter> + Copy + Sync + Send + 'static
{
    let pretty = tracing_subscriber::fmt::layer()
        .pretty()
        .with_timer(ChronoLocal::rfc_3339())
        .with_writer(io::stderr)
        .with_filter(filter::filter_fn(move |metadata| verbosity.into() == Level::TRACE && *metadata.level() > Level::INFO));

    let bare = tracing_subscriber::fmt::layer()
        .with_timer(ChronoLocal::rfc_3339())
        .event_format(Bare)
        .with_writer(io::stderr)
        .with_filter(filter::filter_fn(move |metadata| *metadata.level() <= verbosity.into() && *metadata.level() <= Level::DEBUG));

    tracing_subscriber::registry()
        .with(pretty)
        .with(bare)
        .try_init()?;

    Ok(())
}
