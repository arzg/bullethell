use ggez::graphics;

pub trait Sprite {
    fn sprite(img_data: &[u8], ctx: &mut ggez::Context) -> anyhow::Result<graphics::Image> {
        use {
            image::ImageDecoder,
            std::{convert::TryInto, io::Read},
        };

        let decoder = image::png::PngDecoder::new(img_data)?;

        let (width, height) = decoder.dimensions();
        let (width, height): (u16, u16) = (width.try_into()?, height.try_into()?);

        let mut rgba = vec![];
        decoder.into_reader()?.read_to_end(&mut rgba)?;

        let mut image = graphics::Image::from_rgba8(ctx, width, height, &rgba)?;

        // Disable antialiasing as all sprites are in 8-bit style.
        image.set_filter(graphics::FilterMode::Nearest);

        Ok(image)
    }
}
