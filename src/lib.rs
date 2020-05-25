use ggez::graphics;

mod bullet;
mod damage;
mod health;
mod hitbox;
mod image_drawable;
mod laser;
mod position;
mod ship;
mod sky_core;
mod sprite;
mod step_distance;
mod velocity;

pub use {
    bullet::Bullet,
    damage::{Damage, TakeDamage},
    health::{Health, HealthBar},
    hitbox::Hitbox,
    image_drawable::ImageDrawable,
    laser::Laser,
    position::{CenterPosition, Position},
    ship::Ship,
    sky_core::SkyCore,
    sprite::Sprite,
    step_distance::StepDistance,
    velocity::Velocity,
};

pub type Point = ggez::nalgebra::Point2<f32>;
pub type Vector = ggez::nalgebra::Vector2<f32>;

pub fn are_overlapping(lhs: graphics::Rect, rhs: graphics::Rect) -> bool {
    (lhs.x < (rhs.x + rhs.w))
        && ((lhs.x + lhs.w) > rhs.x)
        && (lhs.y < (rhs.y + rhs.h))
        && ((lhs.y + lhs.h) > rhs.y)
}
