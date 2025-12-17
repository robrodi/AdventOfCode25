use colored::Colorize;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = input.split(",");
    let mut sum_of_invalid_numbers: u64 = 0;

    for range in ranges {
        let bounds: Vec<&str> = range.split("-").collect();
        let start: u64 = bounds[0].parse().unwrap();
        let end: u64 = bounds[1].parse().unwrap();

        let invalid_numbers: Vec<u64> = validate_range(start, end).collect::<Vec<u64>>();
        let s = start.to_string().white().bold();
        let e = end.to_string().white().bold();
        if invalid_numbers.len() > 0 {
            println!("{}-{} has {} invalid IDs, {:?}.", s, e, invalid_numbers.len(), invalid_numbers);
        } else{
            println!("{}-{} contains no invalid IDs", s, e);
        }

        sum_of_invalid_numbers += invalid_numbers.iter().sum::<u64>();
    }

    Some(sum_of_invalid_numbers)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

fn validate_number(number: u64) -> bool {
    let debug = false;
    if number == 0 { return true; }
    // cant be a full repetition with an odd number of digits.
    let num_digits = number.ilog10() + 1;
    if debug { print!("Number: {}. num_digits: {}. ", number, num_digits); }
    if !num_digits.is_multiple_of(2) { 
        if debug { println!("Odd number of digits, automatically valid."); }
        return true; 
    }

    let split = ((num_digits / 2) - 1) as usize;

    let s = number.to_string();

    let l = s[0..=split].to_string();
    let r = s[split+1..].to_string();

    let result = l != r;
    if debug { println!("split = {}. L: {}, R: {}. Result: {}", split, l, r, result); } //if result { "✓".green().bold() } else { "x".red().bold() });

    result 
}

fn validate_range(start: u64, end: u64) -> impl Iterator<Item = u64> {
    let numbers = start..=end;
    numbers.filter(|n| !validate_number(*n))    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }
    
    #[test]
    fn simple_part_one_number_validation() {
        assert!(validate_number(1));
        assert!(validate_number(12));
        assert!(!validate_number(11));
        assert!(!validate_number(11111111));
        assert!(!validate_number(22222222));
        assert!(validate_number(12321));
        assert!(!validate_number(1010));
        assert!(!validate_number(1188511885));
    }

    #[test]
    fn simple_part_one_range_validation() {
        let result = validate_range(11, 22).collect::<Vec<u32>>();
        assert_eq!(result, vec![11, 22]);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
