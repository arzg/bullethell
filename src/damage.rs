pub trait Damage {
    const DAMAGE: u16;
}

pub trait TakeDamage: crate::Health {
    fn take_damage<D: Damage>(&mut self, _damager: &D) {
        // We subtract without underflowing to keep the health at zero if a fatal shot has been
        // fired.
        *self.health_mut() = self.health().saturating_sub(D::DAMAGE);
    }
}

impl<T: crate::Health> TakeDamage for T {}
