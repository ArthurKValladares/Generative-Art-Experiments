use math::{mat::Mat4, vec::Vec3};

fn new_orthographic_proj(
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
    near: f32,
    far: f32,
) -> Mat4 {
    Mat4::from_rows_array([
        2.0 / (right - left),
        0.0,
        0.0,
        -(right + left) / (right - left),
        //
        0.0,
        2.0 / (bottom - top),
        0.0,
        -(bottom + top) / (bottom - top),
        //
        0.0,
        0.0,
        1.0 / (far - near),
        -(near) / (far - near),
        //
        0.0,
        0.0,
        0.0,
        1.0,
    ])
}

fn new_infinite_perspective_proj(aspect_ratio: f32, y_fov: f32, z_near: f32) -> Mat4 {
    let g = 1.0 / (y_fov * 0.5).tan();
    let e = 1.0 - 10e-6;
    Mat4::from_rows_array([
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
    ])
}

fn look_at(eye: Vec3, at: Vec3, up: Vec3) -> Mat4 {
    let z_axis = (at - eye).normalized();
    let x_axis = z_axis.cross(&up).normalized();
    let y_axis = x_axis.cross(&z_axis);
    let z_axis = z_axis.negate();

    Mat4::from_rows_array([
        x_axis.x(),
        x_axis.y(),
        x_axis.z(),
        -x_axis.dot(&eye), //
        y_axis.x(),
        y_axis.y(),
        y_axis.z(),
        -y_axis.dot(&eye), //
        z_axis.x(),
        z_axis.y(),
        z_axis.z(),
        -z_axis.dot(&eye), //
        0.0,
        0.0,
        0.0,
        1.0,
    ])
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

#[derive(Debug)]
pub struct PerspectiveData {
    pub aspect_ratio: Option<f32>,
    pub y_fov: f32,
    pub z_near: f32,
    pub z_far: Option<f32>,
}

#[derive(Debug)]
pub enum CameraType {
    Orthographic(OrtographicData),
    Perspective(PerspectiveData),
}

impl CameraType {
    pub fn projection(&self, window_width: f32, window_height: f32) -> CameraProjection {
        match self {
            Self::Orthographic(data) => CameraProjection::new_orthographic(data),
            Self::Perspective(data) => {
                CameraProjection::new_perspective(data, window_width, window_height)
            }
        }
    }
}

impl Default for CameraType {
    fn default() -> Self {
        Self::Orthographic(OrtographicData {
            left: -5.0,
            right: 5.0,
            top: 5.0,
            bottom: -5.0,
            near: 0.0,
            far: 1.0,
        })
    }
}

#[derive(Debug)]
pub enum CameraProjection {
    Orthographic(Mat4),
    Perspective(Mat4),
}

impl CameraProjection {
    pub fn new_orthographic(data: &OrtographicData) -> Self {
        Self::Orthographic(new_orthographic_proj(
            data.left,
            data.right,
            data.top,
            data.bottom,
            data.near,
            data.far,
        ))
    }

    pub fn new_perspective(data: &PerspectiveData, window_width: f32, window_height: f32) -> Self {
        let aspect_ratio = data.aspect_ratio.unwrap_or(window_width / window_height);
        let mat = new_infinite_perspective_proj(aspect_ratio, data.y_fov, data.z_near);
        Self::Perspective(mat)
    }

    pub fn to_raw_matrix(self) -> Mat4 {
        match self {
            CameraProjection::Orthographic(mat) => mat,
            CameraProjection::Perspective(mat) => mat,
        }
    }
}

impl Default for CameraProjection {
    fn default() -> Self {
        CameraType::default().projection(1000.0, 1000.0)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CameraMatrices {
    view: Mat4,
    proj: Mat4,
}

#[derive(Debug)]
pub struct Camera {
    pos: Vec3,
    ty: CameraType,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            pos: Default::default(),
            ty: Default::default(),
        }
    }
}

impl Camera {
    pub fn from_type(ty: CameraType) -> Self {
        Self {
            pos: Default::default(),
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
        let view = look_at(self.pos, Vec3::zero(), Vec3::new(0.0, 1.0, 0.0));
        let proj = self
            .ty
            .projection(window_width, window_height)
            .to_raw_matrix();
        CameraMatrices { view, proj }
    }
}
