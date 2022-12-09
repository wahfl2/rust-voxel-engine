use nalgebra::{Vector3, Vector2};

use super::vertex::Vertex;

pub struct Triangle {
    pub a: Vector3<f32>,
    pub b: Vector3<f32>,
    pub c: Vector3<f32>,
}

impl Triangle {
    pub fn new(vertices: [Vector3<f32>; 3]) -> Self {
        Self {
            a: vertices[0],
            b: vertices[1],
            c: vertices[2],
        }
    }

    pub fn normal(&self) -> Vector3<f32> {
        (self.c - self.a).cross(&(self.b - self.a))
    }
}

pub struct Quad {
    vertices: [Vector3<f32>; 4],
}

impl Quad {
    pub fn new_unchecked(vertices: [Vector3<f32>; 4]) -> Self {
        Self {
            vertices
        }
    }

    /// Creates a quad facing the x-axis
    pub fn new_x_center(center: Vector3<f32>, width: f32, height: f32, facing_positive: bool) -> Self {
        let (cx, cy, cz) = (center.x, center.y, center.z);
        let (half_width, half_height) = (width * 0.5, height * 0.5 * {
            match facing_positive {
                true => -1.0,
                false => 1.0,
            }
        });

        Self::new_unchecked([
            Vector3::new(cx, cy + half_height, cz - half_width),
            Vector3::new(cx, cy + half_height, cz + half_width),
            Vector3::new(cx, cy - half_height, cz + half_width),
            Vector3::new(cx, cy - half_height, cz - half_width),
        ])
    }

    /// Creates a quad facing the y-axis
    pub fn new_y_center(center: Vector3<f32>, width: f32, height: f32, facing_positive: bool) -> Self {
        let (cx, cy, cz) = (center.x, center.y, center.z);
        let (half_width, half_height) = (width * 0.5, height * 0.5 * {
            match facing_positive {
                true => 1.0,
                false => -1.0,
            }
        });

        Self::new_unchecked([
            Vector3::new(cx - half_height, cy, cz - half_width),
            Vector3::new(cx + half_height, cy, cz - half_width),
            Vector3::new(cx + half_height, cy, cz + half_width),
            Vector3::new(cx - half_height, cy, cz + half_width),
        ])
    }

    /// Creates a quad facing the z-axis
    pub fn new_z_center(center: Vector3<f32>, width: f32, height: f32, facing_positive: bool) -> Self {
        let (cx, cy, cz) = (center.x, center.y, center.z);
        let (half_width, half_height) = (width * 0.5, height * 0.5 * {
            match facing_positive {
                true => -1.0,
                false => 1.0,
            }
        });

        Self::new_unchecked([
            Vector3::new(cx - half_width, cy - half_height, cz),
            Vector3::new(cx + half_width, cy - half_height, cz),
            Vector3::new(cx + half_width, cy + half_height, cz),
            Vector3::new(cx - half_width, cy + half_height, cz),
        ])
    }

    pub fn get_vertex_positions(&self) -> &[Vector3<f32>; 4] {
        &self.vertices
    }

    pub fn get_vertices(&self, texture_id: u32) -> [Vertex; 6] {
        let normal = self.get_triangles().0.normal();

        let a = Vertex::new(self.vertices[0], normal, Vector2::new(0.0, 0.0), texture_id);
        let b = Vertex::new(self.vertices[1], normal, Vector2::new(1.0, 0.0), texture_id);
        let c = Vertex::new(self.vertices[2], normal, Vector2::new(1.0, 1.0), texture_id);
        let d = Vertex::new(self.vertices[3], normal, Vector2::new(0.0, 1.0), texture_id);

        [a, b, c, a, c, d]
    }

    pub fn get_triangles(&self) -> (Triangle, Triangle) {
        (
            Triangle::new([self.vertices[0], self.vertices[1], self.vertices[2]]),
            Triangle::new([self.vertices[0], self.vertices[2], self.vertices[3]]),
        )
    }
}