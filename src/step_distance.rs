pub trait StepDistance: crate::Position + crate::Velocity {
    fn step_distance(&mut self, distance: f32) {
        let velocity = self.velocity();
        *self.pos_mut() += velocity * distance;
    }
}
