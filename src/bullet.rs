use {crate::Sprite, ggez::graphics};

pub struct Bullet {
    pos: crate::Point,
    velocity: crate::Vector,
    sprite_cache: graphics::Image,
}

impl<'a> Bullet {
    const IMG_DATA: &'a [u8] = include_bytes!("bullet.png");

    pub fn new(pos: crate::Point, ctx: &mut ggez::Context) -> Self {
        use rand::Rng;

        let velocity = {
            let mut rng = rand::thread_rng();

            let mut rand_vector =
                crate::Vector::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0));

            // Normalize so that we can randomly modify the speed from a consistent base.
            rand_vector.normalize_mut();
            rand_vector *= rng.gen_range(0.5, 2.0);

            rand_vector
        };

        Self {
            pos,
            velocity,
            sprite_cache: Self::sprite(Self::IMG_DATA, ctx).unwrap(),
        }
    }
}

impl crate::Position for Bullet {
    fn pos(&self) -> crate::Point {
        self.pos
    }

    fn pos_mut(&mut self) -> &mut crate::Point {
        &mut self.pos
    }
}

impl crate::Velocity for Bullet {
    fn velocity(&self) -> crate::Vector {
        self.velocity
    }

    fn velocity_mut(&mut self) -> &mut crate::Vector {
        &mut self.velocity
    }
}

impl crate::StepDistance for Bullet {}

impl crate::Damage for Bullet {
    const DAMAGE: u16 = 1;
}

impl Sprite for Bullet {}

impl AsRef<graphics::Image> for Bullet {
    fn as_ref(&self) -> &graphics::Image {
        &self.sprite_cache
    }
}

impl crate::Hitbox for Bullet {
    const WIDTH_MUL: f32 = 0.85;
    const HEIGHT_MUL: f32 = 0.85;
}
