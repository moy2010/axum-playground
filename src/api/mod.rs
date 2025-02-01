use std::{sync::Arc, time::Duration};

use axum::{
    response::Response,
    routing::{get, post},
    Router,
};
use route_handlers::{create_user, delete_user, get_user_by_id, update_user};
use tower_http::trace::{self, TraceLayer};
use tracing::{Level, Span};

mod route_handlers;

use crate::services::Services;

#[derive(Copy, Clone, Debug)]
pub enum LatencyUnit {
    /// Use seconds.
    Seconds,
    /// Use milliseconds.
    Millis,
    /// Use microseconds.
    Micros,
    /// Use nanoseconds.
    Nanos,
}
struct Latency {
    unit: LatencyUnit,
    duration: Duration,
}

impl std::fmt::Display for Latency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.unit {
            LatencyUnit::Seconds => write!(f, "{} s", self.duration.as_secs_f64()),
            LatencyUnit::Millis => write!(f, "{} ms", self.duration.as_millis()),
            LatencyUnit::Micros => write!(f, "{} μs", self.duration.as_micros()),
            LatencyUnit::Nanos => write!(f, "{} ns", self.duration.as_nanos()),
        }
    }
}

macro_rules! dyn_event {
    ($lvl:ident, $($arg:tt)+) => {
        match $lvl {
            ::tracing::Level::TRACE => ::tracing::trace!($($arg)+),
            ::tracing::Level::DEBUG => ::tracing::debug!($($arg)+),
            ::tracing::Level::INFO => ::tracing::info!($($arg)+),
            ::tracing::Level::WARN => ::tracing::warn!($($arg)+),
            ::tracing::Level::ERROR => ::tracing::error!($($arg)+),
        }
    };
}

pub fn app(services: Arc<Services>) -> Router {
    Router::new()
        .route("/users", post(create_user))
        .route(
            "/users/:id",
            get(get_user_by_id).patch(update_user).delete(delete_user),
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(|response: &Response, latency: Duration, _span: &Span| {
                    let latency = Latency {
                        unit: LatencyUnit::Millis,
                        duration: latency,
                    };

                    let level = if response.status().as_u16() >= 400 {
                        tracing::Level::ERROR
                    } else {
                        tracing::Level::INFO
                    };

                    dyn_event!(
                        level,
                        %latency,
                        status = response.status().as_u16(),
                        "finished processing request"
                    );
                }),
        )
        .with_state(services)
}
