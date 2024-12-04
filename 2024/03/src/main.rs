use std::fs;
use std::env;
use std::ops::Range;
use regex::Regex;

fn find_mul(s: &str) -> Vec<regex::Captures> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap(); 
    let mut captures: Vec<regex::Captures> = vec![];
    re.captures_iter(&s).for_each(|c| captures.push(c));

    return captures;
}

fn find_disabled_ranges(s: &str) -> Vec<Range<usize>> {
    let mut disabled_ranges: Vec<Range<usize>> = vec![];

    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();

    let mut current_range = Range::<usize>::default();
    let mut range_open = false;
    let mut pos: usize = 0;

    while pos < s.len() {
        if range_open {
            // saw a don't(), looking for a do()
            let do_match = do_re.find_at(&s, pos);
            match do_match {
                None => { // no more do()'s
                    pos = s.len();
                    current_range.end = s.len();
                }
                Some(_match) => { // found a do()
                    pos = _match.range().end;
                    current_range.end = _match.range().end;
                    range_open = false;
                }
            }

            disabled_ranges.push(current_range.clone());
        } else {
            // looking for a don't()
            let dont_match = dont_re.find_at(&s, pos);
            match dont_match {
                None => { // no more don't()'s
                    pos = s.len();
                }
                Some(_match) => { // found a don't()
                    pos = _match.range().end;
                    current_range.start = _match.range().end;
                    range_open = true;
                }
            }
        }
    }

    return disabled_ranges;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1])
        .expect("error reading input");

    let mut p1sum: i32 = 0;
    let mut p2sum: i32 = 0;

    let captures = find_mul(&contents);
    let disabled_ranges = find_disabled_ranges(&contents);
    
    for capture in captures {
        let (_, [x, y]) = capture.extract();
        let product = x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap();

        p1sum += product;

        let capture_start = capture.get(0).unwrap().start();
        if disabled_ranges.clone().into_iter().all(|r| !r.contains(&capture_start)) {
            p2sum += product;
        }
    }

    println!("part 1: {}", p1sum);
    println!("***");
    println!("part 2: {}", p2sum);
}

