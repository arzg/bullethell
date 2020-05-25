pub trait Velocity {
    fn velocity(&self) -> crate::Vector;
    fn velocity_mut(&mut self) -> &mut crate::Vector;
}
