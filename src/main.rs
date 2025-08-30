use termtab::app::State;

fn main() {
    let mut terminal = ratatui::init();
    let mut state = State::default();
    let app_result = state.run(&mut terminal);
    ratatui::restore();
    app_result
}
