use math::mat::Mat4;

// TODO: Perspective camera

#[derive(Default)]
pub struct WorldMatrix {
    _mat: Mat4,
}

#[derive(Default)]
pub struct ViewMatrix {
    _mat: Mat4,
}

#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ProjMatrix {
    mat: Mat4,
}

impl ProjMatrix {
    pub fn new_ortographic(
        left: f32,
        right: f32,
        top: f32,
        bottom: f32,
        near: f32,
        far: f32,
    ) -> Self {
        Self {
            mat: Mat4::from_rows_array([
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
            ]),
        }
    }

    pub fn to_camera_matrices(self) -> CameraMatrices {
        CameraMatrices { proj: self }
    }
}

#[repr(C)]
pub struct CameraMatrices {
    proj: ProjMatrix,
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
    Ortographic(ProjMatrix),
    Perspective(ProjMatrix),
}

impl Camera {
    pub fn new_ortographic(data: OrtographicData) -> Self {
        Self::Ortographic(ProjMatrix::new_ortographic(
            data.left,
            data.right,
            data.top,
            data.bottom,
            data.near,
            data.far,
        ))
    }

    pub fn projection_matrix(&self) -> &ProjMatrix {
        match self {
            Self::Ortographic(proj) => proj,
            Self::Perspective(proj) => proj,
        }
    }
}
