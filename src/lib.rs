use std::ops::Add;

#[derive(Debug)]
pub struct Point(pub usize, pub usize);

impl Point {
    pub fn lower(&mut self) {
        self.0 -= 1;
    }
}

#[derive(Debug, Default)]
pub struct RelPoint(pub i64, pub i64);

impl RelPoint {
    pub fn lower(&mut self) {
        self.0 -= 1;
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

#[derive(Debug)]
pub struct Piece {
    pub center: Point,
    pub positions: [[RelPoint; 3]; 4],
}

impl Piece {
    pub fn move_center(&mut self, move_x: i64, move_y: i64) {
        let new_center_x = (self.center.0 as i64 + move_x) as usize;
        let new_center_y = (self.center.1 as i64 + move_y) as usize;
        self.center.0 = new_center_x;
        self.center.1 = new_center_y;
    }
}

// T
pub fn t() -> Piece {
    let center = Point(1, 4);
    let pos_0: [RelPoint; 3] = [RelPoint(-1, 0), RelPoint(0, -1), RelPoint(0, 1)];
    let pos_1: [RelPoint; 3] = [RelPoint(-1, 0), RelPoint(0, -1), RelPoint(1, 0)];
    let pos_2: [RelPoint; 3] = [RelPoint(0, -1), RelPoint(1, 0), RelPoint(0, 1)];
    let pos_3: [RelPoint; 3] = [RelPoint(-1, 0), RelPoint(1, 0), RelPoint(0, 1)];
    Piece {
        center,
        positions: [pos_0, pos_1, pos_2, pos_3],
    }
}
