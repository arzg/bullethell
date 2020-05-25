use ggez::graphics;

pub trait Position {
    fn pos(&self) -> crate::Point;
    fn pos_mut(&mut self) -> &mut crate::Point;
}

pub trait CenterPosition: AsRef<graphics::Image> + Position {
    fn center_pos(&self) -> crate::Point {
        let img = self.as_ref();
        let dimens = img.dimensions();
        let pos = self.pos();

        crate::Point::new(pos.x - dimens.w / 2.0, pos.y - dimens.h / 2.0)
    }
}

impl<T: AsRef<graphics::Image> + Position> CenterPosition for T {}
