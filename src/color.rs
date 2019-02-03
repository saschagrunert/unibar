//! Main color scheme definitions and structures

pub struct ColorScheme;

#[allow(dead_code)]
impl ColorScheme {
    pub const fn black() -> [f32; 4] {
        [0.05, 0.05, 0.05, 1.]
    }

    pub const fn background() -> [f32; 4] {
        [0.156, 0.164, 0.211, 1.]
    }

    pub const fn cyan() -> [f32; 4] {
        [0.545, 0.913, 0.992, 1.]
    }

    pub const fn foreground() -> [f32; 4] {
        [0.972, 0.972, 0.949, 1.]
    }

    pub const fn green() -> [f32; 4] {
        [0.313, 0.98, 0.482, 1.]
    }

    pub const fn orange() -> [f32; 4] {
        [1., 0.721, 0.423, 1.]
    }

    pub const fn pink() -> [f32; 4] {
        [1., 0.474, 0.776, 1.]
    }

    pub const fn purple() -> [f32; 4] {
        [0.741, 0.576, 0.976, 1.]
    }

    pub const fn red() -> [f32; 4] {
        [1., 0.333, 0.333, 1.]
    }

    pub const fn selection() -> [f32; 4] {
        [0.384, 0.447, 0.643, 1.]
    }

    pub const fn yellow() -> [f32; 4] {
        [0.945, 0.98, 0.549, 1.]
    }
}
