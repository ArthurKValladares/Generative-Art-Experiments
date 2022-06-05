use math::mat::Mat4;

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

pub fn new_perspective_proj(aspect_ratio: f32, y_fov: f32, z_near: f32, z_far: f32) -> Mat4 {
    let g = 1.0 / (y_fov * 0.5).tan();
    let k = z_far / (z_far - z_near);

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
        k,
        -z_near * k,
        //
        0.0,
        0.0,
        1.0,
        0.0,
    ])
}

pub fn new_infinite_perspective_proj(aspect_ratio: f32, y_fov: f32, z_near: f32) -> Mat4 {
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
    pub fn build(&self, window_width: f32, window_height: f32) -> Camera {
        match self {
            Self::Orthographic(data) => Camera::new_orthographic(data),
            Self::Perspective(data) => Camera::new_perspective(data, window_width, window_height),
        }
    }
}

impl Default for CameraType {
    fn default() -> Self {
        Self::Orthographic(OrtographicData {
            left: 0.0,
            right: 1.0,
            top: 1.0,
            bottom: 0.0,
            near: 0.0,
            far: 1.0,
        })
    }
}

pub enum Camera {
    Orthographic(Mat4),
    Perspective(Mat4),
}

impl Camera {
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
        let mat = if let Some(z_far) = data.z_far {
            new_perspective_proj(aspect_ratio, data.y_fov, data.z_near, z_far)
        } else {
            new_infinite_perspective_proj(aspect_ratio, data.y_fov, data.z_near)
        };
        Self::Perspective(mat)
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
        CameraType::default().build(1000.0, 1000.0)
    }
}
