use ggez::graphics;

pub trait OnScreen: crate::Hitbox {
    fn is_on_screen(&self, ctx: &ggez::Context) -> bool {
        self.hitbox().overlaps(&graphics::screen_coordinates(ctx))
    }
}

impl<T: crate::Hitbox> OnScreen for T {}
