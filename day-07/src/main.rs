use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{Read, self};

const EXTRA_TIME: u8 = 60;
const WORKERS: usize = 5;

fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer)
        .expect("Read stdin");
    let buffer = buffer.trim();

    let input : Vec<Requirement> = buffer.split("\n")
        .map(|i| i.trim())
        .map(|s| Requirement::parse(s).unwrap())
        .collect();

    let mut steps = HashMap::new();

    for requirement in &input {
        steps.entry(requirement.prereq).or_insert_with(|| Step::new(requirement.prereq));
        let step = steps.entry(requirement.step).or_insert_with(|| Step::new(requirement.step));
        step.add_prereq(requirement.prereq);
    }

    while steps.len() > 0 {
        let mut available_steps: Vec<&str> = steps.iter()
            .filter(|(_, val)| !val.has_prereqs())
            .map(|(key, _)| *key)
            .collect();
        available_steps.sort();

        let chosen_step = available_steps[0];

        print!("{}", chosen_step);
        steps.remove(chosen_step);
        for other_step in steps.values_mut() {
            other_step.remove_prereq(chosen_step);
        }
    }
    println!();

    let mut steps = HashMap::new();
    for requirement in &input {
        steps.entry(requirement.prereq).or_insert_with(|| Step::new(requirement.prereq));
        let step = steps.entry(requirement.step).or_insert_with(|| Step::new(requirement.step));
        step.add_prereq(requirement.prereq);
    }

    let mut completed = HashSet::new();
    let mut workers = vec![Worker::Idle; WORKERS];
    let mut second = 0;
    while steps.len() > 0 || workers.iter().filter(|i| match i { Worker::Idle => false, _ => true }).count() > 0 {
        // 1. Determine if any workers are done.
        for i in 0..workers.len() {
            let worker = &workers[i];
            let worker = match worker {
                Worker::Working { step, remaining: 0 } => {
                    completed.insert(*step);
                    Worker::Idle
                },
                Worker::Working { step, remaining } => Worker::Working { step, remaining: remaining - 1 },
                Worker::Idle => Worker::Idle,
            };
            workers[i] = worker;
        }

        // 2. Fill all open workers.
        let mut available_steps: Vec<&str> = steps.iter()
            .filter(|(_, val)| val.has_prereqs_complete(&completed))
            .map(|(key, _)| *key)
            .collect(); 

        available_steps.sort();

        for i in 0..workers.len() {
            let worker = &workers[i];
            let worker = match worker {
                Worker::Idle => {
                    if available_steps.len() > 0 {
                        let step = available_steps.remove(0);
                        steps.remove(step);
                        let remaining = step_time(step) + EXTRA_TIME - 1;
                        Worker::Working { step, remaining }
                    }
                    else {
                        Worker::Idle
                    }
                },
                Worker::Working { step, remaining } => Worker::Working { step, remaining: *remaining },
            };
            workers[i] = worker;
        }

        second += 1;
    }
    println!("{}", second - 1);
}

#[derive(Debug)]
struct Step<'a> {
    name: &'a str,
    prereqs: HashSet<&'a str>,
}


impl<'a> Step<'a> {
    fn new(name: &'a str) -> Step<'a> {
        Step {
            name,
            prereqs: HashSet::new(),
        }
    }

    fn add_prereq(&mut self, prereq: &'a str) {
        self.prereqs.insert(prereq);
    }

    fn remove_prereq(&mut self, prereq: &'a str) {
        self.prereqs.remove(prereq);
    }

    fn has_prereqs(&self) -> bool {
        self.prereqs.len() > 0
    }

    fn has_prereqs_complete(&self, completed: &HashSet<&str>) -> bool {
        self.prereqs.difference(&completed).count() == 0
    }
}

fn step_time(t: &str) -> u8 {
    let first_char = t.bytes().next().unwrap();
    first_char - b'A' + 1
}

#[derive(Debug)]
struct Requirement<'a> {
    prereq: &'a str,
    step: &'a str,
}

impl<'a> Requirement<'a> {
    fn parse(string: &'a str) -> Option<Requirement<'a>> {
        let re = Regex::new(r"Step (?P<prereq>.*) must be finished before step (?P<step>.*) can begin\.").unwrap();
        let captures = re.captures(string)?;
        let prereq = captures.name("prereq")?.as_str();
        let step = captures.name("step")?.as_str();
        Some(Requirement {
            prereq,
            step,
        })
    }
}

#[derive(Debug, Clone)]
enum Worker<'a> {
    Idle,
    Working { step: &'a str, remaining: u8 },
}
