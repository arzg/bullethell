use ggez::graphics;

const HEALTHBAR_WIDTH: f32 = 64.0;
const HEALTHBAR_HEIGHT: f32 = 8.0;
const HEALTHBAR_COLOR: (u8, u8, u8) = (119, 249, 169);
const HEALTHBAR_BG_COLOR: (u8, u8, u8) = (229, 37, 72);

pub trait Health {
    const MAX_HEALTH: u16;

    fn health(&self) -> u16;
    fn health_mut(&mut self) -> &mut u16;

    fn is_dead(&self) -> bool {
        self.health() == 0
    }
}

pub struct HealthBarMesh {
    bg: graphics::Mesh,
    healthbar: graphics::Mesh,
}

impl HealthBarMesh {
    pub fn draw(&self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::draw(ctx, &self.bg, (crate::Point::new(0.0, 0.0),))?;
        graphics::draw(ctx, &self.healthbar, (crate::Point::new(0.0, 0.0),))?;

        Ok(())
    }
}

pub trait HealthBar: Health + crate::CenterPosition {
    fn health_bar(&self, ctx: &mut ggez::Context) -> ggez::GameResult<HealthBarMesh> {
        let pos = self.center_pos();
        let health_frac: f32 = f32::from(self.health()) / f32::from(Self::MAX_HEALTH);

        // Leave a gap the size of the healthbar between the healthbar and whatever we are
        // displaying the health of.
        let (x, y) = (pos.x, pos.y - HEALTHBAR_HEIGHT * 2.0);

        let bg = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect {
                x,
                y,
                w: HEALTHBAR_WIDTH,
                h: HEALTHBAR_HEIGHT,
            },
            HEALTHBAR_BG_COLOR.into(),
        )?;

        let healthbar = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect {
                x,
                y,
                w: HEALTHBAR_WIDTH * health_frac,
                h: HEALTHBAR_HEIGHT,
            },
            HEALTHBAR_COLOR.into(),
        )?;

        Ok(HealthBarMesh { bg, healthbar })
    }
}

impl<T: Health + crate::CenterPosition> HealthBar for T {}
