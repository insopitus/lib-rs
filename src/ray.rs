use crate::linear_algebra::Vector3;


pub struct Ray{
    origin:Vector3,
    direction:Vector3
}
impl Ray{
    pub fn new(origin:Vector3, direction:Vector3)->Self{
        Self{
            origin,
            direction
        }
    }
    pub fn at(&self,distance:f32)->Vector3{
        self.origin+distance*self.direction
    }
}