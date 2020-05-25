use {
    crate::{Health, Sprite},
    ggez::graphics,
};

pub struct SkyCore {
    pos: crate::Point,
    velocity: crate::Vector,
    sprite_cache: graphics::Image,
    health: u16,
}

impl<'a> SkyCore {
    const IMG_DATA: &'a [u8] = include_bytes!("sky_core.png");

    pub fn new(ctx: &mut ggez::Context) -> Self {
        let screen_width = graphics::screen_coordinates(ctx).w;

        Self {
            pos: crate::Point::new(screen_width / 2.0, 0.0),
            velocity: crate::Vector::new(0.0, 1.0), // Move down the screen.
            sprite_cache: Self::sprite(Self::IMG_DATA, ctx).unwrap(),
            health: Self::MAX_HEALTH,
        }
    }

    pub fn shoot(&self, ctx: &mut ggez::Context) -> crate::Bullet {
        crate::Bullet::new(self.pos, ctx)
    }
}

impl crate::Position for SkyCore {
    fn pos(&self) -> crate::Point {
        self.pos
    }

    fn pos_mut(&mut self) -> &mut crate::Point {
        &mut self.pos
    }
}

impl crate::Velocity for SkyCore {
    fn velocity(&self) -> crate::Vector {
        self.velocity
    }

    fn velocity_mut(&mut self) -> &mut crate::Vector {
        &mut self.velocity
    }
}

impl crate::StepDistance for SkyCore {}

impl Health for SkyCore {
    const MAX_HEALTH: u16 = 500;

    fn health(&self) -> u16 {
        self.health
    }

    fn health_mut(&mut self) -> &mut u16 {
        &mut self.health
    }
}

impl Sprite for SkyCore {}

impl AsRef<graphics::Image> for SkyCore {
    fn as_ref(&self) -> &graphics::Image {
        &self.sprite_cache
    }
}

impl crate::Hitbox for SkyCore {}
