use crate::view::{transform::Transform, utility_graphics::{draw_polygon, draw_polygon_lines}};

use super::view_entity::ViewEntity;

use macroquad::prelude::*;

#[allow(unused)]
pub struct Polygon {
    pub transform: Transform,
    pub vertices: Vec<Vec2>,
    /// When None, these vertices are calculate from 'vertices' and the transform.
    pub draw_vertices: Option<Vec<Vec2>>,
    pub fill_color: Option<Color>,
    pub stroke_color: Option<Color>,
    pub stroke_width: f32,
}

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
            draw_vertices: None,
            fill_color,
            stroke_color,
            stroke_width,
        }
    }

    pub fn update_draw_vertices(&mut self, parent_transform: &Transform) {
        let transform = *parent_transform * self.transform;
        let (pos, rot, scale) = transform.trans_rot_scale();
        let mut verts = Vec::new();
        for v in &self.vertices {
            let rot_vec = Vec2::from_angle(rot);
            let mut adj_v = *v * scale;
            adj_v = adj_v.rotate(rot_vec);
            adj_v = adj_v + pos;
            verts.push(adj_v);
        }
        self.draw_vertices = Some(verts);
    }
}

impl ViewEntity for Polygon {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn set_translation(&mut self, translation: Vec2) {
        self.transform.translation = translation;
        self.draw_vertices = None;
    }

    fn set_rotation(&mut self, rotation: f32) {
        self.transform.rotation = rotation;
        self.draw_vertices = None;
    }

    // fn contains_point(&mut self, point: &Vec2, parent_transform: &Transform) -> bool {
    //     if self.draw_vertices.is_none() {
    //         self.update_draw_vertices(parent_transform);
    //     }
    //     let vertices = &self.draw_vertices.as_ref().unwrap();
    //     polygon_contains_point(vertices, *point)
    // }

    fn process_mouse(&mut self, point: &Vec2, parent_transform: &Transform) -> bool {
        // let contains = self.contains_point(point, parent_transform);
        // self.mouse_status.update(point, contains); // need point for dragging
        // if self.mouse_status.left_mouse_released {
        //     if let Some(sender) = SENDER.get() {
        //         if let Some(action) = &self.action {
        //             sender.send(action.clone()).expect("Send error");
        //         }
        //     }
        // }
        // contains
        false
    }

    fn draw(&mut self, parent_transform: &Transform) {
        if self.draw_vertices.is_none() {
            self.update_draw_vertices(parent_transform);
        }

        if let Some(vertices) = &self.draw_vertices {
            if let Some(color) = self.fill_color {
                draw_polygon(&vertices, color);
            }
    
            if let Some(color) = self.stroke_color {
                draw_polygon_lines(&vertices, self.stroke_width, color);
            }
        }
       
        // let gl = unsafe { get_internal_gl().quad_gl };
        // gl.push_model_matrix(transform.matrix());
        // if let Some(color) = self.fill_color {
        //     draw_polygon(&self.vertices, color);
        // }
        // if let Some(color) = self.stroke_color {
        //     draw_polygon_lines(&self.vertices, self.stroke_width, color);
        // }
        // gl.pop_model_matrix();
    }
}
