use std::fs;
use std::env;

fn is_increasing(v: &Vec<u64>) -> bool {
    let mut last: u64 = v[0];

    for i in 1..v.len() {
        if last >= v[i] {
            return false;
        }

        last = v[i];
    }

    return true;
}

fn is_decreasing(v: &Vec<u64>) -> bool {
    let mut last: u64 = v[0];

    for i in 1..v.len() {
        if last <= v[i] {
            return false;
        }

        last = v[i];
    }

    return true;
}

fn is_diff_in_bounds(v: &Vec<u64>) -> bool {
    for i in 1..v.len() {
        let diff: u64 = v[i-1].abs_diff(v[i]);
        if !((diff >= 1) && (diff <= 3)) {
            return false;
        }
    }

    return true;
}

fn tally_safe_reports(reports: &Vec<(Vec<u64>, bool)>) -> u64 {
    let mut tally: u64 = 0;

    for report in reports {
        if report.0.len() > 1 && report.1 {
            tally += 1;
        }
    }

    return tally;
}

//pt 2
fn is_safe_dampener(v: &Vec<u64>) -> bool {
    for i in 0..v.len() {
        let mut p: Vec<u64> = v.clone();
        p.remove(i);

        if (is_increasing(&p) || is_decreasing(&p)) && is_diff_in_bounds(&p) {
            return true;
        }
    }

    return false;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1])
        .expect("error reading input");

    let mut reports: Vec<(Vec<u64>, bool)> = contents
        .split('\n')
        .map(|s| {
            (s.split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<u64>>(), false)
        })
        .filter(|v| v.0.len() > 2)
        .collect();

    for report in &mut reports {
        report.1 = (is_increasing(&report.0) || is_decreasing(&report.0)) && is_diff_in_bounds(&report.0);
    }

    let num_safe: u64 = tally_safe_reports(&reports);

    println!("num safe: {}", num_safe);
    println!("***");

    for report in &mut reports {
        // skip if already marked safe
        if report.1 {
            continue;
        }

        report.1 = is_safe_dampener(&report.0);
    }

    let num_safe_dampener: u64 = tally_safe_reports(&reports);

    println!("num safe w/ dampener: {}", num_safe_dampener);
}

