use kiss3d::{camera::FirstPerson, nalgebra::Point3};

const MAX_EYE_Z: f32 = 100.0;
const MIN_EYE_Z: f32 = -100.0;
const MAX_EYE_X: f32 = 100.0;
const MIN_EYE_X: f32 = -100.0;

pub struct Camera{
    eye_x: f32,
    eye_y: f32,
    eye_z: f32,
    at: Point3<f32>,
    fp_camera: FirstPerson
}
impl Camera{
    pub fn new(eye_x: f32, eye_y: f32, eye_z: f32, at_x: f32, at_y: f32, at_z: f32) -> Self {
        let at = Point3::new(at_x, at_y, at_z);
        let fp_camera = FirstPerson::new(
            Point3::new(eye_x, eye_y, eye_z),
            at
        );

        Self{
            eye_x,
            eye_y,
            eye_z,
            at,
            fp_camera
        }
    }

    fn update_camera(&mut self) {
        self.fp_camera = FirstPerson::new(
            Point3::new(self.eye_x, self.eye_y, self.eye_z),
            self.at
        );
    }

    pub fn move_x(&mut self, x: f32) {
        self.eye_x += x;
        self.eye_x = self.eye_x.max(MIN_EYE_X).min(MAX_EYE_X);
        self.update_camera();
    }

    pub fn move_z(&mut self, z: f32) {
        self.eye_z += z;
        self.eye_z = self.eye_z.max(MIN_EYE_Z).min(MAX_EYE_Z);
        self.update_camera();
    }

    pub fn get_fp(&mut self) -> &mut FirstPerson {
        &mut self.fp_camera
    } 
}