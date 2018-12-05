use super::input::*;

#[derive(Debug)]
pub struct SleepRange {
    pub guard: usize,
    pub date: String,
    pub start_minute: usize,
    pub end_minute: usize,
}

impl SleepRange {
    pub fn get_ranges(sorted: Vec<TaggedInput>) -> Vec<SleepRange> {
        let initial_state = FoldState {
            ranges: Vec::new(),
            date: "".to_owned(),
            guard: 0,
            state: SleepState::Awake,
        };

        let final_state = sorted.iter().fold(initial_state, |mut state, item| {
            let date = item.date.to_owned();
            let minute = item.minute;
            let input = &item.input;

            state.state = match (input, &state.state) {
                (Input::BeginShift(guard), _) => {
                    state.date = date;
                    state.guard = *guard;
                    SleepState::Awake
                },
                (Input::FallAsleep, _) => {
                    SleepState::SleepingSince(minute as usize)
                },
                (Input::WakeUp, SleepState::SleepingSince(since)) => {
                    state.ranges.push(SleepRange {
                        guard: state.guard,
                        date: state.date.clone(),
                        start_minute: *since,
                        end_minute: minute as usize,
                    });
                    SleepState::Awake
                },
                _ => {
                    unreachable!();
                },
            };

            state
        });

        final_state.ranges
    }
}

struct FoldState {
    ranges: Vec<SleepRange>,
    date: String,
    guard: usize,
    state: SleepState
}

enum SleepState {
    Awake,
    SleepingSince(usize),
}
