use std::path::PathBuf;

use ratatui::DefaultTerminal;

#[derive(Default)]
enum Status {
    #[default]
    Running,
    Done,
}

#[derive(Default)]
pub struct State {
    file_path: PathBuf,
    status: Status,
}

impl State {
    fn update(&mut self, msg: Message) {
        todo!()
    }

    fn view(&self) {
        todo!()
    }

    pub fn run(&self, terminal: &mut DefaultTerminal) {
        while let Status::Running = self.status {}
    }
}

enum Message {}
