use std::{io, path::PathBuf};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, widgets::Paragraph};

use crate::tab::Tab;

enum Message {
    ChangeMode(Mode),
    Quit,
}

#[derive(Debug)]
enum Mode {
    Normal,
    Insert,
}

#[derive(Debug)]
pub struct App {
    path: PathBuf,
    mode: Mode,
    status: AppStatus,
    tab: Tab,
}

#[derive(Debug)]
enum AppStatus {
    Running,
    Done,
}

impl App {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            mode: Mode::Normal,
            status: AppStatus::Running,
            tab: Tab::new(),
        }
    }

    fn view(&self, frame: &mut Frame) {
        frame
            .render_widget(Paragraph::new(format!("{:?}", self)), frame.area());
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ChangeMode(mode) => self.mode = mode,
            Message::Quit => self.status = AppStatus::Done,
        }
    }

    fn handle_key_press(key_event: KeyEvent) -> Option<Message> {
        match key_event.code {
            KeyCode::Char('i') => Some(Message::ChangeMode(Mode::Insert)),
            KeyCode::Esc => Some(Message::ChangeMode(Mode::Normal)),
            KeyCode::Char('q') => Some(Message::Quit),
            _ => None,
        }
    }

    fn handle_event() -> io::Result<Option<Message>> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                Ok(Self::handle_key_press(key))
            }
            _ => Ok(None),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while let AppStatus::Running = self.status {
            terminal.draw(|frame| self.view(frame))?;
            let current_message = Self::handle_event()?;
            if current_message.is_some() {
                self.update(current_message.unwrap());
            }
        }
        Ok(())
    }
}
