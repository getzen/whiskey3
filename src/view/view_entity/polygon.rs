use crate::view::{transform::Transform, utility_graphics::{draw_polygon, draw_polygon_lines, polygon_contains_point}};

use super::view_entity::ViewEntity;

use macroquad::prelude::*;

#[allow(unused)]
pub struct Polygon {
    pub transform: Transform,
    pub vertices: Vec<Vec2>,
    pub fill_color: Option<Color>,
    pub stroke_color: Option<Color>,
    pub stroke_width: f32,
}

#[allow(unused)]
impl Polygon {
    pub fn new(
        vertices: Vec<Vec2>,
        fill_color: Option<Color>,
        stroke_color: Option<Color>,
        stroke_width: f32,
    ) -> Self {
        Self {
            transform: Transform::new(),
            vertices,
            fill_color,
            stroke_color,
            stroke_width,
        }
    }

    /*
    For use with Rapier2D physics

    /// Converts the polygon vertices to a vec of Points and creates
    /// a Collider. The shape must be convex.
    pub fn create_convex_collider(&self) -> Collider {
        let mut points = Vec::new();
        for v in &self.vertices {
            points.push(point!(v.x, v.y));
        }
        ColliderBuilder::convex_hull(&points).unwrap().build()
    }

    /// Converts the polygon vertices to a vec of vertex pairs and a vec
    /// of matching indices, and feeds these to a convex decomposer to
    /// create a compound shape collider from a possibly non-convex polygon.
    pub fn create_compound_collider(&self) -> Collider {
        let mut coll_verts = Vec::new();
        let mut indices = Vec::new();
        for (idx, v) in self.vertices.iter().enumerate() {
            coll_verts.push(Point::new(v.x, v.y));
            if idx == self.vertices.len() - 1 {
                indices.push([idx as u32, 0]);
            } else {
                indices.push([idx as u32, idx as u32 + 1]);
            }
        }
        // In case the vertices form a concave objext, we use this decomposer.
        ColliderBuilder::convex_decomposition(&coll_verts, &indices).build()
    }
    */
}

impl ViewEntity for Polygon {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn set_translation(&mut self, translation: Vec2) {
        self.transform.translation = translation;
    }

    fn set_rotation(&mut self, rotation: f32) {
        self.transform.rotation = rotation;
    }

    fn contains_point(&mut self, point: &Vec2, parent_transform: &Transform) -> bool {
        let transform = *parent_transform * self.transform;
        let local_pt = transform.convert_point_to_local(point);
        polygon_contains_point(&self.vertices, local_pt)
    }

    fn process_mouse(&mut self, point: &Vec2, parent_transform: &Transform) -> bool {
        self.contains_point(point, parent_transform)
    }

    fn draw(&mut self, parent_transform: &Transform) {
        let transform = *parent_transform * self.transform;

        let gl = unsafe { get_internal_gl().quad_gl };
        gl.push_model_matrix(transform.matrix());
    
        if let Some(color) = self.fill_color {
            draw_polygon(&self.vertices, color);
        }
        if let Some(color) = self.stroke_color {
            draw_polygon_lines(&self.vertices, self.stroke_width, color);
        }
        gl.pop_model_matrix();
    }
}
