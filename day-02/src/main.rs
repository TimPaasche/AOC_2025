use std::collections::HashSet;
use std::env;
use std::fs;
use std::hash::Hash;

struct IdRange {
    start: u32,
    end: u32,
}

fn main() {
    let path = env::current_dir().unwrap().to_str().unwrap().to_string();
    let contents_with_bom = fs::read_to_string(path + "\\src\\input.txt").unwrap();
    let contents = contents_with_bom.trim_start_matches('\u{FEFF}');
    let mut invalid_ids: HashSet<u32> = HashSet::new();
    let id_range_strs = contents.split(',').collect::<Vec<&str>>();
    let id_ranges: Vec<IdRange> = id_range_strs
        .iter()
        .map(|str| {
            let trimmed_str = str.trim_end_matches("\r\n");
            let splited_str = trimmed_str.split('-').collect::<Vec<&str>>();
            IdRange {
                start: splited_str[0].parse::<u32>().unwrap_or(0),
                end: splited_str[1].parse::<u32>().unwrap_or(0),
            }
        })
        .collect();
    let mut max_len_id = 0;

    // PT1
    // for id_range in id_ranges {
    //     for id in id_range.start..id_range.end {
    //         let id_str = id.to_string();
    //         if id_str.len() > max_len_id { max_len_id = id_str.len(); }
    //         let id_str_split = id_str.split_at(id_str.len() / 2);
    //         let valid: bool = id_str_split.0 != id_str_split.1;
    //         if !valid {
    //             invalid_ids.push(id);
    //         }
    //     }
    // }

    for id_range in id_ranges {
        for id in id_range.start..id_range.end {
            let id_str = id.to_string();
            let max_pattern_len = id_str.len() / 2;
            for pattern_len in 1..=max_pattern_len {
                let pattern = id_str.get(..pattern_len).unwrap().to_string();
                let reduced_id_str = id_str.trim_start_matches(&pattern);
                if reduced_id_str.is_empty() {
                    invalid_ids.insert(id);
                }
            }
        }
    }

    println!("Max ID length: {}", max_len_id);
    let mut sum_of_invalid_ids: u64 = 0;
    for id in invalid_ids {
        sum_of_invalid_ids += id as u64;
        println!("invalid_id: {}", id);
    }
    println!("------------------------------------------");
    println!("sum of invalid ids: {}", sum_of_invalid_ids);
}
