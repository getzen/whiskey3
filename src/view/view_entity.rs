use macroquad::math::Vec2;

/// GameEntities are top-level objects. They are stored by Id.
/// They almost always have a Transform component, along with a
/// drawing component such as a Sprite or Text.
pub trait ViewEntity {
    fn set_translation(&mut self, _translation: Vec2) {
        //self.transform.translation = _translation;
    }

    fn set_rotation(&mut self, _rotation: f32) {
        //self.transform.rotation = _rotation;
    }

    fn update_eventer(&mut self, _mouse_pos: Vec2) -> bool {
        false
        /* Typical:
        let mouse_over = self.eventer.process_events(&_mouse_pos, &self.transform, &anchor);
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

    fn update(&mut self, _time_delta: f32) {}

    fn draw(&mut self) {}
}
