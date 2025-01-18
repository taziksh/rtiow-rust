mod ray;

#[derive(Debug, Clone, Copy)]
pub struct hit_record {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
}

