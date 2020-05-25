use ggez::graphics;

pub trait Hitbox: AsRef<graphics::Image> + crate::CenterPosition {
    const WIDTH_MUL: f32 = 1.0;
    const HEIGHT_MUL: f32 = 1.0;
    const X_OFFSET_MUL: f32 = 0.0;
    const Y_OFFSET_MUL: f32 = 0.0;

    fn hitbox(&self) -> graphics::Rect {
        let pos = self.center_pos();
        let img = self.as_ref();

        let img_width: f32 = img.width().into();
        let img_height: f32 = img.height().into();

        let width = img_width * Self::WIDTH_MUL;
        let height = img_height * Self::HEIGHT_MUL;

        let x_offset = Self::X_OFFSET_MUL * img_width;
        let y_offset = Self::Y_OFFSET_MUL * img_width;

        graphics::Rect {
            x: pos.x - (width - img_width) / 2.0 + x_offset,
            y: pos.y - (height - img_height) / 2.0 + y_offset,
            w: width,
            h: height,
        }
    }
}
