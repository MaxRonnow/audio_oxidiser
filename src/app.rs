use crate::EffectParams;
use crate::ui;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
use std::io;
use std::sync::{Arc, Mutex, atomic::AtomicBool};

pub fn init_ui(running: Arc<AtomicBool>, ui_params: Arc<EffectParams>) -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new(running, ui_params);
    let app_result = app.run(&mut terminal);
    disable_raw_mode()?;
    ratatui::restore();
    app_result
}

#[derive(Debug, Default)]
pub struct App<'a> {
    counter: u8,
    exit: bool,
    running: Arc<AtomicBool>,
    pub tabs: TabsState<'a>,
    ui_params: Arc<EffectParams>,
}

impl<'a> App<'a> {
    pub fn new(running: Arc<AtomicBool>, ui_params: Arc<EffectParams>) -> Self {
        App {
            tabs: TabsState::new(vec!["Distorion", "Delay", "Reverb"]),
            running,
            exit: false,
            counter: 0,
            ui_params,
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        enable_raw_mode()?;
        while !self.exit {
            terminal.draw(|frame| ui::draw(frame, self))?;
            self.handle_events()?;
        }
        Ok(())
    }

    /// updates the application's state based on user input
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
        self.running
            .store(false, std::sync::atomic::Ordering::SeqCst);
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
        self.tabs.next();
    }

    fn decrement_counter(&mut self) {
        self.counter -= 1;
        self.tabs.previous();
    }
}

//impl Widget for &App {
//    fn render(self, area: Rect, buf: &mut Buffer) {
//        let title = Line::from(" Counter App Tutorial ".bold());
//        let instructions = Line::from(vec![
//            " Decrement ".into(),
//            "<Left>".blue().bold(),
//            " Increment ".into(),
//            "<Right>".blue().bold(),
//            " Quit ".into(),
//            "<Q> ".blue().bold(),
//        ]);
//        let block = Block::bordered()
//            .title(title.centered())
//            .title_bottom(instructions.centered())
//            .border_set(border::THICK);
//
//        let counter_text = Text::from(vec![Line::from(vec![
//            "Value: ".into(),
//            self.counter.to_string().yellow(),
//        ])]);
//
//        Paragraph::new(counter_text)
//            .centered()
//            .block(block)
//            .render(area, buf);
//    }
//}

#[derive(Debug, Default)]
pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub const fn new(titles: Vec<&'a str>) -> Self {
        Self { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}
