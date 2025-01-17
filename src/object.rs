use cgmath::Vector3;

pub struct Object
{
    position: Vector3<f32>,
    vertices: Vec<f32>
}

impl Object {
    pub fn new(position: Vector3<f32>, vertices: Vec<f32>) -> Object
    {
        Object{
            position,
            vertices
        }
    }
}