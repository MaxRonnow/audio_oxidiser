use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{self, Span},
    widgets::{
        canvas::{self, Canvas, Circle, Map, MapResolution, Rectangle},
        Axis, BarChart, Block, Cell, Chart, Dataset, Gauge, LineGauge, List, ListItem, Paragraph,
        Row, Sparkline, Table, Tabs, Wrap,
    },
    Frame,
};
use crate::{app::App, effect_ui::distortion_ui::draw_distortion, effect_ui::delay_ui::draw_delay};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(frame.area());
    let tabs = app
        .tabs
        .titles
        .iter()
        .map(|t| text::Line::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect::<Tabs>()
        .block(Block::bordered().title("Audio Oxidiser"))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.index);
    frame.render_widget(tabs, chunks[0]);
    match app.tabs.index {
        0 => draw_distortion(frame, app, chunks[1]),
        1 => draw_delay(frame, app, chunks[1]),
        // 2 => draw_third_tab(frame, app, chunks[1]),
        _ => {}
    };
}

