use nu_plugin::{serve_plugin, JsonSerializer};
use nu_plugin_charming::CharmPlotter;

fn main() {
    serve_plugin(&mut CharmPlotter {}, JsonSerializer {})
}
