pub trait Damage {
    const DAMAGE: u16;
}

pub trait TakeDamage: crate::Health {
    fn take_damage<D: Damage>(&mut self, _damager: &D) {
        *self.health_mut() -= D::DAMAGE;
    }
}

impl<T: crate::Health> TakeDamage for T {}
