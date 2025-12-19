use bit_vec::BitVec;
use colored::Colorize;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let debug = false;
    let lines = input
        .lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();

    let width = lines[0].len();
    let mut bv = BitVec::from_elem(width, false);
    let mut splits = 0;

    for line in lines {
        let bv_read_only = bv.clone();
        for (i, ch) in line.chars().enumerate() {
            
            if ch == 'S' { 
                bv.set(i, true);
                if debug { print!("{}", "S".green().bold()); }
            } else if bv_read_only.get(i).unwrap() && ch == '^' {
                bv.set(i, false);
                splits += 1;

                if i > 0 { bv.set (i-1 , true); }
                if i < width { bv.set (i+1 , true); }
                if debug {  print!("{}", "^".white().bold()); }
            } 
            else if bv_read_only.get(i).unwrap(){
                if debug { print!("{}", "|".white().bold()); }
            } else {
                if debug { print!("{}", ".".dimmed()); }
            }
        }
        if debug { println!(); }
    }
    Some(splits)
}

pub fn part_two(input: &str) -> Option<u64> {
        let lines = input
        .lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();
    
    let debug = false;
    let width = lines[0].len();
    let mut paths: Vec<u64> = vec![0; width];
    let mut splits = 0;

    for line in lines {
        for (i, ch) in line.chars().enumerate() {
            let timelines_on_this_path = paths[i];

            if ch == 'S' { 
                paths[i] = 1;
                if debug { print!("{}", "S".green().bold()); }
            } else if ch == '^' {
                paths[i] = 0;
                splits += 1;
                if i > 0 { paths[i-1] += timelines_on_this_path}
                if i < width { paths[i+1] += timelines_on_this_path; }
                if debug { print!("{}", "^".white().bold()); }
            }
            else if paths[i] > 0{
                if debug { print!("{}", "|".white().bold()); }
            } else {
                if debug { print!("{}", ".".dimmed()); }
            }
        }   
        if debug { println!("Splits: {splits}. Paths: {}", paths.clone().iter().sum::<u64>()); }
    }
    /*
        l3:     1 in, 1 ^        2 choices               2 total = n * 2
        l5:     2 in 2 ^     2 splits 3 choices    (121)  4 total = cumulative n*2
        L7:     3 in, 3^ ,2,2 - 3 splits, 4choices (1331).     8 total != total paths to a. ^ * 2
        L9:             3 splits, 6. total      - 2 const 
        3
        4
        6

     */
    Some(paths.iter().sum::<u64>()  as u64)
}

#[cfg(test)]
mod day7_tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
