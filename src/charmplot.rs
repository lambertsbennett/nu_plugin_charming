use nu_plugin::{EvaluatedCall, LabeledError, Plugin};
use nu_protocol::{Category, PluginSignature, SyntaxShape, Value};

pub struct CharmPlotter;

// need to prune function inputs to what is actually needed.
// impl CharmPlotter {
//     pub fn new() -> Self {
//         Self {}
//     }
//     pub fn rose_plot(&self, name: &str, call: &EvaluatedCall, input: &Value) {
//         debug_print_vals(name, call, input);
//     }
// }

impl Plugin for CharmPlotter {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![
            // plot rose
            PluginSignature::build("plot rose")
                .usage("Create a rose plot with the provided values and labels.")
                .named("labels", SyntaxShape::String, "The category labels.", None)
                .named(
                    "values",
                    SyntaxShape::Number,
                    "The values to plot for each category.",
                    None,
                )
                .category(Category::Experimental),
        ]
    }
    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        debug_print_vals(name, call, input)
    }
}

fn debug_print_vals(
    name: &str,
    call: &EvaluatedCall,
    input: &Value,
) -> Result<Value, LabeledError> {
    println!("name: {0}", name);
    println!("call: {:?}", call);
    println!("input: {:?}", input);
    Ok(Value::Nothing {
        internal_span: call.head,
    })
}
