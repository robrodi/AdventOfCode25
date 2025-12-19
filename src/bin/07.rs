use bit_vec::BitVec;
use colored::Colorize;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {

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
                print!("{}", "S".green().bold());
            } else if bv_read_only.get(i).unwrap() && ch == '^' {
                bv.set(i, false);
                splits += 1;

                if i > 0 { bv.set (i-1 , true); }
                if i < width { bv.set (i+1 , true); }
                print!("{}", "^".white().bold());
            } 
            else if bv_read_only.get(i).unwrap(){
                print!("{}", "|".white().bold());
            } else {
                print!("{}", ".".dimmed());
            }
        }
        println!()
    }
    Some(splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }
}
