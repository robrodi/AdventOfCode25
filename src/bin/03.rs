advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let result = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(best_joltage)
        .sum();
        
    Some(result)
}
pub fn best_joltage(line: &str) -> u64 {
    if line.len() == 0 { return 0; }

    let max = line [0..line.len()-1].chars().max().unwrap();

    let max_index = line.find(max).unwrap();
    let next_max = line[(max_index+1)..].chars().max().unwrap();
    
    println!("Line: {}. Max: {} at index {}. Next Max: {}. Result: {}{}", line, max, max_index, next_max, max, next_max);

    10 * (max.to_digit(10).unwrap() as u64) + (next_max.to_digit(10).unwrap() as u64)
}

// unused.
pub fn all_number_pairs(line: &str) -> Vec<u64> {
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();
    let mut result = Vec::new();

    for i in 0..(len - 1) {
        let num = if i < (len - 2) { 
                            format!("{}{}", chars[i], chars[i+1]).parse::<u64>().unwrap() 
                        } else { 
                            chars[i].to_string().parse::<u64>().unwrap() 
                        }; // chars[i+1];
        println!("Char Pair at index {} is {:2}", i, num);
        result.push(num);
    }
    result
}
pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pairs(){
        let result = all_number_pairs("1234567890");
        assert_eq!(result.len(), 9);
        assert_eq!(result, vec![12,23,34,45,56,67,78,89,9]);
    }
    #[test]
    fn test_joltage(){
        assert_eq!(best_joltage("987654321111111"), 98);
        assert_eq!(best_joltage("811111111111119"), 89);
        assert_eq!(best_joltage("234234234234278"), 78);
        assert_eq!(best_joltage("818181911112111"), 92);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
