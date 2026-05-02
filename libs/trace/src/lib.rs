// Tracing setup inspired by batleforc/proxyauthK8S
// https://github.com/batleforc/ProxyAuthK8S/blob/main/libs/server/trace/src/lib.rs

use std::sync::OnceLock;

#[cfg(feature = "otel")]
use opentelemetry::trace::TracerProvider;
use opentelemetry::{global, KeyValue};
#[cfg(feature = "metrics")]
use opentelemetry_otlp::MetricExporter;
use opentelemetry_otlp::SpanExporter;
#[cfg(feature = "metrics")]
use opentelemetry_sdk::metrics::SdkMeterProvider;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::trace::{RandomIdGenerator, Sampler};
use opentelemetry_sdk::{trace::SdkTracerProvider, Resource};
use tracing_subscriber::Registry;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Layer};

/// Context describing the running service for tracing purposes.
#[derive(Clone, Debug)]
pub struct Context {
    pub pod_name: String,
    pub service_name: String,
}

/// Holds the providers created during tracing initialisation so they can be
/// shut down cleanly at process exit.
pub struct TracingOutput {
    pub tracer_provider: SdkTracerProvider,
    #[cfg(feature = "metrics")]
    pub meter_provider: SdkMeterProvider,
}

fn get_resource(ctx: &Context) -> Resource {
    static RESOURCE: OnceLock<Resource> = OnceLock::new();
    RESOURCE
        .get_or_init(|| {
            Resource::builder()
                .with_service_name(ctx.service_name.clone())
                .with_attributes(vec![
                    KeyValue::new("service.pod", ctx.pod_name.clone()),
                    KeyValue::new("service.version", env!("CARGO_PKG_VERSION").to_string()),
                ])
                .build()
        })
        .clone()
}

fn init_traces(ctx: &Context) -> SdkTracerProvider {
    let exporter = SpanExporter::builder()
        .with_tonic()
        .build()
        .expect("Failed to create span exporter");
    SdkTracerProvider::builder()
        .with_resource(get_resource(ctx))
        .with_sampler(Sampler::AlwaysOn)
        .with_id_generator(RandomIdGenerator::default())
        .with_max_events_per_span(64)
        .with_max_attributes_per_span(16)
        .with_batch_exporter(exporter)
        .build()
}

#[cfg(feature = "metrics")]
fn init_metrics(ctx: &Context) -> SdkMeterProvider {
    let exporter = MetricExporter::builder()
        .with_tonic()
        .build()
        .expect("Failed to create metric exporter");

    SdkMeterProvider::builder()
        .with_periodic_exporter(exporter)
        .with_resource(get_resource(ctx))
        .build()
}

/// Initialise tracing with optional OpenTelemetry export and console output.
///
/// Call this once at the beginning of `main`, before starting any async tasks.
/// Store the returned [`TracingOutput`] and pass it to [`shutdown_tracing`]
/// before the process exits.
pub fn start_tracing(ctx: &Context) -> TracingOutput {
    global::set_text_map_propagator(TraceContextPropagator::new());

    let tracer_provider = init_traces(ctx);
    global::set_tracer_provider(tracer_provider.clone());

    #[cfg(feature = "metrics")]
    let meter_provider = init_metrics(ctx);
    #[cfg(feature = "metrics")]
    global::set_meter_provider(meter_provider.clone());

    #[cfg(feature = "otel")]
    let tracer = tracer_provider.tracer(ctx.service_name.clone());

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"))
        .add_directive("hyper=off".parse().unwrap())
        .add_directive("tonic=off".parse().unwrap())
        .add_directive("h2=off".parse().unwrap());

    let filter_fmt = EnvFilter::new("info").add_directive("opentelemetry=info".parse().unwrap());

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_thread_names(true)
        .with_filter(filter_fmt);

    #[cfg(feature = "otel")]
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    #[cfg(feature = "otel")]
    let subscriber = Registry::default()
        .with(env_filter)
        .with(telemetry)
        .with(fmt_layer);

    #[cfg(not(feature = "otel"))]
    let subscriber = Registry::default().with(env_filter).with(fmt_layer);

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to install `tracing` subscriber.");

    TracingOutput {
        tracer_provider,
        #[cfg(feature = "metrics")]
        meter_provider,
    }
}

/// Flush and shut down all tracing providers.
///
/// Returns an error string listing every provider that failed to shut down.
pub fn shutdown_tracing(tracing_output: TracingOutput) -> Result<(), String> {
    let mut errors = Vec::new();

    if let Err(e) = tracing_output.tracer_provider.shutdown() {
        errors.push(format!("tracer provider: {e}"));
    }

    #[cfg(feature = "metrics")]
    if let Err(e) = tracing_output.meter_provider.shutdown() {
        errors.push(format!("meter provider: {e}"));
    }

    if !errors.is_empty() {
        return Err(format!(
            "Failed to shutdown providers:\n{}",
            errors.join("\n")
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_is_cloneable() {
        let ctx = Context {
            pod_name: "test-pod".to_string(),
            service_name: "test-service".to_string(),
        };
        let cloned = ctx.clone();
        assert_eq!(cloned.pod_name, "test-pod");
        assert_eq!(cloned.service_name, "test-service");
    }

    #[test]
    fn context_debug() {
        let ctx = Context {
            pod_name: "pod-1".to_string(),
            service_name: "svc".to_string(),
        };
        let s = format!("{ctx:?}");
        assert!(s.contains("pod-1"));
        assert!(s.contains("svc"));
    }
}
