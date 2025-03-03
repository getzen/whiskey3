use macroquad::math::Vec2;

use crate::view::transform::Transform;

#[allow(unused)]

/// ViewEntities are drawables.
/// To get a concrete type from a trait object, implement as_any() below, then:
/// if let Some(sprite) = my_trait_obj.as_any().downcast_ref::<Sprite>().expect("Failed to downcast.") {
///     sprite...
/// }
pub trait ViewEntity {
    //fn as_any(&self) -> &dyn std::any::Any; // simply return 'self'

    fn set_translation(&mut self, translation: Vec2) {
        //self.transform.translation = _translation;
    }

    fn set_rotation(&mut self, rotation: f32) {
        //self.transform.rotation = _rotation;
    }

    /// For top-level entities, pass in Transform::new().
    fn process_mouse(&mut self, mouse_pos: &Vec2, parent_transform: &Transform) -> bool {
        false
        /* Typical:
        let mouse_over = self.eventer.process_mouse(&_mouse_pos, &self.transform);
        if self.eventer.left_mouse_released {
            if let Some(sender) = SENDER.get() {
                if let Some(action) = &self.action {
                    sender.send(action.clone()).expect("Send error");
                }
            }
        }
        mouse_over
        */
    }

    fn update(&mut self, time_delta: f32) {}

    /// For top-level entities, pass in Transform::new().
    fn draw(&mut self, parent_transform: &Transform) {}
}
