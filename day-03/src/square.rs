#[derive(Eq, PartialEq, Debug)]
pub struct Square {
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

impl Square {
    pub fn new(left: usize, top: usize, width: usize, height: usize) -> Square {
        Square {
            left,
            top,
            width,
            height,
        }
    }

    pub fn parse(input: &str) -> Option<Square> {
        let parts1: Vec<&str> = input.split(" @ ").collect();
        let parts2: Vec<&str> = parts1[1].split(": ").collect();
        let position = parts2[0];
        let size = parts2[1];

        let positions: Vec<usize> = position.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
        let sizes: Vec<usize> = size.split("x").map(|x| x.parse::<usize>().unwrap()).collect();

        Some(Square {
            left: positions[0],
            top: positions[1],
            width: sizes[0],
            height: sizes[1],
        })
    }

    pub fn intersect(&self, other: &Square) -> Option<Square> {
        let left1 = self.left;
        let right1 = self.left + self.width;
        let top1 = self.top;
        let bottom1 = self.top + self.height;

        let left2 = other.left;
        let right2 = other.left + other.width;
        let top2 = other.top;
        let bottom2 = other.top + other.height;

        let max_left = std::cmp::max(left1, left2);
        let min_right = std::cmp::min(right1, right2);

        if max_left >= min_right {
            return None;
        }

        let max_top = std::cmp::max(top1, top2);
        let min_bottom = std::cmp::min(bottom1, bottom2);

        if max_top >= min_bottom {
            return None;
        }

        Some(Square { left: max_left, top: max_top, width: min_right - max_left, height: min_bottom - max_top })
    }

    pub fn points(&self) -> SquareIterator {
        SquareIterator::new(self)
    }
}

pub struct SquareIterator<'a> {
    square: &'a Square,
    current: Option<(usize, usize)>,
}

impl<'a> SquareIterator<'a> {
    fn new(square: &'a Square) -> SquareIterator<'a> {
        SquareIterator {
            square,
            current: Some((square.left, square.top)),
        }
    }
}

impl<'a> Iterator for SquareIterator<'a> {
    type Item=(usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((left, top)) = self.current {
            let mut new_left = left + 1;
            let mut new_top = top;
            if new_left >= self.square.left + self.square.width {
                new_left = self.square.left;
                new_top = top + 1;
            }
            if new_top >= self.square.top + self.square.height {
                self.current = None;
            }
            else {
                self.current = Some((new_left, new_top))
            }
            Some((left, top))
        }
        else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator() {
        let square = Square::new(0, 0, 1, 1);

        for point in square.points() {
            println!("{:?}", point);
        }

        assert_eq!(square.points().count(), 1);
    }

    #[test]
    fn test_intersection() {
        let square1 = Square::new(3, 1, 4, 4);
        let square2 = Square::new(1, 3, 4, 4);

        let intersection = square1.intersect(&square2);

        assert_eq!(intersection, Some(Square { left: 3, top: 3, width: 2, height: 2 }));

        let square1 = Square::new(3, 1, 4, 4);
        let square2 = Square::new(5, 5, 2, 2);

        let intersection = square1.intersect(&square2);

        assert_eq!(intersection, None);
    }

    #[test]
    fn test_parse() {
        let square = Square::parse("#1 @ 1,3: 4x5");

        assert_eq!(square, Some(Square { left: 1, top: 3, width: 4, height: 5 }));
    }
}
