use ratatui::widgets::StatefulWidget;

pub struct TabState {
    selected_string: u8,
}

#[derive(Debug)]
pub struct Tab {
    measures: Vec<Measure>,
}

impl StatefulWidget for Tab {
    type State = TabState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        todo!()
    }
}

impl Tab {
    pub fn new() -> Self {
        Self {
            measures: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct Measure {
    time_signature: TimeSignature,
    contents: Vec<RhythmValue>,
}

impl Measure {
    fn new(time_signature: TimeSignature) -> Self {
        Self {
            time_signature,
            contents: Vec::new(),
        }
    }

    fn set_time_signature(&mut self, time_signature: TimeSignature) {
        self.time_signature = time_signature;
    }

    fn clear_content(&mut self) {
        self.contents.clear();
    }
}

#[derive(Debug)]
struct Rest {
    duration: Duration,
}

impl Rest {
    fn new(duration: Duration) -> Self {
        Self { duration }
    }
}

#[derive(Debug)]
struct Note {
    string: u8,
    duration: Duration,
    fret: u8,
    slide_in: Option<Direction>,
    slide_out: Option<Direction>,
    tap: bool,
    tie: bool,
}

impl Note {
    fn new(string: u8, duration: Duration) -> Self {
        Self {
            string,
            duration,
            fret: Default::default(),
            slide_in: Default::default(),
            slide_out: Default::default(),
            tap: Default::default(),
            tie: Default::default(),
        }
    }

    fn fret(mut self, fret: u8) -> Self {
        self.fret = fret;
        self
    }

    fn slide_in(mut self, direction: Direction) -> Self {
        self.slide_in = Some(direction);
        self
    }

    fn slide_out(mut self, direction: Direction) -> Self {
        self.slide_out = Some(direction);
        self
    }

    fn tap(mut self, tap: bool) -> Self {
        self.tap = tap;
        self
    }

    fn tie(mut self, tie: bool) -> Self {
        self.tie = tie;
        self
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

#[derive(Debug)]
enum RhythmValue {
    Notes(Vec<Note>),
    Rest(Rest),
}

impl RhythmValue {
    fn add_note(&mut self, note: Note) {
        match self {
            RhythmValue::Notes(notes) => notes.push(note),
            RhythmValue::Rest(_) => *self = Self::Notes(vec![note]),
        }
    }
}

#[derive(Debug)]
struct TimeSignature {
    numerator: u8,
    denominator: u8,
}

impl TimeSignature {
    fn new(numerator: u8, denominator: u8) -> Result<Self, &'static str> {
        if numerator == 0 {
            return Err("Numerator cannot be zero");
        }
        if denominator == 0 {
            return Err("Denominator cannot be zero");
        }
        if !denominator.is_power_of_two() {
            return Err("Denominator must be a power of two");
        }
        Ok(Self {
            numerator,
            denominator,
        })
    }
}

#[derive(Debug)]
struct Duration {
    numerator: u8,
    denominator: u8,
}

impl Duration {
    fn new(numerator: u8, denominator: u8) -> Result<Self, &'static str> {
        if numerator == 0 {
            return Err("Numerator cannot be zero");
        }
        if denominator == 0 {
            return Err("Denominator cannot be zero");
        }
        Ok(Self {
            numerator,
            denominator,
        })
    }
}
