use macroquad::math::Vec2;

use crate::view::transform::Transform;

#[allow(unused)]

/// ViewEntities are drawable objects.
/*
To retrieve concrete struct from view_entities:
let entity = self.view_entities.get_mut(&self.foo_id).unwrap();
entity.as_any_mut().downcast_mut::<Foo>().unwrap().radius = 50.;
-- OR --
if let Some(foo) = entity.as_any_mut().downcast_mut::<Foo>() {
    foo.radius = 50.;
}
*/
pub trait ViewEntity {
    //fn as_any(&self) -> &dyn std::any::Any; // simply return 'self'

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any; // simply return 'self'

    fn set_translation(&mut self, translation: Vec2) {
        //self.transform.translation = translation;
    }

    fn set_rotation(&mut self, rotation: f32) {
        //self.transform.rotation = rotation;
    }

    fn contains_point(&mut self, point: &Vec2, parent_transform: &Transform) -> bool {
        false
        /* Typical
        let transform = *parent_transform * self.transform;
        let mut local_pt = transform.convert_point_to_local(point);
        local_pt += self.anchor * self.size;
        rect_contains_point(Vec2::ZERO, self.size, local_pt)
        */
    }

    /// For top-level entities, pass in Transform::new().
    fn process_mouse(&mut self, point: &Vec2, parent_transform: &Transform) -> bool {
        false
        /* Typical:
        let contains_pt = self.contains_point(point, parent_transform);
        self.mouse_state.update(contains_pt, point);
        if self.mouse_state.left_mouse_released {
            if let Some(sender) = SENDER.get() {
                if let Some(action) = &self.action {
                    sender.send(action.clone()).expect("Send error");
                }
            }
        }
        contains
        */
    }

    fn update(&mut self, time_delta: f32) {}

    /// For top-level entities, pass in Transform::new().
    fn draw(&mut self, parent_transform: &Transform) {}
}
