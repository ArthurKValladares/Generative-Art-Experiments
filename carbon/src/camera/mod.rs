use math::mat::Mat4;

// TODO: Perspective camera

pub fn new_ortographic_proj(
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
        0.0,
        //
        0.0,
        2.0 / (bottom - top),
        0.0,
        0.0,
        //
        0.0,
        0.0,
        1.0 / (far - near),
        0.0,
        //
        -(right + left) / (right - left),
        -(bottom + top) / (bottom - top),
        -(near) / (far - near),
        1.0,
    ])
}

pub struct OrtographicData {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub near: f32,
    pub far: f32,
}

pub enum CameraType {
    Ortographic(OrtographicData),
    Perspective,
}

impl CameraType {
    pub fn build(self) -> Camera {
        match self {
            Self::Ortographic(data) => Camera::new_ortographic(data),
            Self::Perspective => todo!(),
        }
    }
}

pub enum Camera {
    Ortographic(Mat4),
    Perspective(Mat4),
}

impl Camera {
    pub fn new_ortographic(data: OrtographicData) -> Self {
        Self::Ortographic(new_ortographic_proj(
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
            Camera::Ortographic(mat) => mat,
            Camera::Perspective(mat) => mat,
        }
    }
}
