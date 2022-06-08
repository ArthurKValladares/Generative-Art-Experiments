use math::{mat::Mat4, vec::Vec3};

fn new_orthographic_proj(
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
    near: f32,
    far: f32,
) -> Mat4 {
    let w_inv = 1.0 / (right - left);
    let h_inv = 1.0 / (bottom - top);
    let d_inv = 1.0 / (far - near);
    Mat4::from_data(
        2.0 * w_inv,
        0.0,
        0.0,
        -(right + left) * w_inv,
        //
        0.0,
        2.0 * h_inv,
        0.0,
        -(bottom + top) * h_inv,
        //
        0.0,
        0.0,
        d_inv,
        -near * d_inv,
        //
        0.0,
        0.0,
        1.0,
        0.0,
    )
}

fn new_infinite_perspective_proj(aspect_ratio: f32, y_fov: f32, z_near: f32) -> Mat4 {
    let g = 1.0 / (y_fov * 0.5).tan();
    let e = 1.0 - 10e-6;
    Mat4::from_data(
        g / aspect_ratio,
        0.0,
        0.0,
        0.0,
        //
        0.0,
        g,
        0.0,
        0.0,
        //
        0.0,
        0.0,
        e,
        -z_near * e,
        //
        0.0,
        0.0,
        1.0,
        0.0,
    )
}

fn look_at(eye: Vec3, at: Vec3, world_up: Vec3) -> Mat4 {
    let forward = (at - eye).normalized();
    let right = world_up.cross(&forward).normalized();
    let up = right.cross(&forward);
    // TODO: Still wrong
    Mat4::from_data(
        right.x(),
        right.y(),
        right.z(),
        0.0,
        up.x(),
        up.y(),
        up.z(),
        0.0,
        forward.x(),
        forward.y(),
        forward.z(),
        0.0,
        eye.x(),
        eye.y(),
        eye.z(),
        1.0,
    )
}

#[derive(Debug)]
pub struct OrtographicData {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub near: f32,
    pub far: f32,
}

impl Default for OrtographicData {
    fn default() -> Self {
        Self {
            left: -5.0,
            right: 5.0,
            top: -5.0,
            bottom: 5.0,
            near: 0.0,
            far: 100.0,
        }
    }
}
#[derive(Debug)]
pub struct PerspectiveData {
    pub aspect_ratio: Option<f32>,
    pub y_fov: f32,
    pub z_near: f32,
    pub z_far: Option<f32>,
}

impl Default for PerspectiveData {
    fn default() -> Self {
        Self {
            aspect_ratio: None,
            y_fov: 20.0,
            z_near: 0.01,
            z_far: None,
        }
    }
}

#[derive(Debug)]
pub enum CameraType {
    Orthographic(OrtographicData),
    Perspective(PerspectiveData),
}

impl CameraType {
    pub fn ortographic(data: OrtographicData) -> Self {
        Self::Orthographic(data)
    }

    pub fn perspective(data: PerspectiveData) -> Self {
        Self::Perspective(data)
    }

    pub fn projection(&self, window_width: f32, window_height: f32) -> CameraProjection {
        match self {
            Self::Orthographic(data) => CameraProjection::new_orthographic(data),
            Self::Perspective(data) => {
                CameraProjection::new_perspective(data, window_width, window_height)
            }
        }
    }
}

#[derive(Debug)]
pub enum CameraProjection {
    Orthographic(glam::Mat4),
    Perspective(glam::Mat4),
}

impl CameraProjection {
    pub fn new_orthographic(data: &OrtographicData) -> Self {
        Self::Orthographic(glam::Mat4::orthographic_rh(
            data.left,
            data.right,
            data.bottom,
            data.top,
            data.near,
            data.far,
        ))
    }

    pub fn new_perspective(data: &PerspectiveData, window_width: f32, window_height: f32) -> Self {
        let aspect_ratio = data.aspect_ratio.unwrap_or(window_width / window_height);
        let mat = glam::Mat4::perspective_infinite_rh(data.y_fov, aspect_ratio, data.z_near);
        Self::Perspective(mat)
    }

    pub fn to_raw_matrix(self) -> glam::Mat4 {
        match self {
            CameraProjection::Orthographic(mat) => mat,
            CameraProjection::Perspective(mat) => mat,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CameraMatrices {
    view: glam::Mat4,
    proj: glam::Mat4,
}

#[derive(Debug)]
pub struct Camera {
    pos: Vec3,
    front: Vec3,
    ty: CameraType,
}

impl Camera {
    pub fn from_type(ty: CameraType) -> Self {
        Self {
            pos: Default::default(),
            front: Vec3::new(1.0, 0.0, 0.0),
            ty,
        }
    }

    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }

    pub fn update_position(&mut self, translation: Vec3) {
        self.pos += translation;
    }

    pub fn get_matrices(&self, window_width: f32, window_height: f32) -> CameraMatrices {
        // TODO: This can be better later, have a from vector instead of looking at 0,0,0
        let eye = glam::Vec3::new(self.pos.x(), self.pos.y(), self.pos.z());
        let front = glam::Vec3::new(self.front.x(), self.front.y(), self.front.z());
        let view = glam::Mat4::look_at_rh(
            eye,
            eye + front,
            glam::Vec3::new(0.0, 1.0, 0.0),
        );
        let proj = self
            .ty
            .projection(window_width, window_height)
            .to_raw_matrix();
        CameraMatrices { view, proj }
    }
}
