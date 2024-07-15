use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x1: usize,
    pub x2: usize,
    pub y1: usize,
    pub y2: usize,
}

impl Rect {
    pub fn new(x: usize, y: usize, w: usize, h: usize) -> Rect {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    // Returns true if this overlaps with other
    pub fn intersect(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn center(&self) -> (usize, usize) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }

    pub fn rand_position(&self) -> (i32, i32) {
        let mut rand = rand::thread_rng();
        let x = rand.gen_range(self.x1 + 1..self.x2);
        let y = rand.gen_range(self.y1 + 1..self.y2);
        (x as i32, y as i32)
    }
}
