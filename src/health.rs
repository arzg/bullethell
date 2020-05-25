use ggez::graphics;

const HEALTHBAR_WIDTH: f32 = 64.0;
const HEALTHBAR_HEIGHT: f32 = 8.0;
const HEALTHBAR_COLOR: (u8, u8, u8) = (119, 249, 169);

pub trait Health {
    const MAX_HEALTH: u16;

    fn health(&self) -> u16;
    fn health_mut(&mut self) -> &mut u16;
}

pub trait HealthBar: Health + crate::CenterPosition {
    fn health_bar(&self, ctx: &mut ggez::Context) -> ggez::GameResult<graphics::Mesh> {
        let pos = self.center_pos();
        let health_frac: f32 = f32::from(self.health()) / f32::from(Self::MAX_HEALTH);

        graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect {
                x: pos.x,
                // Leave a gap the size of the healthbar between the healthbar and whatever we are
                // displaying the health of.
                y: pos.y - HEALTHBAR_HEIGHT * 2.0,
                w: HEALTHBAR_WIDTH * health_frac,
                h: HEALTHBAR_HEIGHT,
            },
            HEALTHBAR_COLOR.into(),
        )
    }
}

impl<T: Health + crate::CenterPosition> HealthBar for T {}
