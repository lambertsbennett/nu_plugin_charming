use crate::plot_rose::plot_rose;

#[derive(Default)]
pub struct CharmPlot;

// need to prune function inputs to what is actually needed.
impl CharmPlot {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn usage() -> &'static str {
        "Usage: generate charming plots from Nu inputs"
    }
    pub fn rose_plot(
        &self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
        labels: Vec<Value>,
        values: Vec<Value>
        config: Option<&PlotConfig>,
    ) -> Result<Value, LabeledError> {
        plot_rose(name, call, input, labels, values, config)
    }
}
