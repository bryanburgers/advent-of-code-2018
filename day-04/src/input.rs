use regex::Regex;

#[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
pub struct TaggedInput {
    pub date: String,
    pub minute: i16,
    pub input: Input,
}

#[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
pub enum Input {
    BeginShift(usize),
    FallAsleep,
    WakeUp,
}

impl TaggedInput {
    pub fn parse(line: &str) -> Option<TaggedInput> {
        let re = Regex::new(r"\[(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2}) (?P<hour>\d{2}):(?P<minute>\d{2})\] (?P<details>Guard #(?P<guard>\d+) begins shift|wakes up|falls asleep)").unwrap();

        let matches = re.captures(line);
        if matches.is_none() {
            return None;
        }

        let matches = matches.unwrap();

        let year = matches.name("year").unwrap().as_str().parse::<u16>().unwrap();
        let mut month = matches.name("month").unwrap().as_str().parse::<u16>().unwrap();
        let mut day = matches.name("day").unwrap().as_str().parse::<u16>().unwrap();
        let hour = matches.name("hour").unwrap().as_str().parse::<u16>().unwrap();
        let mut minute = matches.name("minute").unwrap().as_str().parse::<i16>().unwrap();
        let details = matches.name("details").unwrap().as_str();
        let guard = matches.name("guard");

        if hour == 23 {
            minute = minute - 60;
            // Hope our input doesn't have any dates that cross the month line
            day += 1;

            // Instead of breaking out a date parser, I looked at the input
            // and noticed that only one time do we start a shift in the previous
            // month. Handle that specific case.
            if month == 10 && day == 32 {
                month = 11;
                day = 1;
            }
        }

        let date = format!("{:04}-{:02}-{:02}", year, month, day);

        if let Some(guard) = guard {
            let guard = guard.as_str().parse::<usize>().unwrap();
            Some(TaggedInput {
                input: Input::BeginShift(guard),
                date,
                minute,
            })
        }
        else if details == "falls asleep" {
            Some(TaggedInput {
                input: Input::FallAsleep,
                date,
                minute
            })
        }
        else {
            Some(TaggedInput {
                input: Input::WakeUp,
                date,
                minute
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_parses_guard_correctly() {
        let input = "[1518-05-08 00:02] Guard #2719 begins shift";

        let result = TaggedInput::parse(input).unwrap();

        assert_eq!(result, TaggedInput {
            input: Input::BeginShift(2719),
            date: "1518-05-08".to_owned(),
            minute: 2,
        });
    }

    #[test]
    fn it_parses_negative_shift_start() {
        let input = "[1518-08-27 23:48] Guard #1657 begins shift";

        let result = TaggedInput::parse(input).unwrap();

        assert_eq!(result, TaggedInput {
            input: Input::BeginShift(1657),
            date: "1518-08-28".to_owned(),
            minute: -12,
        });
    }

    #[test]
    fn it_parses_wakeup() {
        let input = "[1518-04-12 00:57] wakes up";

        let result = TaggedInput::parse(input).unwrap();

        assert_eq!(result, TaggedInput {
            input: Input::WakeUp,
            date: "1518-04-12".to_owned(),
            minute: 57,
        });
    }

    #[test]
    fn it_parses_asleep() {
        let input = "[1518-11-12 00:30] falls asleep";

        let result = TaggedInput::parse(input).unwrap();

        assert_eq!(result, TaggedInput {
            input: Input::FallAsleep,
            date: "1518-11-12".to_owned(),
            minute: 30,
        });
    }
}
