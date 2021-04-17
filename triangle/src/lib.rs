use num::Num;

pub struct Triangle<N: Num + PartialOrd + Copy> {
    x: N,
    y: N,
    z: N,
}

impl<N: Num + PartialOrd + Copy> Triangle<N> {
    pub fn build(sides: [N; 3]) -> Option<Triangle<N>> {
        if sides[0].is_zero() || sides[1].is_zero() || sides[2].is_zero() {
            return None;
        }
        if sides[0] + sides[1] < sides[2] ||
            sides[1] + sides[2] < sides[0] ||
            sides[0] + sides[2] < sides[1] {
            return None;
        }
        Some(Self {
            x: sides[0],
            y: sides[1],
            z: sides[2],
        })
    }

    pub fn is_equilateral(&self) -> bool {
        self.x == self.y && self.y == self.z
    }

    pub fn is_scalene(&self) -> bool {
        self.x != self.y && self.y != self.z && self.z != self.x
    }

    pub fn is_isosceles(&self) -> bool {
        self.x == self.y || self.y == self.z || self.z == self.x
    }

    /// The case where the sum of the lengths of two sides equals that of the third
    /// is known as a degenerate triangle - it has zero area and looks like a single line
    pub fn is_degenerate(&self) -> bool {
        self.x + self.y == self.z || self.y + self.z == self.x || self.z + self.x == self.y
    }
}
