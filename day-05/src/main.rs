use std::cmp::max;
use std::collections::HashSet;
use tools::read_input_file;

#[derive(Debug)]
#[derive(Clone)]
struct IdRange {
    lower: u64,
    upper: u64,
}

fn main() {
    let input: String = read_input_file();
    let lines: Vec<&str> = input.lines().collect();
    let id_ranges= collect_id_ranges(lines.clone());
    let ids = collect_ids(lines.clone());
    println!("{:?}", id_ranges);
    println!("{:?}", ids);
    let valid_ids: HashSet<u64> = ids.iter().filter(|&id| validate_id(id, &id_ranges)).cloned().collect();
    println!("Valid IDs:");
    println!("{:?}", valid_ids);
    println!("Count of valid IDs: {}", valid_ids.len());
    println!("Count of valid IDs in the range: {}", count_valid_ids(&id_ranges));
}

fn collect_id_ranges(lines: Vec<&str>) -> Vec<IdRange> {
    let id_ranges_str: Vec<&str> = lines
        .into_iter()
        .take_while(|l| l.len() > 0 && l.contains('-'))
        .collect();
    parse_id_ranges(id_ranges_str)
}

fn parse_id_ranges(id_ranges_str: Vec<&str>) -> Vec<IdRange> {
    id_ranges_str
        .into_iter()
        .map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            IdRange {
                lower: start.parse::<u64>().unwrap(),
                upper: end.parse::<u64>().unwrap(),
            }
        })
        .collect()
}

fn collect_ids(lines: Vec<&str>) -> Vec<u64> {
    let id_ranges_str: Vec<&str> = lines
        .into_iter()
        .skip_while(|l| l.len() == 0 || l.contains('-'))
        .collect();
    id_ranges_str.into_iter().map(|l| l.parse::<u64>().unwrap()).collect()
}

fn validate_id(id: &u64, ranges: &Vec<IdRange>) -> bool {
    ranges.iter().any(|range| range.lower <= *id && range.upper >= *id)
}

fn count_valid_ids(ranges: &Vec<IdRange>) -> u64 {
    if ranges.is_empty() {
        return 0;
    }
    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort_by_key(|r| r.lower);

    let mut total_count: u64 = 0;

    let mut current_start = sorted_ranges[0].lower;
    let mut current_end = sorted_ranges[0].upper;

    for range in sorted_ranges.iter().skip(1) {
        if range.lower <= current_end + 1 {
            current_end = max(current_end, range.upper);
        } else {
            total_count += (current_end - current_start) + 1;
            current_start = range.lower;
            current_end = range.upper;
        }
    }
    total_count += (current_end - current_start) + 1;

    total_count
}