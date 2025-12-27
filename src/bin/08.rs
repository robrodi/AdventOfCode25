advent_of_code::solution!(8);
use std::fmt;
use std::collections::HashMap;
use std::collections::BTreeSet;
use std::vec;

struct Point3d(u32, u32, u32);
struct JunctionBox {
    point: Point3d,
    id: usize
}
impl fmt::Display for JunctionBox {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Box {} at ({}, {}, {})", self.id, self.point.0, self.point.1, self.point.2)
    }
}


// impl ToString for JunctionBox {
//     fn to_string(&self) -> String {
//         format!("Box {} at ({}, {}, {})", self.id, self.point.0, self.point.1, self.point.2)
//     }

// }
pub fn part_one(input: &str) -> Option<u64> {
    // parse
    let lines = input
        .lines()
        .filter(|line| !line.is_empty());
    let points_3d = lines
        .map(|line| line.trim().split(",").map(|num| num.parse::<u32>().unwrap()).collect::<Vec<u32>>())
        .map(|nums| Point3d(nums[0], nums[1], nums[2]))
        .collect::<Vec<Point3d>>();

    let mut boxes: HashMap<usize, JunctionBox> = HashMap::new();
    
    for i in 0..points_3d.len() {
        let p = &points_3d[i];
        boxes.insert(i, JunctionBox { point: Point3d(p.0, p.1, p.2), id: i });
    }

    let mut distances: HashMap<(usize, usize), u32> = HashMap::new();
    let mut closest_distances: BTreeSet<u32> = BTreeSet::new();
    let number_of_connections = 1000;
    
    // n*n distance.
    for i in boxes.keys() {
        for j in boxes.keys() {
            if i >= j { continue; }
            

            let box_a = boxes.get(i).unwrap();
            let box_b = boxes.get(j).unwrap();

            // store distance?
            let distance = distance_sq(&box_a.point, &box_b.point);
            distances.insert((box_a.id, box_b.id), distance);
            // if closest_distances.len() < number_of_connections || distance <= *closest_distances.iter().last().unwrap() {
            closest_distances.insert(distance);
            // }
        }
    }

    

    let nth = closest_distances.iter().nth(number_of_connections - 1).unwrap();
    println!("Distances: {}, nth: {}", distances.len(), (*nth as f64).sqrt());

    let mut top_n = Vec::new();
    for ((id_a, id_b), dist) in distances {
        if dist <= *nth {
            top_n.push( (id_a, id_b, dist) );
            // let box_a = boxes.get(&id_a).unwrap();
            // let box_b = boxes.get(&id_b).unwrap();

            // println!("Box {} <-> Box {} = {}", box_a, box_b, (*dist as f64).sqrt());
        }
    }
    
    let mut sorted_top_n = top_n.clone();
    sorted_top_n.sort_by_key(|b| b.2);
    // for top in sorted_topN.iter().take(number_of_connections)  {
    //     let box_a = boxes.get(&top.0).unwrap();
    //     let box_b = boxes.get(&top.1).unwrap();

    //     println!("Box {} <-> Box {} = {}", box_a, box_b, (top.2 as f64).sqrt());
    // }

    println!();
    let mut circuits: Vec<Vec<usize>> = Vec::new();

    let mut connections = 0;
    let num_cords = number_of_connections;

    for pair in sorted_top_n{
        let mut found = false;
        let mut first_circuit_index: Option<usize>= None;
        let mut to_join: Vec<(usize, usize)> = Vec::new();

        for c in 0..circuits.len() {

            if c >= circuits.len() || num_cords <= connections { break; } // circuits may shrink during loop.

            let circuit = &mut circuits[c];
            if circuit.contains(&pair.0) || circuit.contains(&pair.1) {
                if first_circuit_index == None { first_circuit_index = Some(c); }
                if found 
                { 
                    println!("Join? First Circuit: {first_circuit_index:?} Current Circuit: {c}" );
                    to_join.push( (first_circuit_index.unwrap(), c) );
                    // // add all of current circuit to first circuit
                    // let first_circuit = &mut circuits[first_circuit_index.unwrap()];
                    // first_circuit.append(circuit);
                    // // remove current circuit from circuit list. 
                    // circuits.remove(c);
                }
                found = true;

                if !circuit.contains(&pair.0) {
                    circuit.push(pair.0);
                } else if !circuit.contains(&pair.1) {
                    circuit.push(pair.1);
                }
                else {
                    println!("Both boxes already in circuit!");
                    continue;
                }

                //println!("{connections:2} Adding {}-{} to Existing circuit {:?}", &pair.0, &pair.1, circuit);
                connections += 1;

            }

            for joins in &to_join {
                let circuit_b = &mut circuits.remove(joins.1).clone();
                let circuit_a = &mut circuits[joins.0];
                //println!("Joining circuits {:?} and {:?}", circuit_a, circuit_b);

                for b in circuit_b {
                    if !circuit_a.contains(b) {
                        circuit_a.push(*b);
                    }
                }
            }
        }
        if !found {
            circuits.push(vec![pair.0, pair.1]);
            println!("{connections:2} Adding {}-{} to NEW     circuit {:?}", boxes[&pair.0], boxes[&pair.1], circuits.last().unwrap());
            connections += 1;
        }
    }


    println!("Total Circuits: {}", circuits.len());
    let mut sorted_circuits = circuits.clone();
    sorted_circuits.sort_by_key(|f| f.len());
    sorted_circuits.reverse();
    for c in sorted_circuits.clone() {
        println!(" Circuit: {} - {:?}", c.len(), c);
    }
    let top_3: Vec<u64>= sorted_circuits.iter().take(3).map(|n| n.len() as u64).collect();
    top_3.iter().for_each(|n| println!(" Top 3 Circuit Size: {}", n));
    Some(top_3.iter().product())
}

fn distance_sq(a: &Point3d, b: &Point3d) -> u32 {
    let dx = (a.0 as i64 - b.0 as i64).pow(2);
    let dy = (a.1 as i64 - b.1 as i64).pow(2);
    let dz = (a.2 as i64 - b.2 as i64).pow(2);

    (dx + dy + dz) as u32
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod day8_tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result: Option<u64> = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
