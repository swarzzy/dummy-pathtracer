use std::ops::{Add, Sub, Mul, Neg, Div};

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct v3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[allow(dead_code)]
impl v3 {
    pub fn make(x: f32, y: f32, z: f32) -> v3 {
        v3 {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn ident() -> v3 {
        v3 { x: 1.0, y: 1.0, z: 1.0 }
    }

    pub fn len_sq(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(self) -> f32 {
        self.len_sq().sqrt()
    }

    pub fn normalize(self) -> v3 {
        let l = 1.0 / self.len();
        v3 {
            x: self.x * l,
            y: self.y * l,
            z: self.z * l
        }
    }

    pub fn dot(self, other: v3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: v3) -> v3 {
        v3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }

    pub fn hadamard(self, other: v3) -> v3 {
        v3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }

    pub fn reflect(self, n: v3) -> v3 {
        self - (self.dot(n) * 2.0 * n)
    }

}

impl Neg for v3 {
    type Output = Self;

    fn neg(self) -> Self {
        v3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for v3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Add<f32> for v3 {
    type Output = Self;

    fn add(self, scalar: f32) -> Self {
        Self {
            x: self.x + scalar,
            y: self.y + scalar,
            z: self.z + scalar
        }
    }
}

impl Sub for v3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Mul<f32> for v3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar
        }
    }
}

impl Div<f32> for v3 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar
        }
    }
}

impl Mul<v3> for f32 {
    type Output = v3;

    fn mul(self, vec: v3) -> v3 {
        v3 {
            x: self * vec.x,
            y: self * vec.y,
            z: self * vec.z
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_v3_test() {
        let mut vec = v3::default();
        let vec2 = vec;
        vec.x = 1.0;
        vec.y = 2.0;
        vec.z = 3.0;
        let vec3 = v3::make(4.0, 5.0, 6.0) + vec;
        println!("vec: {:?}", vec);
        println!("vec2: {:?}", vec2);
        println!("vec3: {:?}", vec3);
        println!("mul: {:?}", vec * 2.0);
        println!("mul: {:?}", 2.0 * vec);

        let cross = v3::make(1.0, 0.0, 0.0).cross(v3::make(0.0, 1.0, 0.0));
        assert!(cross.x == 0.0);
        assert!(cross.y == 0.0);
        assert!(cross.z == 1.0);

        cross.mutate();
        println!("cross: {:?}", cross);
    }
}
