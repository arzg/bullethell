use {
    crate::{Health, Sprite},
    ggez::graphics,
};

pub struct Ship {
    pos: crate::Point,
    velocity: crate::Vector,
    sprite_cache: graphics::Image,
    health: u16,
}

impl<'a> Ship {
    const IMG_DATA: &'a [u8] = include_bytes!("ship.png");

    pub fn new(pos: crate::Point, ctx: &mut ggez::Context) -> Self {
        Self {
            pos,
            velocity: crate::Vector::new(0.0, 1.0),
            sprite_cache: Self::sprite(Self::IMG_DATA, ctx).unwrap(),
            health: Self::MAX_HEALTH,
        }
    }

    pub fn shoot(&self, ctx: &mut ggez::Context) -> crate::Laser {
        crate::Laser::new(self.pos, ctx)
    }

    pub fn clamp_pos_to_screen(&mut self, ctx: &ggez::Context) {
        let screen_dimens = graphics::screen_coordinates(ctx);

        self.pos.x = num::clamp(self.pos.x, 0.0, screen_dimens.w);
        self.pos.y = num::clamp(self.pos.y, 0.0, screen_dimens.h);
    }
}

impl crate::Position for Ship {
    fn pos(&self) -> crate::Point {
        self.pos
    }

    fn pos_mut(&mut self) -> &mut crate::Point {
        &mut self.pos
    }
}

impl crate::Velocity for Ship {
    fn velocity(&self) -> crate::Vector {
        self.velocity
    }

    fn velocity_mut(&mut self) -> &mut crate::Vector {
        &mut self.velocity
    }
}

impl crate::StepDistance for Ship {}

impl Health for Ship {
    const MAX_HEALTH: u16 = 10;

    fn health(&self) -> u16 {
        self.health
    }

    fn health_mut(&mut self) -> &mut u16 {
        &mut self.health
    }
}

impl Sprite for Ship {}

impl AsRef<graphics::Image> for Ship {
    fn as_ref(&self) -> &graphics::Image {
        &self.sprite_cache
    }
}

impl crate::Hitbox for Ship {
    const WIDTH_MUL: f32 = 0.5;
    const HEIGHT_MUL: f32 = 0.4;
    const Y_OFFSET_MUL: f32 = 0.05;
}
