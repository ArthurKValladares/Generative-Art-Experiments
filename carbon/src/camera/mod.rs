use math::mat::Mat4;

// TODO: Perspective camera

pub fn new_orthographic_proj(
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
    pub z_far: Option<f32>,
    pub z_near: f32,
}

#[derive(Debug)]
pub enum CameraType {
    Orthographic(OrtographicData),
    Perspective(PerspectiveData),
}

impl CameraType {
    pub fn build(self) -> Camera {
        match self {
            Self::Orthographic(data) => Camera::new_orthographic(data),
            Self::Perspective(_data) => todo!(),
        }
    }
}

pub enum Camera {
    Orthographic(Mat4),
    Perspective(Mat4),
}

impl Camera {
    pub fn new_orthographic(data: OrtographicData) -> Self {
        Self::Orthographic(new_orthographic_proj(
            data.left,
            data.right,
            data.top,
            data.bottom,
            data.near,
            data.far,
        ))
    }

    pub fn raw_matrix(&self) -> &Mat4 {
        match self {
            Camera::Orthographic(mat) => mat,
            Camera::Perspective(mat) => mat,
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new_orthographic(OrtographicData {
            left: 0.0,
            right: 1.0,
            top: 1.0,
            bottom: 0.0,
            near: 0.0,
            far: 1.0,
        })
    }
}