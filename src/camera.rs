use crate::kmath::Vec3;
use glam::Mat4;
use std::f32::consts::PI;

const LOOK_SENSITIVITY: f32 = 0.001;

pub struct Camera {
    pub aspect: f32,
    pub fovx: f32,  // is actually half fov
    pub pos: Vec3,
    pub dir: Vec3,
    pub up: Vec3,

    pub yaw: f32,
    pub pitch: f32,
}

impl Camera {
    pub fn new(fovx: f32, aspect: f32, start_pos: Vec3) -> Camera {
        let start_dir = Vec3::new(0.0, 0.0, 1.0);
        let start_pitch = 0.0;
        let start_yaw = 0.0;
        let up = Vec3::new(0.0, 1.0, 0.0);

        Camera {
            aspect, fovx, pos: start_pos, dir: start_dir, up, pitch: start_pitch, yaw: start_yaw,
        }
    }

    pub fn vp_up(&self) -> Vec3 {
        let right = self.dir.cross(self.up).normalize();
        self.dir.cross(right)
    }

    pub fn projection_mat(&self) -> Mat4 {
        let znear = 0.05;
        let zfar = 1000.0;
        Mat4::perspective_lh(self.fovx, self.aspect, znear, zfar)
    }

    pub fn view_mat(&self) -> Mat4 {
        let pos = glam::Vec3::new(self.pos.x, self.pos.y, self.pos.z);
        let dir = glam::Vec3::new(self.dir.x, self.dir.y, self.dir.z);
        let up = glam::Vec3::new(self.up.x, self.up.y, self.up.z);

        Mat4::look_at_lh(pos, pos + dir, up)
    }

    pub fn view_mat_nomove(&self) -> Mat4 {
        let pos = glam::Vec3::new(0.0, 0.0, 0.0);
        let dir = glam::Vec3::new(self.dir.x, self.dir.y, self.dir.z);
        let up = glam::Vec3::new(self.up.x, self.up.y, self.up.z);

        Mat4::look_at_lh(pos, pos + dir, up)
    }

    pub fn update_y(&mut self, amount: f32) {
        self.pos.y += amount
    }

    pub fn update_x(&mut self, amount: f32) {
        let movt_dir = -Vec3::new(self.dir.x, 0.0, self.dir.z).normalize().cross(self.up);
        self.pos += movt_dir * amount;
    }

    pub fn update_z(&mut self, amount: f32) {
        let movt_dir = Vec3::new(self.dir.x, 0.0, self.dir.z).normalize();
        self.pos += movt_dir * amount;
    }

    /*
    pub fn update_look_x(&mut self, amount: f32) {
        self.yaw = (self.yaw + LOOK_SENSITIVITY*amount as f32 + 2.0*PI) % (2.0*PI);

        let rotation_mat = Mat4::from_rotation_y(self.yaw) * Mat4::from_rotation_x(self.pitch);
        let dir = rotation_mat.transform_vector3(glam::Vec3::new(0.0, 0.0, 1.0));
        self.dir = Vec3::new(dir.x, dir.y, dir.z);
    }

    pub fn update_look_y(&mut self, amount: f32) {
        self.pitch = self.pitch + LOOK_SENSITIVITY * amount as f32;
        let safety = 0.001;
        if self.pitch < (-PI/2.0 + safety) {
            self.pitch = (-PI/2.0 + safety);
        }
        if self.pitch > (PI/2.0 - safety) {
            self.pitch = (PI/2.0 - safety);
        }
    }
    */

    pub fn update_look(&mut self, x: f32, y: f32) {
        let sensitivity = 0.001f32;
        let right = self.dir.cross(self.up).normalize();

        self.dir = self.dir.rotate_about_vec3(self.up, x*sensitivity);

        use std::f32::consts::FRAC_PI_2;
        let safety = 0.001;
        
        let desired_pitch_amount = -y*sensitivity;
        let actual_final_pitch = (self.pitch + desired_pitch_amount).clamp(-FRAC_PI_2 + safety, FRAC_PI_2 - safety);
        let actual_pitch_amount = actual_final_pitch - self.pitch;

        self.pitch = actual_final_pitch;
        self.dir = self.dir.rotate_about_vec3(right, actual_pitch_amount);
    }

    // can probably cache these baddies hey
    pub fn point_in_vision(&self, p: Vec3) -> bool {
        let vp_up = self.vp_up();
        let right = self.dir.cross(self.up).normalize();
        let fovy = self.fovx / self.aspect;

        let  nbot = -self.dir.rotate_about_vec3(right, -fovy).cross(right).normalize();
        let  ntop = self.dir.rotate_about_vec3(right, fovy).cross(right).normalize();
        let  nleft = -self.dir.rotate_about_vec3(vp_up, -self.fovx).cross(vp_up).normalize();
        let  nright = self.dir.rotate_about_vec3(vp_up, self.fovx).cross(vp_up).normalize();

        let frustum_cull_safety_factor = -10.0;
        nbot.dot(p - self.pos) > frustum_cull_safety_factor &&
        ntop.dot(p - self.pos) > frustum_cull_safety_factor &&
        nleft.dot(p - self.pos) > frustum_cull_safety_factor &&
        nright.dot(p - self.pos) > frustum_cull_safety_factor
    }
}