use echarts::{
    component::{Axis, Grid, Legend, SelectedMode, Tooltip},
    element::{AxisLabel, AxisLine, AxisTick, MarkLine, SplitLine},
    series::{pictorial_bar, Series},
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .tooltip(Tooltip::new())
        .legend(
            Legend::new()
                .selected_mode(SelectedMode::Single)
                .data(vec!["typeA", "typeB"]),
        )
        .x_axis(
            Axis::new()
                .axis_tick(AxisTick::new().show(false))
                .axis_line(AxisLine::new().show(false))
                .axis_label(AxisLabel::new().show(false))
                .data(vec!["a", "b", "c", "d", "e"]),
        )
        .y_axis(
            Axis::new()
                .max(150)
                .offset(20)
                .split_line(SplitLine::new().show(false)),
        )
        .grid(Grid::new().top("center").height("230"))
        .mark_line(MarkLine::new().z(-100))
        .series(Series::PictorialBar(
            pictorial_bar::PictorialBar::new().name("typeA"),
        ))
}
