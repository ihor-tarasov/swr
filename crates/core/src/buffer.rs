use glam::*;

use crate::f32_to_u8;

pub struct ColorBuffer<'a> {
    width: u32,
    height: u32,
    data: &'a mut [u32],
}

impl<'a> ColorBuffer<'a> {
    pub const fn new(width: u32, height: u32, data: &'a mut [u32]) -> Self {
        debug_assert!(data.len() >= width as usize * height as usize);
        Self {
            width,
            height,
            data,
        }
    }

    pub fn clear(&mut self, color: Vec4) {
        let color = u32::from_le_bytes([
            f32_to_u8(color.z),
            f32_to_u8(color.y),
            f32_to_u8(color.x),
            f32_to_u8(color.w),
        ]);
        self.data.fill(color);
    }
}
