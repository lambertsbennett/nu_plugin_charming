use crate::utils::{show_plot, PlotConfig};
use charming::{
    component::Legend,
    element::ItemStyle,
    series::{Pie, PieRoseType},
    Chart, HtmlRenderer,
};
use nu_engine::CallExt;
use nu_plugin::{EvaluatedCall, LabeledError, Plugin};
use nu_protocol::{PluginSignature, Spanned, Value};
use num_traits::Num;
use std::iter::zip;

pub fn plot_rose(
    _name: &str,
    call: &EvaluatedCall,
    input: &Value,
    labels: Vec<Value>,
    values: Vec<Value>,
    config: Option<PlotConfig>,
) -> Result<Value, LabeledError> {
    let cfg = config.unwrap_or(PlotConfig::new_default_config());
    // need to change this!
    let labels: Vec<Value> = call.req(engine_state, stack, 0)?;
    let values: Vec<Value> = call.req(engine_state, stack, 1)?;
    // need to change this!

    let label_values: Vec<String> = labels.iter().map(|x| x.as_string().unwrap()).collect();
    let values_values: Vec<f64> = values.iter().map(|x| x.as_float().unwrap()).collect();

    create_plot(label_values, values_values, cfg)?
}

fn create_plot<T: Num>(
    labels: Vec<String>,
    values: Vec<T>,
    config: PlotConfig,
) -> Result<Value, LabeledError> {
    let data = zip(values, labels).collect();
    let chart = Chart::new()
        .legend(Legend::new().top(config.legend_loc))
        .series(
            Pie::new()
                .name(config.title)
                .rose_type(PieRoseType::Radius)
                .radius(vec!["50", "250"])
                .center(vec!["50%", "50%"])
                .item_style(ItemStyle::new().border_radius(8))
                .data(data),
        );

    let renderer = HtmlRenderer::new("chart", 1000, 800);
    html_str = renderer.render(&chart).map_err(|e| LabeledError {
        span: Some(call.head),
        msg: e.to_string(),
        label: "problem rendering html".to_string(),
    })?;
    show_plot(html_str);
    Ok(Value::string(html_str, call.head))
}
