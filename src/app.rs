use std::{io, path::PathBuf};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, widgets::Paragraph};

use crate::tab::Tab;

enum Message {
    KeyPressed(KeyEvent),
    Quit,
}

#[derive(Debug)]
enum Mode {
    Normal,
    Insert,
    Command,
}

#[derive(Debug)]
pub struct App {
    path: PathBuf,
    mode: Mode,
    status: AppStatus,
    tab: Tab,
    command: String,
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
            command: String::new(),
        }
    }

    fn view(&self, frame: &mut Frame) {
        frame
            .render_widget(Paragraph::new(format!("{:?}", self)), frame.area());
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::KeyPressed(key_event) => match self.mode {
                Mode::Normal => self.handle_normal_mode_key(key_event),
                Mode::Insert => self.handle_insert_mode_key(key_event),
                Mode::Command => self.handle_command_mode_key(key_event),
            },
            Message::Quit => self.status = AppStatus::Done,
        }

        if !self.command.is_empty() {
            self.mode = Mode::Command;
        }

        if let Mode::Command = self.mode {
            if self.command.is_empty() {
                self.mode = Mode::Normal;
            }
        }
    }

    fn handle_normal_mode_key(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('i') => self.mode = Mode::Insert,
            KeyCode::Char(':') => self.command.push(':'),
            _ => (),
        }
    }

    fn handle_insert_mode_key(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.mode = Mode::Normal,
            _ => (),
        }
    }

    fn handle_command_mode_key(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.command.clear(),
            KeyCode::Backspace => {
                self.command.pop();
            }
            KeyCode::Char(c) => self.command.push(c),
            _ => (),
        }
    }

    fn handle_event() -> io::Result<Option<Message>> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                Ok(Some(Message::KeyPressed(key)))
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
