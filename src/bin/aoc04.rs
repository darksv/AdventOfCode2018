use std::collections::HashMap;
use chrono::NaiveDateTime;
use chrono::Timelike;

fn main() {
    let input = std::fs::read_to_string("inputs/input04.txt").unwrap();

    let mut entries = parse_input(input);
    entries.sort_by_key(|entry| entry.datetime);

    let table_by_guard = generate_tables(&entries);

    let (guard, minute) = find_most_sleepy_guard_ever(&table_by_guard).unwrap();
    println!("Part 1 = {}", minute as u32 * guard as u32);

    let (guard, minute) = find_most_predictable_sleeper(&table_by_guard).unwrap();
    println!("Part 2 = {}", guard as u32 * minute as u32);
}

fn find_most_predictable_sleeper(table_by_guard: &HashMap<u32, [u32; 60]>) -> Option<(u32, usize)> {
    let (guard, minute, _times) = table_by_guard
        .iter()
        .filter_map(|(guard, table)| {
            let (minute, times) = max_with_index(table)?;
            Some((*guard, minute, *times))
        })
        .max_by_key(|(_, _, times)| *times)?;
    Some((guard, minute))
}

fn find_most_sleepy_guard_ever(table_by_guard: &HashMap<u32, [u32; 60]>) -> Option<(u32, usize)> {
    let guard = table_by_guard
        .iter()
        .max_by_key(|(_, minutes)| minutes.iter().sum::<u32>())
        .map(|(guard, _)| *guard)?;
    let most_sleepy_minute = max_with_index(&table_by_guard[&guard])?.0;
    Some((guard, most_sleepy_minute))
}

fn generate_tables(events: &[Event]) -> HashMap<u32, [u32; 60]> {
    let mut table_by_guard = HashMap::new();
    let mut guard = None;
    let mut sleep_from = None;
    for entry in events {
        match entry.event_type {
            EventType::BeginShift => {
                guard = entry.guard;
            }
            EventType::FallAsleep => {
                sleep_from = Some(entry.datetime);
            }
            EventType::WakeUp => {
                let guard = guard.unwrap();
                let start = sleep_from.unwrap().minute();
                let end = entry.datetime.minute();

                let table = table_by_guard
                    .entry(guard)
                    .or_insert([0; 60]);

                for minutes in start..end {
                    table[minutes as usize] += 1;
                }

                sleep_from = None;
            }
        }
    }
    table_by_guard
}

fn max_with_index(items: &[u32]) -> Option<(usize, &u32)> {
    items
        .iter()
        .enumerate()
        .max_by_key(|(_, value)| **value)
}

fn parse_input(input: String) -> Vec<Event> {
    const DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M";

    let re = regex::Regex::new(r"Guard #(\d+) begins shift").unwrap();

    input.lines().filter_map(|line| {
        let datetime = NaiveDateTime::parse_from_str(&line[1..17], DATETIME_FORMAT).ok()?;
        let (event_type, guard) = match &line[19..] {
            "falls asleep" => (EventType::FallAsleep, None),
            "wakes up" => (EventType::WakeUp, None),
            other => {
                let id = re.captures(other)?[1].parse().ok()?;
                (EventType::BeginShift, Some(id))
            }
        };

        Some(Event {
            datetime,
            guard,
            event_type,
        })
    }).collect()
}

#[derive(Debug)]
struct Event {
    datetime: NaiveDateTime,
    guard: Option<u32>,
    event_type: EventType,
}

#[derive(Debug)]
enum EventType {
    BeginShift,
    FallAsleep,
    WakeUp,
}
