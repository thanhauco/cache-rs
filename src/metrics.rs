use metrics_exporter_prometheus::PrometheusBuilder;
fn init() {
    PrometheusBuilder::new().install().expect("failed to install recorder");
}