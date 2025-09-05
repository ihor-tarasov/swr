mod clip;
mod frame;

pub use frame::Frame;
pub use glam::{FloatExt, Mat3, Mat4, Quat, Vec2, Vec3, Vec4, mat3, mat4, quat, vec2, vec3, vec4};

const MAX_VARYINGS: usize = 10;

pub trait Varying {
    fn lerp(&self, other: &Self, ratio: f32) -> Self;
}

pub const fn f32_to_u8(value: f32) -> u8 {
    (value * 255.0) as u8
}

pub const fn u8_to_f32(value: u8) -> f32 {
    (value as f32) / 255.0
}

/// For facing determination, see subsection 3.5.1 of
/// https://www.khronos.org/registry/OpenGL/specs/es/2.0/es_full_spec_2.0.pdf
///
/// This is the same as (but more efficient than)
///     let ab = b - a;
///     let ac = c - a;
///     ab.cross(ac).z <= 0
fn is_back_facing(ndc_coords: [Vec3; 3]) -> bool {
    let a = ndc_coords[0];
    let b = ndc_coords[1];
    let c = ndc_coords[2];
    let signed_area = a.x * b.y - a.y * b.x + b.x * c.y - b.y * c.x + c.x * a.y - c.y * a.x;
    signed_area <= 0.0
}

fn viewport_transform(width: i32, height: i32, ndc_coord: Vec3) -> Vec3 {
    // [-1, 1] -> [0, w]
    let x = (ndc_coord.x + 1.0) * 0.5 * width as f32;
    // [-1, 1] -> [0, h]
    let y = (ndc_coord.y + 1.0) * 0.5 * height as f32;
    // [-1, 1] -> [0, 1]
    let z = (ndc_coord.z + 1.0) * 0.5;
    vec3(x, y, z)
}

struct BoundingBox {
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
}

impl BoundingBox {
    fn find(abc: [Vec2; 3], width: i32, height: i32) -> Self {
        let min = abc[0].min(abc[1]).min(abc[2]);
        let max = abc[0].max(abc[1]).max(abc[2]);
        Self {
            min_x: (min.x.floor() as i32).max(0),
            min_y: (min.y.floor() as i32).max(0),
            max_x: (max.x.ceil() as i32).min(width - 1),
            max_y: (max.y.ceil() as i32).min(height - 1),
        }
    }
}

/// For barycentric coordinates, see
/// http://blackpawn.com/texts/pointinpoly/
///
/// solve
///     P = A + s * AB + t * AC  -->  AP = s * AB + t * AC
/// then
///     s = (AC.y * AP.x - AC.x * AP.y) / (AB.x * AC.y - AB.y * AC.x)
///     t = (AB.x * AP.y - AB.y * AP.x) / (AB.x * AC.y - AB.y * AC.x)
///
/// notice
///     P = A + s * AB + t * AC
///       = A + s * (B - A) + t * (C - A)
///       = (1 - s - t) * A + s * B + t * C
/// then
///     weight_A = 1 - s - t
///     weight_B = s
///     weight_C = t
fn calculate_weights(abc: [Vec2; 3], p: Vec2) -> Vec3 {
    let a = abc[0];
    let b = abc[1];
    let c = abc[2];
    let ab = b - a;
    let ac = c - a;
    let ap = p - a;
    let factor = 1.0 / (ab.x * ac.y - ab.y * ac.x);
    let s = (ac.y * ap.x - ac.x * ap.y) * factor;
    let t = (ab.x * ap.y - ab.y * ap.x) * factor;
    vec3(1.0 - s - t, s, t)
}

/// For depth interpolation, see subsection 3.5.1 of
/// https://www.khronos.org/registry/OpenGL/specs/es/2.0/es_full_spec_2.0.pdf
fn interpolate_depth(screen_depths: [f32; 3], weights: Vec3) -> f32 {
    let depth0 = screen_depths[0] * weights.x;
    let depth1 = screen_depths[1] * weights.y;
    let depth2 = screen_depths[2] * weights.z;
    depth0 + depth1 + depth2
}
