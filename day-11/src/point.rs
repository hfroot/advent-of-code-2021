#[derive(Copy, Clone)]
pub struct Point {
    x: u8,
    y: u8,
    // feels weird to covert to flat array here but oh well
    pub neighbours: [Option<u8>;8]
}

// all this point logic assumes a 10x10 grid
fn point(x: &u8, y: &u8) -> Point {
    let mut neighbours = [None;8];
    let y = *y;
    let x = *x;
    if y != 0 {
        let up = (y - 1) * 10 + x;
        neighbours[0] = Some(up);
        if x != 9 {
            neighbours[1] = Some(up + 1);
        }
        if x != 0 {
            neighbours[7] = Some(up - 1);
        }
    }
    let this_point = y * 10 + x;
    if x != 0 {
        neighbours[6] = Some(this_point - 1);
    }
    if x != 9 {
        neighbours[2] = Some(this_point + 1);
    }
    if y != 9 {
        let down = (y + 1) * 10 + x;
        neighbours[4] = Some(down);
        if x != 9 {
            neighbours[3] = Some(down + 1);
        }
        if x != 0 {
            neighbours[5] = Some(down - 1);
        }
    }
    Point {
        x,
        y,
        neighbours: neighbours,
    }
}

pub fn point_from_idx(idx: &u8) -> Point {
    let y = idx / 10;
    point(&(idx - y * 10), &y)
}

pub fn point_to_idx(point: Point) -> u8 {
    point.y * 10 + point.x
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use crate::point::*;
    #[test]
    fn correct_point() {
        let p = point_from_idx(&35);
        assert_eq!(p.x, 5);
        assert_eq!(p.y, 3);
    }
    #[test]
    fn correct_neighbours() {
        let p = point(&5,&5);
        assert_eq!(p.neighbours, [Some(45), Some(46), Some(56), Some(66), Some(65), Some(64), Some(54), Some(44)]);
        let p = point(&0, &0);
        assert_eq!(p.neighbours, [None, None, Some(1), Some(11), Some(10), None, None, None]);
        let p = point(&9, &0);
        assert_eq!(p.neighbours, [None, None, None, None, Some(19), Some(18), Some(8), None]);
        // I'm not convinced need to cap length of grid but for sanity's sake:
        let p = point(&9, &9);
        assert_eq!(p.neighbours, [Some(89), None, None, None, None, None, Some(98), Some(88)]);
    }
}