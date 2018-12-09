use std::io::{Read, self};
use std::collections::{HashSet, HashMap};

const min: i32 = -300;
const max: i32 = 600;

fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer)
        .expect("Read stdin");
    let buffer = buffer.trim();

    let mut input : Vec<Coordinate> = buffer.split("\n")
        .map(|i| i.trim())
        .enumerate()
        .map(|(idx, s)| Coordinate::parse(idx, s).unwrap())
        .collect();

    let mut infinite = HashSet::new();
    let mut counts = HashMap::new();

    for y in min..=max {
        let is_edge = y == min || y == max;
        for x in min..=max {
            let is_edge = is_edge || x == min || x == max;

            let point = (x, y);
            let closest = closest(&input, &point);

            if let Some(coord) = closest {
                if is_edge {
                    infinite.insert(coord.id);
                }

                let entry = counts.entry(coord.id).or_insert(0);
                *entry += 1;
            }
        }
    }

    let mut items: Vec<(&usize, &i32)> = counts.iter()
        .filter(|(key, val)| !infinite.contains(key))
        .collect();

    items.sort_by_key(|(key, val)| 0 - *val);

    println!("{}", *items[0].1);

}

type Point = (i32, i32);

struct Coordinate {
    id: usize,
    point: Point,
}

impl Coordinate {
    fn new(id: usize, point: Point) -> Coordinate {
        Coordinate {
            id,
            point,
        }
    }

    fn parse(idx: usize, s: &str) -> Option<Coordinate> {
        let mut r = s.split(", ");
        let x = r.next()?.parse::<i32>().unwrap();
        let y = r.next()?.parse::<i32>().unwrap();
        Some(Self::new(idx, (x, y)))
    }

    fn manhatten_distance(&self, point: &Point) -> i32 {
        let x_distance = (self.point.0 - point.0).abs();
        let y_distance = (self.point.1 - point.1).abs();

        x_distance + y_distance
    }
}

fn closest<'a>(list: &'a Vec<Coordinate>, point: &Point) -> Option<&'a Coordinate> {
    let mut closest_distance = 10000;
    let mut closest_coordinate = None;
    for coord in list {
        let distance = coord.manhatten_distance(point);
        if distance < closest_distance {
            closest_distance = distance;
            closest_coordinate = Some(coord);
        }
        else if distance == closest_distance {
            closest_coordinate = None
        }
    }

    closest_coordinate
}
