use kiss3d::{nalgebra::Point3, window::Window};

pub struct WireCube {
    a: Point3<f32>,
    b: Point3<f32>,
    c: Point3<f32>,
    d: Point3<f32>,
    e: Point3<f32>,
    f: Point3<f32>,
    g: Point3<f32>,
    h: Point3<f32>,
    color: Point3<f32>,
}
impl WireCube {
    pub fn new(x: f32, y: f32, z: f32, size: f32, color_r: f32, color_g: f32, color_b: f32) -> Self {
        let a = Point3::new(x, y, z);
        let b = Point3::new(x + size, y, z);
        let c = Point3::new(x, y, z + size);
        let d = Point3::new(x + size, y, z + size);
        let e = Point3::new(x, y + size, z);
        let f = Point3::new(x + size, y + size, z);
        let g = Point3::new(x, y + size, z + size);
        let h = Point3::new(x + size, y + size, z + size);
        let color = Point3::new(color_r, color_g, color_b);
        
        Self { a, b, c, d, e, f, g, h, color }
    }

    pub fn draw(&self, window: &mut Window) {
        window.draw_line(&self.a, &self.b, &self.color);
        window.draw_line(&self.a, &self.c, &self.color);
        window.draw_line(&self.b, &self.d, &self.color);
        window.draw_line(&self.c, &self.d, &self.color);
        window.draw_line(&self.e, &self.f, &self.color);
        window.draw_line(&self.e, &self.g, &self.color);
        window.draw_line(&self.f, &self.h, &self.color);
        window.draw_line(&self.g, &self.h, &self.color);
        window.draw_line(&self.a, &self.e, &self.color);
        window.draw_line(&self.b, &self.f, &self.color);
        window.draw_line(&self.c, &self.g, &self.color);
        window.draw_line(&self.d, &self.h, &self.color);
    }
    
}