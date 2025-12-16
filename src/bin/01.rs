use colored::Colorize;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let numbers = group_lines(input);
    let mut stop = 50;
    let mut stops = 0;
    for item in &numbers {
        stop += item;
        stop %= 100;
        if stop == 0 {
            stops += 1;
        }
    }
    Some(stops)
}

pub fn part_two(input: &str) -> Option<u64> {
    let numbers = group_lines(input);
    let mut stop = 50;
    let mut stops = 0;
    let debug = false;

    if debug{
        println!("The dial starts by pointing at {}.", stop);
    }

    for item in &numbers {

        let rotation = *item;
        if rotation == 0 {
            continue;
        }
        let last = stop;
        let zeros =
        if rotation > 0 {
            stop = (stop + rotation) % 100;
            (last + rotation) / 100
        } else {
            let reversed = (100 - stop) % 100;
            stop = (stop + rotation).rem_euclid(100);
            (reversed - rotation) / 100
        };

        stops += zeros as u64;
        if debug {
            print!("{:0>4} The dial is rotated {} to point at {}", stops, rotation, stop);
            if zeros == 1 {
                print!(", ; during this rotation, it points at 0 {}", "once".white().bold());
            } else if zeros > 1 {
                print!("; during this rotation, it points at 0 {} time(s)", zeros.to_string().white().bold());
            }

            println!(".");
        }
    }
    Some(stops)
}

fn parse_rotation(input: &str) -> i32 {
    let line = input.trim();

    let i = line[1..].parse::<i32>();

    match line.chars().nth(0).unwrap() {
        'L' => -i.unwrap(), 
        'R' => i.unwrap(),
        _ => panic!("Line must start with L or R: {}", line),
    }
}

fn group_lines(input: &str) -> Vec<i32> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_rotation)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
