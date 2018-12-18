use regex::Regex;
use std::io::{Read, self};
use std::str::FromStr;

fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer)
        .expect("Read stdin");
    let buffer = buffer.trim();

    let input : Vec<Point> = buffer.split("\n")
        .map(|i| i.trim())
        .map(|s| s.parse::<Point>().unwrap())
        .collect();
    let points = Points::from_vec(input);

    let mut min_area = std::i64::MAX;
    let mut min_index = 0;
    let mut min_bounds = None;
    for i in 9000..11000 {
        let points = points.transit(i);
        let area = points.area();
        if area < min_area {
            min_area = area;
            min_index = i;
            min_bounds = Some(points.bounds());
        }
        /*
        println!("{:?} => {}", points.bounds(), points.area());
        print!("{}", points.output(-6, -4, 22, 16));
        println!();
        */
    }

    println!("{}: {:?}", min_index, min_bounds);

    for i in -2..=2 {
        let points = points.transit(min_index + i);
        let bounds = min_bounds.unwrap();
        print!("{}", points.output(bounds.0 - 5, bounds.1 - 5, (bounds.2 - bounds.0 + 10) as usize, (bounds.3 - bounds.1 + 10) as usize));
        println!();
    }
}

#[derive(Debug, Clone, Copy)]
struct Point(i32, i32, i32, i32);

impl Point {
    fn transit(self, steps: i32) -> Point {
        let x = self.0 + (self.2 * steps);
        let y = self.1 + (self.3 * steps);
        Point(x, y, self.2, self.3)
    }
}

impl FromStr for Point {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Point, Self::Err> {
        let re = Regex::new(r"position=<\s*(?P<px>-?\d+),\s*(?P<py>-?\d+)> velocity=<\s*(?P<vx>-?\d+),\s*(?P<vy>-?\d+)>").unwrap();
        let captures = re.captures(s).unwrap();
        let px = captures.name("px").unwrap().as_str().parse::<i32>()?;
        let py = captures.name("py").unwrap().as_str().parse::<i32>()?;
        let vx = captures.name("vx").unwrap().as_str().parse::<i32>()?;
        let vy = captures.name("vy").unwrap().as_str().parse::<i32>()?;

        Ok(Point(px, py, vx, vy))
    }
}

#[derive(Debug)]
struct Points(Vec<Point>);

impl Points {
    fn from_vec(input: Vec<Point>) -> Points {
        Points(input)
    }

    fn transit(&self, steps: i32) -> Points {
        Points(self.0.iter().map(|point| point.transit(steps)).collect())
    }

    fn bounds(&self) -> (i32, i32, i32, i32) {
        use std::cmp::{min, max};

        if self.0.len() == 0 {
            return (0, 0, 0, 0);
        }

        let mut min_x = self.0[0].0;
        let mut max_x = self.0[0].0;
        let mut min_y = self.0[0].1;
        let mut max_y = self.0[0].1;

        for p in &self.0 {
            min_x = min(min_x, p.0);
            max_x = max(max_x, p.0);
            min_y = min(min_y, p.1);
            max_y = max(max_y, p.1);
        }

        (min_x, min_y, max_x, max_y)
    }

    fn area(&self) -> i64 {
        let bounds = self.bounds();
        let width = bounds.2 - bounds.0;
        let height = bounds.3 - bounds.1;

        width as i64 * height as i64
    }

    fn output(&self, start_x: i32, start_y: i32, width: usize, height: usize) -> String {
        let mut vec: Vec<Vec<bool>> = Vec::new();
        for _ in 0 .. height {
            let mut v = Vec::new();
            v.resize(width, false);
            vec.push(v);
        }

        for point in &self.0 {
            let x = point.0;
            let y = point.1;

            let x2 = x - start_x;
            let y2 = y - start_y;

            if x2 < 0 || y2 < 0 {
                continue;
            }
            let x2 = x2 as usize;
            let y2 = y2 as usize;

            if x2 >= width || y2 >= height {
                continue;
            }

            vec[y2][x2] = true;
        }

        let mut s = String::new();
        for line in vec {
            s += &line.iter().map(|x| if *x { '#' } else { '.' }).collect::<String>();
            s += "\n";
        }
        s
    }
}
