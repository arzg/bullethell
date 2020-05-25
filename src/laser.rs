use {crate::Sprite, ggez::graphics};

pub struct Laser {
    pos: crate::Point,
    sprite_cache: graphics::Image,
}

impl<'a> Laser {
    const IMG_DATA: &'a [u8] = include_bytes!("laser.png");

    pub fn new(pos: crate::Point, ctx: &mut ggez::Context) -> Self {
        Self {
            pos,
            sprite_cache: Self::sprite(Self::IMG_DATA, ctx).unwrap(),
        }
    }
}

impl crate::Position for Laser {
    fn pos(&self) -> crate::Point {
        self.pos
    }

    fn pos_mut(&mut self) -> &mut crate::Point {
        &mut self.pos
    }
}

impl crate::Damage for Laser {
    const DAMAGE: u16 = 1;
}

impl Sprite for Laser {}

impl AsRef<graphics::Image> for Laser {
    fn as_ref(&self) -> &graphics::Image {
        &self.sprite_cache
    }
}

impl crate::Hitbox for Laser {}
