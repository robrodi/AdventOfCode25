advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let lines= lines(input);

    let _height = lines.len();
    let _width = lines[0].len();

    let mut accessible_rolls = 0;
    for h in 0..lines.len() {
        for w in 0..lines[0].len() {
            if !is_roll(h, w, &lines) { 
                print!(".");
                continue; 
            }
            let neighbors = num_neighbors(h, w, &lines);
            if neighbors < 4 { 
                accessible_rolls += 1; 
                print!("x");
            }
            else {
                print!("@");
            }
        }
        println!();
    }

    Some(accessible_rolls)
}
fn lines(input: &str) -> Vec<&str> {
    input
        .lines()
        .filter(|line| !line.is_empty()).collect()
}

fn is_roll(h: usize, w:usize, lines: &Vec<&str>) -> bool {
    lines[h].as_bytes()[w] == b'@'
}

fn num_neighbors(h: usize, w: usize, lines: &Vec<&str>) -> u32 { 
    let mut tests = Vec::new();

    if h > 0 { // is not top row.
        if w > 0 { tests.push((h-1,w-1)); }
        tests.push((h-1,w));
        if w < lines[0].len() - 1 { tests.push((h-1,w+1)); }
    }

    if w > 0 { tests.push((h,w-1)); }
    if w < lines[0].len() - 1 { tests.push((h,w+1)); }

    if h < lines.len() - 1 { // is not bottom row.
        if w > 0 { tests.push((h+1,w-1)); }
        tests.push((h+1,w));
        if w < lines[0].len() - 1 { tests.push((h+1,w+1)); }
    }

    tests.iter().map(|(nh, nw)| is_roll(*nh, *nw, lines) )
                .filter(|&y|y).count() as u32
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
#[test]
    fn test_is_roll() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let lines = lines(input);
        assert!(!is_roll(0, 0,  &lines));
        assert!(!is_roll(0, 1,  &lines));
        assert!(is_roll(1, 0,  &lines));
        assert!(is_roll(0, 2,  &lines));
    }

    #[test]
    fn test_num_neighbors() {
        let _input = &advent_of_code::template::read_file("examples", DAY);
        let test = "@..\n.@.\n...\n";
        assert!(is_roll(0, 0, &lines(test)));
        assert_eq!(num_neighbors(1, 1, &lines(test)), 1);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
