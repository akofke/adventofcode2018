extern crate chrono;
extern crate regex;

use self::chrono::prelude::*;
use self::regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
enum ReportType {
    BeginsShift(u32),
    FallsAsleep,
    WakesUp,
}

#[derive(Debug)]
struct GuardReport {
    timestamp: NaiveDateTime,
    report_type: ReportType,
}

fn parse_logs(input: &str) -> Vec<GuardReport> {
    let re = Regex::new(r"[\[](?P<date>.*)[\]] ((?:Guard #(?P<id>\d+).*)|(?P<other>.*))").unwrap();
    return input
        .lines()
        .map(|line| {
            let caps = re.captures(line).expect("regex");
            let timestamp = NaiveDateTime::parse_from_str(
                caps.name("date").expect("date").as_str(),
                "%Y-%m-%d %H:%M",
            )
            .expect("dateparse");
            let report_type = caps.name("id").map_or_else(
                || match caps.name("other").expect("other").as_str() {
                    "falls asleep" => ReportType::FallsAsleep,
                    "wakes up" => ReportType::WakesUp,
                    _ => unreachable!(),
                },
                |id| ReportType::BeginsShift(id.as_str().parse::<u32>().unwrap()),
            );
            GuardReport {
                timestamp,
                report_type,
            }
        })
        .collect();
}

fn collect_sleep_schedule(mut reports: Vec<GuardReport>) -> HashMap<u32, Vec<i32>> {
    reports.sort_by_key(|report| report.timestamp);
    //    println!("{:#?}", reports);

    let mut guard_sleeps = HashMap::new();
    let mut current_guard = 0;
    let mut sleep_start_minute = 0usize;

    for report in reports {
        match report.report_type {
            ReportType::BeginsShift(id) => current_guard = id,
            ReportType::FallsAsleep => sleep_start_minute = report.timestamp.minute() as usize,
            ReportType::WakesUp => {
                let wake_minute = report.timestamp.minute() as usize;
                let sleep_times = guard_sleeps
                    .entry(current_guard)
                    .or_insert_with(|| vec![0; 60]);
                for minute in &mut sleep_times[sleep_start_minute..wake_minute] {
                    *minute += 1;
                }
            }
        }
    }

    return guard_sleeps;
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> i32 {
    let reports = parse_logs(input);
    let guard_sleeps = collect_sleep_schedule(reports);

    let (best_guard, best_minutes) = guard_sleeps
        .into_iter()
        .max_by_key(|(_id, minutes)| minutes.iter().sum::<i32>())
        .unwrap();

    let best_minute = best_minutes
        .iter()
        .enumerate()
        .max_by_key(|&(_, val)| val)
        .unwrap()
        .0;
    println!(
        "guard #{}, minute {}, answer {}",
        best_guard,
        best_minute,
        best_guard * best_minute as u32
    );
    0
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> i32 {
    let reports = parse_logs(input);
    let guard_sleeps = collect_sleep_schedule(reports);

    let (best_guard, best_minutes) = guard_sleeps
        .into_iter()
        .max_by_key(|(_, minutes)| *minutes.iter().max().unwrap())
        .unwrap();

    let best_minute = best_minutes
        .iter()
        .enumerate()
        .max_by_key(|&(_, val)| val)
        .unwrap()
        .0;
    println!(
        "guard #{}, minute {}, answer {}",
        best_guard,
        best_minute,
        best_guard * best_minute as u32
    );
    0
}
