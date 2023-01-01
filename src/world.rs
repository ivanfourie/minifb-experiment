use crate::vector::{ Vec2, Vec3 };
use crate::display::{ Display, Drawable };

const N_POINTS: usize = 9 * 9 * 9;

pub struct World {
    cube_points: [Vec3; N_POINTS],
    projected_points: [Vec2; N_POINTS],
    camera_position: Vec3,
    cube_rotation: Vec3,
    fov_factor: f32,
}

impl World {
    pub fn new() -> Result<Self, String> {
        let mut point_count = 0;
        let mut cube_points = [Vec3 { x: 0.0, y: 0.0, z: 0.0 }; N_POINTS];

        let mut x = -1.0;
        let mut y = -1.0;
        let mut z = -1.0;

        while x <= 1.0 {
            while y <= 1.0 {
                while z <= 1.0 {
                    let new_point = Vec3 { x, y, z };
                    cube_points[point_count] = new_point;
                    point_count += 1;
                    z += 0.25;
                }
                z = -1.0;
                y += 0.25;
            }
            y = -1.0;
            x += 0.25
        }

        Ok(World {
            cube_points,
            projected_points: [Vec2 { x: 0.0, y: 0.0 }; N_POINTS],
            camera_position: Vec3 { x: 0.0, y: 0.0, z: -5.0 },
            cube_rotation: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            fov_factor: 640.0,
        })
    }

    pub fn project(&mut self, point: Vec3) -> Vec2 {
        Vec2 {
            x: (self.fov_factor * point.x) / point.z,
            y: (self.fov_factor * point.y) / point.z,
        }
    }

    pub fn update(&mut self) {
        self.cube_rotation.x += 0.01;
        self.cube_rotation.y += 0.01;
        self.cube_rotation.z += 0.01;

        for i in 0..N_POINTS {
            let point = self.cube_points[i];

            let mut transformed_point = point.rotate_x(self.cube_rotation.x);
            transformed_point = transformed_point.rotate_y(self.cube_rotation.y);
            transformed_point = transformed_point.rotate_z(self.cube_rotation.z);

            // Translate the points away from the camera
            transformed_point.z -= self.camera_position.z;

            // Project the current point
            let projected_point = self.project(transformed_point);

            // Save the projected 2D vector in the array of projected points
            self.projected_points[i] = projected_point;
        }
    }

    pub fn render(&self, display: &mut Display) {
        display.clear_color_buffer(0xFF000000);
        display.draw_grid(10, 0xFF444444);
        
        for i in 0..N_POINTS {
            let point = self.projected_points[i];
            display.draw_rect(
                (point.x + (display.width as f32 / 2.0)) as i32,
                (point.y + (display.height as f32 / 2.0)) as i32,
                4,
                4,
                0xFFFFFF00
            );
        }
    }
}