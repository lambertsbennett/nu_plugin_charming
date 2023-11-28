mod charmplot;
use charmplot::CharmPlotter;
use nu_plugin::{serve_plugin, JsonSerializer};

fn main() {
    serve_plugin(&mut CharmPlotter {}, JsonSerializer {})
}
