mod buffer;

pub use buffer::ColorBuffer;
pub use glam::{FloatExt, Mat3, Mat4, Quat, Vec2, Vec3, Vec4, mat3, mat4, quat, vec2, vec3, vec4};

pub const fn f32_to_u8(value: f32) -> u8 {
    (value * 255.0) as u8
}

pub const fn u8_to_f32(value: u8) -> f32 {
    (value as f32) / 255.0
}
