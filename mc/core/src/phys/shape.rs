

pub struct Vertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vertex {

    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

}


pub struct AABB {
    from: Vertex,
    to: Vertex
}

impl AABB {

    pub const fn new(x0: f64, y0: f64, z0: f64, x1: f64, y1: f64, z1: f64) -> Self {
        Self {
            from: Vertex { x: x0, y: y0, z: z0 },
            to: Vertex { x: x1, y: y1, z: z1 }
        }
    }

}


pub enum Shape {
    Empty,
    Cube(AABB),
    Or(&'static Shape, &'static Shape)
}

impl Shape {

    pub const fn new_cube(x0: f64, y0: f64, z0: f64, x1: f64, y1: f64, z1: f64) -> Self {
        Self::Cube(AABB::new(x0, y0, z0, x1, y1, z1))
    }

}


pub static Y_AXIS_AABB: Shape = Shape::new_cube(6.5, 0.0, 6.5, 9.5, 16.0, 9.5);
pub static Z_AXIS_AABB: Shape = Shape::new_cube(6.5, 6.5, 0.0, 9.5, 9.5, 16.0);
pub static X_AXIS_AABB: Shape = Shape::new_cube(0.0, 6.5, 6.5, 16.0, 9.5, 9.5);
pub static JOINED: Shape = Shape::Or(&X_AXIS_AABB, &Z_AXIS_AABB);