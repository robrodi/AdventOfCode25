advent_of_code::solution!(5);
use std::collections::BTreeSet;
use std::cmp::min;
use std::cmp::max;


pub fn part_one(input: &str) -> Option<u64> {
    let debug = false;
    let (starts, ends, ids) = parse(input);

    let mut fresh_id_count = 0;
    let range_count = starts.len();
    for id in ids  {

        let mut j = 0;
        while j < range_count {
            if (debug) { println!("Checking ID {id} against range {}-{}", starts[j], ends[j]); }
            if starts[j] <= id && ends[j] >= id {
                fresh_id_count += 1;
                if (debug) { println!("ID {id} is fresh."); }
                break;
            }    
            j += 1;
        }
    }

    Some(fresh_id_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (starts, ends, _ids) = parse(input);

    Some(starts.into_iter().zip(ends).map(|(s, e)| { e-s +1 }).sum())
}

fn parse (input: &str) -> (Vec<u64>, Vec<u64>, Vec<u64>) {
    let debug = false;
    let mut starts :BTreeSet<u64> = BTreeSet::new();
    let mut ends :BTreeSet<u64> = BTreeSet::new();
    let mut ids: Vec<u64> = Vec::new();
    let lines: std::str::Lines<'_> = input.lines();
    let mut overlaps = 0;
    for line in lines {
        if line == "" { continue; }

        if line.contains('-') {
            let parts: Vec<&str> = line.split("-").collect();
            let mut s :u64 = parts[0].parse().unwrap();
            let mut e: u64 = parts[1].parse().unwrap();
            if (debug) { print!("Parsing range: {}-{}. ", parts[0], parts[1]); }

            let size: usize = starts.len();
            let mut starts_to_remove = Vec::new();
            let mut ends_to_remove = Vec::new();

            for i in 0..size {
                let existing_start = *starts.iter().nth(i).unwrap();
                let existing_end = *ends.iter().nth(i).unwrap();
                if (debug) { print!("\n\tChecking against existing range {existing_start}-{existing_end}."); }

                // start before existing end.
                // case 1: existing start, new start, existing end, new end
                // case 2: new start, existing start, new end, existing end
                if (s <= existing_end && e >= existing_start) || (s <= existing_end && e >= existing_start) {
                    let new_start = min(s, existing_start);
                    let new_end = max(e, existing_end);
                    starts_to_remove.push(existing_start);
                    ends_to_remove.push(existing_end);
                    overlaps += 1;
                    if (debug) { println!("\n\tNew Range {existing_start}-{existing_end} overlaps with existing range {s}-{e}. Target range {new_start}-{new_end}"); }
                    s = new_start;
                    e = new_end;    
                    continue;
                }
            }
            println!();
            for rem in 0..starts_to_remove.len()
            {
                let s_val = starts_to_remove[rem];
                let e_val = ends_to_remove[rem];

                starts.remove(&s_val);
                ends.remove(&e_val);
            }
            
            starts.insert(s);
            ends.insert(e);
        } else {
            ids.push(line.parse().unwrap());
        }
    }

    println!("Total number of ranges: {}. Total overlaps merged: {overlaps}", starts.len());
    
    (starts.into_iter().collect(), ends.into_iter().collect(), ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5_test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn day5_test_part_one_overlap(){
        // overlaps at the end of existing range
        let (starts, ends, ids) = parse(&advent_of_code::template::read_file("examples", DAY));
        let size: usize = starts.len();

        for i in 0..size {
            println!("Range {}: {}-{}", i+1, starts[i], ends[i]);
        }
        assert_eq!(size, 2);

        // overlaps at the start of existing range
        let (starts, ends, ids) = parse("4-6\n1-5");
        assert_eq!(starts.len(), 1);
        assert_eq!(starts[0], 1);
        assert_eq!(ends[0], 6);

        // entirely inside existing range
        let (starts, ends, ids) = parse("1-6\n3-5");
        assert_eq!(starts.len(), 1);
        assert_eq!(starts[0], 1);
        assert_eq!(ends[0], 6);

    }

    #[test]
    fn day5_test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
