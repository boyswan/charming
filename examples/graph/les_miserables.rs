use echarts::basic::label;
use echarts::basic::line_style;
use echarts::component::legend;
use echarts::component::title;
use echarts::component::tooltip;
use echarts::series::graph;
use echarts::series::Series;
use echarts::Chart;

fn main() {
    let mut data: graph::Data = serde_json::from_str(include_str!("les-miserables.json")).unwrap();
    for d in data.nodes.iter_mut() {
        if d.symbol_size > 30.0 {
            d.label = Some(graph::NodeLabel::new().show(true));
        }
    }
    let legend: Vec<String> = data.categories.iter().map(|c| c.name.clone()).collect();
    let chart = Chart::new()
        .title(
            title::Title::new()
                .text("Les Miserables")
                .subtext("Circular layout")
                .top("bottom")
                .left("right"),
        )
        .legend(legend::Legend::new().data(legend))
        .tooltip(tooltip::Tooltip::new())
        .series(Series::Graph(
            graph::Graph::new()
                .name("Les Miserables")
                .layout(graph::Layout::Circular)
                .circular(graph::Circular::new().rotate_label(true))
                .roam(true)
                .label(
                    label::Label::new()
                        .position(label::Position::Right)
                        .formatter("{b}"),
                )
                .line_style(line_style::LineStyle::new().color("source").curveness(0.3))
                .data(data),
        ));
    println!("{}", chart.to_string());
}