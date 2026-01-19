use crate::app::{App, ParamSelection};
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
        block::title,
        canvas::{self, Canvas, Circle, Line, Map, MapResolution, Rectangle},
    },
};

pub struct ParamWidget {
    min: f32,
    max: f32,
    pub value: f32,
    knob: Circle,
    line: Line,
    selected: bool,
    name: String,
}

impl ParamWidget {
    pub fn new(name: String, value: f32, min: f32, max: f32) -> Self {
        ParamWidget {
            min,
            max,
            value,
            name,
            knob: Circle {
                x: 0.0,
                y: 0.0,
                radius: 5.0,
                color: Color::White,
            },
            line: Line {
                x1: 0.0,
                y1: 0.0,
                x2: 1.0,
                y2: 1.0,
                color: Color::White,
            },
            selected: false,
        }
    }

    pub fn draw_knob(&mut self, frame: &mut Frame, app: &mut App, area: Rect) {
        let angle = self.get_rad(self.value);
        self.line.x2 = 4.0 * f32::cos(angle) as f64;
        self.line.y2 = 4.0 * f32::sin(angle) as f64;

        let chunks =
            Layout::vertical([Length(2), Length(1), Length(8), Length(1), Min(0)]).split(area);

        let knob = Canvas::default()
            //.block(Block::bordered().title("knob"))
            .marker(symbols::Marker::Braille)
            .paint(|ctx| {
                ctx.draw(&self.knob);
                ctx.draw(&self.line);
            })
            .x_bounds([-6.0, 6.0])
            .y_bounds([-6.0, 6.0]);

        let mut title = Span::styled(&self.name, Style::default()).into_centered_line();
        if self.selected {
            title = Span::styled(
                &self.name,
                Style::default().add_modifier(Modifier::REVERSED),
            )
            .into_centered_line();
        }

        let value = Span::from("value").into_centered_line();
        frame.render_widget(title, chunks[1]);
        frame.render_widget(knob, chunks[2]);
        frame.render_widget(value, chunks[3]);
    }

    fn get_rad(&self, value: f32) -> f32 {
        let normalized_value = (value - self.min) / (self.max - self.min);
        if normalized_value < 0.8333333333333333 {
            -4.7123889803847 * normalized_value + 3.9269908169872
        } else {
            -4.7123889803847 * normalized_value + 10.2101761241668
        }
    }
}
