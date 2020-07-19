use crate::Vec3::vec3d;

pub struct ray{
    pub point: vec3d, pub direction: vec3d
}

impl ray{
    pub fn at(&mut self, t: f32) -> vec3d{
        return self.point + t * self.direction;
    }
}




