use crate::plot_bar::plot_bar;
use crate::plot_rose::plot_rose;
use crate::plot_scatter::plot_scatter;
use nu_plugin::{EvaluatedCall, LabeledError, Plugin};
use nu_protocol::{Category, PluginSignature, SyntaxShape, Type, Value};

#[derive(Default)]
pub struct CharmPlot;

// need to prune function inputs to what is actually needed.
impl CharmPlot {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn usage() -> &'static str {
        "Usage: generate Charming plots from Nu inputs"
    }
    pub fn rose_plot(
        &self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
        // labels: Vec<Value>,
        // values: Vec<Value>
    ) {
        debug_print_vals(name, call, input);
    }
    pub fn scatter_plot(
        &self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
        // x: Vec<Value>,
        // y: Vec<Value>
    ) {
        debug_print_vals(name, call, input);
    }
    pub fn bar_plot(
        &self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
        // labels: Vec<Value>,
        // values: Vec<Value>
    ) {
        debug_print_vals(name, call, input);
    }
}

impl Plugin for CharmPlot {
    // TODO
}

fn debug_print_vals(name: &str, call: &EvaluatedCall, input: &Value) {
    println!("name: {0}, call: {1}, input: {2}", name, call, input)
}

// Normally these funcs return Result<Value, LabeledError>
