advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let result = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(best_joltage_2)
        .sum();
        
    Some(result)
}
pub fn best_joltage_2(line: &str) -> u64 {
    if line.len() == 0 { return 0; }

    let max = line [0..line.len()-1].chars().max().unwrap();

    let max_index = line.find(max).unwrap();
    let next_max = line[(max_index+1)..].chars().max().unwrap();
    
    println!("Line: {}. Max: {} at index {}. Next Max: {}. Result: {}{}", line, max, max_index, next_max, max, next_max);

    10 * (max.to_digit(10).unwrap() as u64) + (next_max.to_digit(10).unwrap() as u64)
}
pub fn best_joltage_12(line: &str) -> u64 {
    if line.len() == 0 { return 0; }

    let mut max = Vec::new();
    let mut search_start = 0;

    for i in 0..12 {
        let search_legth = 12 - i;
        let search_end = line.len() - search_legth + 1;
        if search_end <= search_start { break; }

        let segment = &line[search_start..search_end];
        let segment_max = segment.chars().max().unwrap();
        max.push(segment_max);

        search_start += segment.find(segment_max).unwrap() + 1;
    }
    
    let number: String = max.into_iter().collect();
    number.parse::<u64>().unwrap()
}

pub fn part_two(input: &str) -> Option<u64> {
    let result = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(best_joltage_12)
        .sum();
        
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_joltage_2(){
        assert_eq!(best_joltage_2("987654321111111"), 98);
        assert_eq!(best_joltage_2("811111111111119"), 89);
        assert_eq!(best_joltage_2("234234234234278"), 78);
        assert_eq!(best_joltage_2("818181911112111"), 92);
    }

    #[test]
    fn test_joltage_12(){
        assert_eq!(best_joltage_12("987654321111111"), 987654321111);
        assert_eq!(best_joltage_12("811111111111119"), 811111111119);
        assert_eq!(best_joltage_12("234234234234278"), 434234234278);
        assert_eq!(best_joltage_12("818181911112111"), 888911112111);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
