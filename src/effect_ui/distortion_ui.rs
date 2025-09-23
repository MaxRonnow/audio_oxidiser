use crate::effect_ui::param_widget;

use ratatui::{
    Frame,
    layout::{
        Constraint::{Length, Min},
        Layout, Rect,
    },
    style::{Color, Modifier, Style},
    symbols,
    text::{self, Span},
    widgets::{
        Axis, BarChart, Block, Cell, Chart, Dataset, Gauge, LineGauge, List, ListItem, Paragraph,
        Row, Sparkline, Table, Tabs, Wrap,
        canvas::{self, Canvas, Circle, Map, MapResolution, Rectangle},
    },
};

use crate::app::App;

pub fn draw_distortion(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::horizontal([Length(10), Length(16), Min(0)]).split(area);
    let block = Block::bordered().title(Span::styled(
        "Distortion",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    frame.render_widget(block, area);
    let mut knob = param_widget::ParamWidget::new("distortion", 0.5, 0.0, 1.0);
    knob.value = 1.0;
    // let text = Span::from("bruh");
    // frame.render_widget(text, chunks[0]);
    knob.draw_knob(frame, app, chunks[1]);
}

//fn draw_param_slider
