advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let (values, rows, columns) = parse_one(input);
    let number_rows = rows - 2;
    let mut column_results = Vec::new();
    for c in 0..columns {
        let numbers = (0..=number_rows).map(|r| get_value(&values, columns, r, c).parse::<u64>().unwrap());
        let operator = get_value(&values, columns, rows - 1, c);
        let value:u64 = if operator == "+"              { numbers.sum() } 
                        else if operator == "*"         { numbers.product() } else { panic!("Unknown operator {}", operator) };
        // println!("Column {c}: {:?} -> {} = {value}", (0..=number_rows).map(|r| get_value(&values, columns, r, c).parse::<u64>().unwrap()).collect::<Vec<u64>>(), operator);
        column_results.push(value);
    }
    Some(column_results.iter().sum())
}
pub fn parse_one(input: &str) -> (Vec<&str>, usize, usize) {
    let lines = input
        .lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();
    let rows = lines.len();
    let values = input.split_whitespace().collect::<Vec<&str>>();
    let columns = values.len() / rows;

    (values, rows, columns)
}

pub fn get_value<'a>(values: &'a Vec<&str>, width: usize, row: usize, col: usize) -> &'a str {
    values[row * width + col]
}

pub fn parse_two(input: &str) -> (Vec<char>, usize, usize) {
    let lines = input
        .lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();
    let rows = lines.len();
    let lchars = input.chars().collect::<Vec<char>>();
    let columns = lchars.len() / rows;

    (lchars, rows, columns)
}
pub fn part_two(input: &str) -> Option<u64> {
    let (lchars, rows, cols) = parse_two(input);
    // println!("Rows: {}, Cols: {}", rows, cols);

    let mut operator: char = ' ';
    let mut problem_results: Vec<u64> = Vec::new();
    let mut problem_parts: Vec<u64> = Vec::new();

    for c in 0..cols {
        let op_char = lchars[index(cols,rows -1, c)];
        if op_char != ' ' { 
            problem_parts = Vec::new();
            operator = op_char; 
            // println!("problem operator: {operator} at index {}", index(cols,rows -1, c));
        }

        // read all the characters in this column except the final operator one.
        let mut column_value = Vec::new();
        for r in 0..rows - 1 {
            let value = lchars[index(cols,r,c)];
            if value == ' ' { continue; }
            column_value.push(value);
        }

        let number_count = column_value.len();
        // if there's a value, parse it to u64 and add to problem parts.
        if number_count > 0 { 
            let s: String = column_value.into_iter().collect();
            let value = s.parse::<u64>().unwrap();
            problem_parts.push(value);
            // println!("  Column {c} values: {value}");
        }

        // if it's an empty column, or the final column, this problem is complete. solve the problem and push the result.
        if number_count == 0 || c == cols -1 { 
            let value = if operator == '+' { 
                                problem_parts.iter().sum() 
                            } else if operator == '*' {
                                problem_parts.iter().product()
                            } else { 
                                panic!("Unknown operator {operator}"); 
                            };
            // println!("New Problem! Last Result = {value}");
            problem_results.push(value);
            continue; 
        }
// error here. it's conditional on an empty column to denote an end. if final column, it fails.

    }

    Some(problem_results.iter().sum())
}

pub fn index(width: usize, row: usize, col: usize) -> usize { row * (width + 1) + col }
    
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day6_test_parse_and_index(){
        let input = &advent_of_code::template::read_file("examples", DAY);
        let (values, rows, columns) = parse_one(input);
        assert_eq!(rows, 4);
        assert_eq!(columns, 4);
        assert_eq!(get_value(&values.clone(), columns, 0, 0), "123");
        assert_eq!(get_value(&values.clone(), columns, 0, 1), "328");
        assert_eq!(get_value(&values.clone(), columns, 2, 3), "314");
        assert_eq!(get_value(&values.clone(), columns, 3, 1), "+");
    }

    #[test]
    fn day6_test_parse_2(){
        let input = &advent_of_code::template::read_file("examples", DAY);
        let (lchars, rows, columns) = parse_two(input);
        for i in 0..lchars.len() {
            print!("{i} : {}.  ", if lchars[i] == '\n' { "\\n".to_string() } else { lchars[i].to_string() } );
        }
        assert_eq!(rows, 4);
        assert_eq!(columns, 15);
        assert_eq!(lchars[index(columns,3,0)], '*');

    }
    #[test]
    fn day6_test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn day6_test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
