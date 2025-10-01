use std::{io, path::PathBuf};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Position, Size},
    widgets::Paragraph,
};

use crate::tab::Tab;

enum Message {
    KeyPressed(KeyEvent),
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
    colon_command: String,
    status_message: String,
    terminal_size: Size,
    cursor_position: Position,
    cursor_snapshot: Option<Position>,
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
            colon_command: Default::default(),
            status_message: Default::default(),
            cursor_position: Default::default(),
            terminal_size: Default::default(),
            cursor_snapshot: Default::default(),
        }
    }

    fn view(&self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Min(0), Constraint::Length(1)])
            .split(frame.area());
        frame.render_widget(
            Paragraph::new(match self.mode {
                Mode::Normal => &self.status_message,
                Mode::Insert => "INSERT",
                Mode::Command => &self.colon_command,
            }),
            layout[1],
        );

        frame.set_cursor_position(self.cursor_position);
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::KeyPressed(key_event) => match self.mode {
                Mode::Normal => self.handle_normal_mode_key(key_event),
                Mode::Insert => {
                    self.status_message.clear();
                    self.handle_insert_mode_key(key_event)
                }
                Mode::Command => {
                    self.status_message.clear();
                    self.handle_command_mode_key(key_event)
                }
            },
        }

        if !self.colon_command.is_empty() {
            self.cursor_position.y = self.terminal_size.height;
            self.mode = Mode::Command;
        }

        if let Mode::Command = self.mode {
            if self.colon_command.is_empty() {
                self.cursor_position = self.cursor_snapshot.unwrap_or_default();
                self.mode = Mode::Normal;
            }
        }
    }

    fn handle_normal_mode_key(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('i') => self.mode = Mode::Insert,
            KeyCode::Char(':') => {
                self.cursor_snapshot = Some(self.cursor_position);
                self.cursor_position.x = 1;
                self.colon_command.push(':');
            }
            KeyCode::Char('h') => {
                if self.cursor_position.x > 0 {
                    self.cursor_position.x -= 1;
                }
            }
            KeyCode::Char('j') => {
                if self.cursor_position.y < self.terminal_size.height - 1 {
                    self.cursor_position.y += 1;
                }
            }
            KeyCode::Char('k') => {
                if self.cursor_position.y > 0 {
                    self.cursor_position.y -= 1;
                }
            }
            KeyCode::Char('l') => {
                if self.cursor_position.x < self.terminal_size.width - 1 {
                    self.cursor_position.x += 1;
                }
            }
            KeyCode::Char('0') => self.cursor_position.x = 0,
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
            KeyCode::Esc => self.colon_command.clear(),
            KeyCode::Backspace => {
                self.cursor_position.x -= 1;
                self.colon_command.remove(self.cursor_position.x as usize);
            }
            KeyCode::Char(c) => {
                self.cursor_position.x += 1;
                self.colon_command
                    .insert(self.cursor_position.x as usize - 1, c);
            }
            KeyCode::Left => {
                if self.cursor_position.x > 1 {
                    self.cursor_position.x -= 1;
                }
            }
            KeyCode::Right => {
                if self.cursor_position.x < self.colon_command.len() as u16 {
                    self.cursor_position.x += 1;
                }
            }
            KeyCode::Enter => {
                self.run_command();
                self.colon_command.clear();
            }
            _ => (),
        }
    }

    fn run_command(&mut self) {
        match &self.colon_command[1..] {
            "quit" | "q" => self.status = AppStatus::Done,
            _ => self.status_message = String::from("Not an editor command"),
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
            self.terminal_size = terminal.size()?;

            terminal.draw(|frame| self.view(frame))?;
            let current_message = Self::handle_event()?;
            if current_message.is_some() {
                self.update(current_message.unwrap());
            }
        }
        Ok(())
    }
}
