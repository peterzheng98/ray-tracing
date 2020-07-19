use std::ops;

#[derive(Clone, Copy, Debug)]
pub struct vec3d{
    pub x: f32, pub y: f32, pub z: f32,
}

impl vec3d{
    pub fn initial(&mut self){
        self.x = 0.0;
        self.y = 0.0;
        self.z = 0.0;
    }
    pub fn x(&mut self) -> f32{
        self.x
    }
    pub fn y(&mut self) -> f32{
        self.y
    }
    pub fn z(&mut self) -> f32{
        self.z
    }

    pub fn length(&mut self) -> f32{
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unitVec(&mut self) -> vec3d{
        vec3d {
            x: self.x / self.length().sqrt(), 
            y: self.y / self.length().sqrt(), 
            z: self.z / self.length().sqrt()
        }
    }
}

impl ops::Add for vec3d {
    type Output = vec3d;

    fn add(self, _rhs: vec3d) -> vec3d {
        vec3d {x: self.x + _rhs.x, y: self.y + _rhs.y, z: self.z + _rhs.z}
    }
}
impl ops::Sub for vec3d {
    type Output = vec3d;

    fn sub(self, _rhs: vec3d) -> vec3d {
        vec3d {x: self.x - _rhs.x, y: self.y - _rhs.y, z: self.z - _rhs.z}
    }
}

impl ops::Mul<f32> for vec3d{
    type Output = vec3d;
    fn mul(self, _rhs: f32) -> vec3d{
        vec3d {x: self.x * _rhs, y: self.y * _rhs, z: self.z * _rhs}
    }
}

impl ops::Mul<vec3d> for f32{
    type Output = vec3d;
    fn mul(self, _rhs: vec3d) -> vec3d{
        vec3d {x: self * _rhs.x, y: self * _rhs.y, z: self * _rhs.z}
    }
}

impl ops::Div<f32> for vec3d{
    type Output = vec3d;
    fn div(self, _rhs: f32) -> vec3d{
        return vec3d{x: self.x as f32 / _rhs, y: self.y as f32 / _rhs, z: self.z as f32 / _rhs};
    }
}