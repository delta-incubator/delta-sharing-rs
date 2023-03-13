use chrono::SecondsFormat;
use colored::Colorize;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Result as FmtResult;
use tracing::field::Field;
use tracing::field::Visit;
use tracing::Event;
use tracing::Level;
use tracing::Subscriber;
use tracing_log::NormalizeEvent;
use tracing_subscriber::field::RecordFields;
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::FmtContext;
use tracing_subscriber::fmt::FormatEvent;
use tracing_subscriber::fmt::FormatFields;
use tracing_subscriber::fmt::FormattedFields;
use tracing_subscriber::prelude::*;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::EnvFilter;

fn colorize(level: Level, message: String) -> impl Display {
    match level {
        Level::ERROR => message.bright_red(),
        Level::WARN => message.bright_yellow(),
        Level::INFO => message.bright_green(),
        Level::DEBUG => message.bright_blue(),
        Level::TRACE => message.bright_purple(),
    }
}

struct Log;

struct LogVisitor {
    fields: String,
    message: String,
}

impl<'w> FormatFields<'w> for Log {
    fn format_fields<R: RecordFields>(&self, mut writer: Writer<'w>, fields: R) -> FmtResult {
        let mut visitor = LogVisitor {
            fields: String::new(),
            message: String::new(),
        };
        fields.record(&mut visitor);
        write!(writer, "{}", visitor.message.bright_white())?;
        write!(writer, "{}", visitor.fields)?;
        Ok(())
    }
}

impl<C, N> FormatEvent<C, N> for Log
where
    C: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, C, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> FmtResult {
        let normalized = event.normalized_metadata();
        let metadata = normalized.as_ref().unwrap_or_else(|| event.metadata());
        let header = format!(
            "{} | {} [{}]",
            metadata.target(),
            chrono::Local::now().to_rfc3339_opts(SecondsFormat::Millis, false),
            metadata.level(),
        );
        writeln!(writer, "{}", colorize(*metadata.level(), header))?;
        ctx.field_format().format_fields(writer.by_ref(), event)?;
        ctx.visit_spans(|span| {
            let ext = span.extensions();
            let data = ext.get::<FormattedFields<Log>>().unwrap();
            write!(writer, "{}", data)
        })?;
        Ok(())
    }
}

impl Visit for LogVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
        match field.name() {
            "message" => self.message = format!("  {:?}\n", value),
            name => {
                self.fields
                    .push_str(&format!("    {}: {:?}\n", name.cyan(), value));
            }
        };
    }
}

pub fn init(use_json: bool, filter: &str) {
    let filter_layer = EnvFilter::new(filter);
    if use_json {
        tracing_subscriber::registry()
            .with(filter_layer)
            .with(fmt::layer().json().with_file(true).with_line_number(true))
            .init();
    } else {
        let fmt_layer = fmt::layer().event_format(Log).fmt_fields(Log);
        tracing_subscriber::registry()
            .with(filter_layer)
            .with(fmt_layer)
            .init();
    }
}
