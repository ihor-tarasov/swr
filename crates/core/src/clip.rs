use std::f32::EPSILON;

use glam::*;

use crate::{MAX_VARYINGS, Varying};

// For triangle clipping, see
// http://fabiensanglard.net/polygon_codec/
// http://graphics.idav.ucdavis.edu/education/GraphicsNotes/Clipping.pdf

#[derive(Clone, Copy)]
enum Plane {
    PositiveW,
    PositiveX,
    NegativeX,
    PositiveY,
    NegativeY,
    PositiveZ,
    NegativeZ,
}

impl Plane {
    fn inside(self, coord: Vec4) -> bool {
        match self {
            Plane::PositiveW => coord.w >= EPSILON,
            Plane::PositiveX => coord.x <= coord.w,
            Plane::NegativeX => coord.x >= -coord.w,
            Plane::PositiveY => coord.y <= coord.w,
            Plane::NegativeY => coord.y >= -coord.w,
            Plane::PositiveZ => coord.z <= coord.w,
            Plane::NegativeZ => coord.z >= -coord.w,
        }
    }

    fn intersect_ratio(self, prev: Vec4, curr: Vec4) -> f32 {
        match self {
            Plane::PositiveW => (prev.w - EPSILON) / (prev.w - curr.w),
            Plane::PositiveX => (prev.w - prev.x) / ((prev.w - prev.x) - (curr.w - curr.x)),
            Plane::NegativeX => (prev.w + prev.x) / ((prev.w + prev.x) - (curr.w + curr.x)),
            Plane::PositiveY => (prev.w - prev.y) / ((prev.w - prev.y) - (curr.w - curr.y)),
            Plane::NegativeY => (prev.w + prev.y) / ((prev.w + prev.y) - (curr.w + curr.y)),
            Plane::PositiveZ => (prev.w - prev.z) / ((prev.w - prev.z) - (curr.w - curr.z)),
            Plane::NegativeZ => (prev.w + prev.z) / ((prev.w + prev.z) - (curr.w + curr.z)),
        }
    }

    fn clip<V>(
        self,
        in_num_vertices: usize,
        in_coords: &[Vec4; MAX_VARYINGS],
        in_varyings: &[V; MAX_VARYINGS],
        out_coords: &mut [Vec4; MAX_VARYINGS],
        out_varyings: &mut [V; MAX_VARYINGS],
    ) -> usize
    where
        V: Varying + Copy,
    {
        let mut out_num_vertices = 0;

        assert!(in_num_vertices >= 3 && in_num_vertices <= MAX_VARYINGS);

        for i in 0..in_num_vertices {
            let prev_index = (i - 1 + in_num_vertices) % in_num_vertices;
            let curr_index = i;
            let prev_coord = in_coords[prev_index];
            let curr_coord = in_coords[curr_index];
            let prev_varyings = &in_varyings[prev_index];
            let curr_varyings = &in_varyings[curr_index];
            let prev_inside = self.inside(prev_coord);
            let curr_inside = self.inside(curr_coord);

            if prev_inside != curr_inside {
                let dest_coord = &mut out_coords[out_num_vertices];
                let dest_varyings = &mut out_varyings[out_num_vertices];
                let ratio = self.intersect_ratio(prev_coord, curr_coord);

                *dest_coord = prev_coord.lerp(curr_coord, ratio);

                // Since this computation is performed in clip space before
                // division by w, clipped varying values are perspective-correct

                *dest_varyings = prev_varyings.lerp(curr_varyings, ratio);

                out_num_vertices += 1;
            }

            if curr_inside {
                let dest_coord = &mut out_coords[out_num_vertices];
                let dest_varyings = &mut out_varyings[out_num_vertices];

                *dest_coord = curr_coord;
                *dest_varyings = *curr_varyings;

                out_num_vertices += 1;
            }
        }

        assert!(out_num_vertices <= MAX_VARYINGS);

        out_num_vertices
    }
}

fn is_vertex_visible(v: Vec4) -> bool {
    v.x.abs() <= v.w && v.y.abs() <= v.w && v.z.abs() <= v.w
}

pub fn triangle<V>(
    in_coords: &mut [Vec4; MAX_VARYINGS],
    in_varyings: &mut [V; MAX_VARYINGS],
    out_coords: &mut [Vec4; MAX_VARYINGS],
    out_varyings: &mut [V; MAX_VARYINGS],
) -> usize
where
    V: Varying + Copy,
{
    let v0_visible = is_vertex_visible(in_coords[0]);
    let v1_visible = is_vertex_visible(in_coords[1]);
    let v2_visible = is_vertex_visible(in_coords[2]);
    if v0_visible && v1_visible && v2_visible {
        out_coords[0] = in_coords[0];
        out_coords[1] = in_coords[1];
        out_coords[2] = in_coords[2];
        out_varyings[0] = in_varyings[0];
        out_varyings[1] = in_varyings[1];
        out_varyings[2] = in_varyings[2];
        3
    } else {
        let mut num_vertices = 3;
        num_vertices = Plane::PositiveW.clip(
            num_vertices,
            in_coords,
            in_varyings,
            out_coords,
            out_varyings,
        );
        if num_vertices < 3 {
            return 0;
        }
        num_vertices = Plane::PositiveX.clip(
            num_vertices,
            out_coords,
            out_varyings,
            in_coords,
            in_varyings,
        );
        if num_vertices < 3 {
            return 0;
        }
        num_vertices = Plane::NegativeX.clip(
            num_vertices,
            in_coords,
            in_varyings,
            out_coords,
            out_varyings,
        );
        if num_vertices < 3 {
            return 0;
        }
        num_vertices = Plane::PositiveY.clip(
            num_vertices,
            out_coords,
            out_varyings,
            in_coords,
            in_varyings,
        );
        if num_vertices < 3 {
            return 0;
        }
        num_vertices = Plane::NegativeY.clip(
            num_vertices,
            in_coords,
            in_varyings,
            out_coords,
            out_varyings,
        );
        if num_vertices < 3 {
            return 0;
        }
        num_vertices = Plane::PositiveZ.clip(
            num_vertices,
            out_coords,
            out_varyings,
            in_coords,
            in_varyings,
        );
        if num_vertices < 3 {
            return 0;
        }
        num_vertices = Plane::NegativeZ.clip(
            num_vertices,
            in_coords,
            in_varyings,
            out_coords,
            out_varyings,
        );
        if num_vertices < 3 {
            return 0;
        }
        num_vertices
    }
}
