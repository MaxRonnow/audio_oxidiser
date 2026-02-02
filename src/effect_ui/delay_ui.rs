use crate::effect_ui::param_widget;
use std::sync::{Arc, Mutex, atomic::Ordering};

use ratatui::{
    Frame,
    layout::{
        Constraint::{Length, Min},
        Layout, Rect, Spacing,
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

pub fn draw_delay(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::horizontal([Length(10), Length(16), Length(16), Min(0)]).split(area);
    let block = Block::bordered().title(Span::styled(
        "Distortion",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    frame.render_widget(block, area);
    let mut volume_knob = param_widget::ParamWidget::new("Volume".to_string(), 0.5, 0.0, 1.0);
    let mut distortion_knob = param_widget::ParamWidget::new("Delay".to_string(), 0.5, 0.0, 1.0);

    let selected = app.param_selection.distortion_index;

    match selected {
        0 => volume_knob.selected = true,
        1 => distortion_knob.selected = true,
        _ => {}
    }
    volume_knob.value = app.effect_params.distortion.level.load(Ordering::Relaxed) as f32;
    distortion_knob.value = app
        .effect_params
        .distortion
        .distortion
        .load(Ordering::Relaxed) as f32;

    volume_knob.draw_knob(frame, app, chunks[1]);
    distortion_knob.draw_knob(frame, app, chunks[2]);
}

//fn draw_param_slider
