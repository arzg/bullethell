use ggez::graphics;

pub trait ImageDrawable: crate::CenterPosition + AsRef<graphics::Image> {
    fn draw(&self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::draw(ctx, self.as_ref(), (self.center_pos(),))
    }
}

impl<T: crate::CenterPosition + AsRef<graphics::Image>> ImageDrawable for T {}
