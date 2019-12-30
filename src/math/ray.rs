use super::vec::v3;

#[derive(Debug, Clone, Copy, Default)]
pub struct Ray {
    pub origin: v3,
    pub dir: v3
}

#[allow(dead_code)]
impl Ray {
    pub fn make(p: v3, d: v3) -> Ray {
        Ray {
            origin: p,
            dir: d
        }
    }

    pub fn travel(self, t: f32) -> v3 {
        self.origin + self.dir * t
    }
}
