use glam::*;

use crate::f32_to_u8;

pub struct Frame<'a> {
    width: u32,
    height: u32,
    color: &'a mut [u32],
    depth: Option<&'a mut [f32]>,
}

impl<'a> Frame<'a> {
    pub fn new(
        width: u32,
        height: u32,
        color: &'a mut [u32],
        mut depth: Option<&'a mut [f32]>,
    ) -> Self {
        debug_assert!(color.len() >= width as usize * height as usize);
        if let Some(depth) = depth.as_deref_mut() {
            debug_assert!(depth.len() >= width as usize * height as usize);
        }
        Self {
            width,
            height,
            color,
            depth,
        }
    }

    pub fn clear(&mut self, color: Option<Vec4>, depth: Option<f32>) {
        if let Some(color) = color {
            let color = u32::from_le_bytes([
                f32_to_u8(color.z),
                f32_to_u8(color.y),
                f32_to_u8(color.x),
                f32_to_u8(color.w),
            ]);
            self.color.fill(color);
        }
        if let (Some(depth_buffer), Some(depth)) = (self.depth.as_deref_mut(), depth) {
            depth_buffer.fill(depth);
        }
    }
}
