advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let (values, rows, columns) = parse(input);
    let number_rows = rows - 2;
    let mut column_results = Vec::new();
    for c in 0..columns {
        let numbers = (0..=number_rows).map(|r| index(&values, columns, r, c).parse::<u64>().unwrap());
        let operator = index(&values, columns, rows - 1, c);
        let value:u64 = if operator == "+"              { numbers.sum() } 
                        else if operator == "*"         { numbers.product() } else { panic!("Unknown operator {}", operator) };
        println!("Column {c}: {:?} -> {} = {value}", (0..=number_rows).map(|r| index(&values, columns, r, c).parse::<u64>().unwrap()).collect::<Vec<u64>>(), operator);
        column_results.push(value);
    }
    Some(column_results.iter().sum())
}
pub fn parse(input: &str) -> (Vec<&str>, usize, usize) {
    let lines =     input
        .lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();
    let rows = lines.len();
    let values = input.split_whitespace().collect::<Vec<&str>>();
    let columns = values.len() / rows;

    (values, rows, columns)
}

pub fn index<'a>(values: &'a Vec<&str>, width: usize, row: usize, col: usize) -> &'a str {
    values[row * width + col]
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day6_test_parse_and_index(){
        let input = &advent_of_code::template::read_file("examples", DAY);
        let (values, rows, columns) = parse(input);
        assert_eq!(rows, 4);
        assert_eq!(columns, 4);
        assert_eq!(index(&values.clone(), columns, 0, 0), "123");
        assert_eq!(index(&values.clone(), columns, 0, 1), "328");
        assert_eq!(index(&values.clone(), columns, 2, 3), "314");
        assert_eq!(index(&values.clone(), columns, 3, 1), "+");
    }
    #[test]
    fn day6_test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn day6_test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
