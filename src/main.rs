use nu_plugin::{serve_plugin, JsonSerializer};
use nu_plugin_charming::CharmPlot;

fn main() {
    serve_plugin(&mut CharmPlot {}, JsonSerializer {})
}
