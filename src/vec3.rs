use std::ops;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 {e: [e0, e1, e2]}
    }

    pub fn x(&self) -> f64 { self.e[0] }
    pub fn y(&self) -> f64 { self.e[1] }
    pub fn z(&self) -> f64 { self.e[2] }

    pub fn length(&self) -> f64 {
        (self.e[0].powi(2) + self.e[1].powi(2) + self.e[2].powi(2)).sqrt()
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.e[0] * other.e[0] + 
        self.e[1] * other.e[1] + 
        self.e[2] * other.e[2]
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 { e: [
            self.e[1] * other.e[2] - self.e[2] * other.e[1],
            self.e[2] * other.e[0] - self.e[0] * other.e[2],
            self.e[0] * other.e[1] - self.e[1] * other.e[0],
        ] }
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self { e: [0.0, 0.0, 0.0] }
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self { e: [-self.e[0], -self.e[1], -self.e[2]] }
    }
}

//TODO
// double operator[](int i) const { return e[i]; }
// double& operator[](int i) { return e[i]; }

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ]
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ]
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, t: f64) -> Self {
        Self {
            e: [self.e[0] * t, self.e[1] * t, self.e[2] * t]
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}
impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, t: f64) -> Self {
        Self { e: [self.e[0] / t, self.e[1] / t, self.e[2] / t] }
    }
}

//TODO
// inline std::ostream& operator<<(std::ostream& out, const vec3& v) {
//     return out << v.e[0] << ' ' << v.e[1] << ' ' << v.e[2];
// }
