use {
    ggez::{event, graphics},
    std::time::Duration,
};

const SKY_COLOR: (u8, u8, u8) = (154, 188, 245);
const SHIP_SPEED: f32 = 200.0;
const LASER_SPEED: f32 = 900.0;
const BULLET_SPEED: f32 = 100.0;
const SKY_CORE_SPEED: f32 = 10.0;
const LASER_COOLDOWN: Duration = Duration::from_millis(250);
const SHIP_OFFSET_FROM_BOTTOM: f32 = 100.0;
const FROZEN_SCREEN_FONT_SIZE: f32 = 150.0;

struct MainState {
    ship: game_test::Ship,
    lasers: Vec<game_test::Laser>,
    sky_core: game_test::SkyCore,
    bullets: Vec<game_test::Bullet>,
    time_since_shot_laser: Duration,
    state: State,
}

#[derive(Copy, Clone)]
enum State {
    Playing,
    Frozen {
        state: FrozenState,
        overlay_alpha: f32,
    },
}

#[derive(Copy, Clone)]
enum FrozenState {
    Died,
    Won,
    Paused,
}

impl MainState {
    fn new(ctx: &mut ggez::Context) -> Self {
        let screen_dimens = graphics::screen_coordinates(ctx);

        Self {
            ship: game_test::Ship::new(
                game_test::Point::new(
                    screen_dimens.w / 2.0,
                    screen_dimens.h - SHIP_OFFSET_FROM_BOTTOM,
                ),
                ctx,
            ),
            lasers: vec![],
            sky_core: game_test::SkyCore::new(ctx),
            bullets: vec![],
            // If the time since the laser was last shot is the laser cooldown, then this means that
            // we can start shooting immediately
            time_since_shot_laser: LASER_COOLDOWN,
            state: State::Playing,
        }
    }

    fn update_playing(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        use {
            game_test::{Health, Hitbox, OnScreen, Position, StepDistance, TakeDamage, Velocity},
            ggez::input::keyboard,
            rand::Rng,
        };

        // The longer between each frame, the further things move. This compensates for any
        // changes in FPS.
        let delta_time = ggez::timer::delta(ctx);
        let delta_time_secs = delta_time.as_secs_f32();

        let adjusted_ship_speed = SHIP_SPEED * delta_time_secs;
        let adjusted_laser_speed = LASER_SPEED * delta_time_secs;
        let adjusted_bullet_speed = BULLET_SPEED * delta_time_secs;
        let adjusted_sky_core_speed = SKY_CORE_SPEED * delta_time_secs;

        let keys = keyboard::pressed_keys(ctx);

        //
        // Pause game if P is pressed
        //

        if keys.contains(&keyboard::KeyCode::P) {
            self.state = State::Frozen {
                state: FrozenState::Paused,
                overlay_alpha: 0.0,
            };
            return Ok(());
        }

        //
        // Ship
        //

        // Move ship with WASD.
        let (mut dx, mut dy) = (0.0, 0.0);

        if keys.contains(&keyboard::KeyCode::W) {
            dy -= 1.0;
        }
        if keys.contains(&keyboard::KeyCode::S) {
            dy += 1.0;
        }
        if keys.contains(&keyboard::KeyCode::A) {
            dx -= 1.0;
        }
        if keys.contains(&keyboard::KeyCode::D) {
            dx += 1.0;
        }

        *self.ship.velocity_mut() = {
            let v = game_test::Vector::new(dx, dy);

            // Normalising a 0.0, 0.0 vector ends up with NaN, which we want to avoid.
            if dx == 0.0 && dy == 0.0 {
                v
            } else {
                v.normalize()
            }
        };
        self.ship.step_distance(adjusted_ship_speed);
        self.ship.clamp_pos_to_screen(ctx);

        //
        // Lasers
        //

        // Fire lasers with space if the cooldown has finished.
        if keys.contains(&keyboard::KeyCode::Space) && self.time_since_shot_laser >= LASER_COOLDOWN
        {
            self.lasers.push(self.ship.shoot(ctx));
            self.time_since_shot_laser = Duration::from_millis(0);
        } else {
            self.time_since_shot_laser += delta_time;
        }

        // Make lasers move up the screen.
        for laser in &mut self.lasers {
            laser.pos_mut().y -= adjusted_laser_speed;
        }

        // Let the Sky Core take damage for every laser that hits it.
        for laser in &self.lasers {
            if laser.hitbox().overlaps(&self.sky_core.hitbox()) {
                self.sky_core.take_damage(laser);
            }
        }

        //
        // Sky Core
        //

        // Shoot a bullet from the Sky Core one in two cycles.
        if rand::thread_rng().gen_range(0, 2) == 0 {
            self.bullets.push(self.sky_core.shoot(ctx));
        }

        // Stop the Sky Core from its march down the screen once it reaches the centre.
        let is_sky_core_before_vertical_center =
            self.sky_core.pos().y < graphics::screen_coordinates(ctx).h / 2.0;

        if is_sky_core_before_vertical_center {
            self.sky_core.step_distance(adjusted_sky_core_speed);
        }

        //
        // Bullets
        //

        for bullet in &mut self.bullets {
            bullet.step_distance(adjusted_bullet_speed);
        }

        // The ship takes damage for every bullet that hits it.
        for bullet in &self.bullets {
            if bullet.hitbox().overlaps(&self.ship.hitbox()) {
                self.ship.take_damage(bullet);
            }
        }

        //
        // Shift states if necessary
        //

        match (self.ship.is_dead(), self.sky_core.is_dead()) {
            (true, true) => panic!("sda"),
            (true, _) => {
                self.state = State::Frozen {
                    state: FrozenState::Died,
                    overlay_alpha: 0.0,
                }
            }
            (_, true) => {
                self.state = State::Frozen {
                    state: FrozenState::Won,
                    overlay_alpha: 0.0,
                }
            }
            _ => (),
        }

        //
        // Clean up
        //

        // Remove all bullets and lasers that aren’t on the screen. We do this because otherwise
        // these bullets and lasers (which you can’t see anyway) are going to be continually
        // re-rendered again and again. We also remove lasers that have hit the Sky Core, and
        // bullets that have hit the ship.

        let ship_hitbox = self.ship.hitbox();
        let sky_core_hitbox = self.sky_core.hitbox();

        self.bullets
            .retain(|bullet| bullet.is_on_screen(ctx) && !bullet.hitbox().overlaps(&ship_hitbox));
        self.lasers
            .retain(|laser| laser.is_on_screen(ctx) && !laser.hitbox().overlaps(&sky_core_hitbox));

        Ok(())
    }

    fn update_frozen(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        use ggez::input::keyboard;

        match self.state {
            State::Frozen {
                ref mut overlay_alpha,
                ..
            } => *overlay_alpha = num::clamp(*overlay_alpha + 0.01, 0.0, 1.0),
            _ => unreachable!(),
        }

        let keys = keyboard::pressed_keys(ctx);

        // Retry when ‘r’ is pressed.
        if keys.contains(&keyboard::KeyCode::R) {
            *self = Self::new(ctx);
        }

        Ok(())
    }

    fn draw_playing(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        use game_test::{HealthBar, ImageDrawable};

        graphics::clear(ctx, SKY_COLOR.into());

        for laser in &self.lasers {
            laser.draw(ctx)?;
        }
        self.ship.draw(ctx)?;

        for bullet in &self.bullets {
            bullet.draw(ctx)?;
        }
        self.sky_core.draw(ctx)?;

        self.ship.health_bar(ctx)?.draw(ctx)?;
        self.sky_core.health_bar(ctx)?.draw(ctx)?;

        Ok(())
    }

    fn draw_frozen(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let (state, overlay_alpha) = match self.state {
            State::Frozen {
                state,
                overlay_alpha,
            } => (state, overlay_alpha),
            _ => unreachable!(),
        };

        self.draw_playing(ctx)?;

        let overlay = {
            let screen_dimens = graphics::screen_coordinates(ctx);
            let alpha = overlay_alpha;

            graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                screen_dimens,
                (0.0, 0.0, 0.0, alpha).into(),
            )?
        };

        graphics::draw(ctx, &overlay, (game_test::Point::new(0.0, 0.0),))?;

        let text = {
            let text = match state {
                FrozenState::Died => "You died.",
                FrozenState::Won => "You won!",
                FrozenState::Paused => "Paused",
            };

            graphics::Text::new(
                graphics::TextFragment::new(text)
                    .scale(graphics::Scale::uniform(FROZEN_SCREEN_FONT_SIZE)),
            )
        };

        let screen_dimens = graphics::screen_coordinates(ctx);
        let (text_width, text_height) = text.dimensions(ctx);

        graphics::draw(
            ctx,
            &text,
            (game_test::Point::new(
                screen_dimens.w / 2.0 - text_width as f32 / 2.0,
                screen_dimens.h / 2.0 - text_height as f32 / 2.0,
            ),),
        )?;

        Ok(())
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        // Allow the user to retry if the game is in a frozen state.
        match self.state {
            State::Playing => self.update_playing(ctx),
            State::Frozen { .. } => self.update_frozen(ctx),
        }
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        match self.state {
            State::Playing => self.draw_playing(ctx)?,
            State::Frozen { .. } => self.draw_frozen(ctx)?,
        }

        graphics::present(ctx)
    }
}

fn main() -> ggez::GameResult {
    use ggez::conf;

    let window_setup: conf::WindowSetup = Default::default();
    let window_setup = window_setup.title("Bullet Hell");

    let (mut ctx, mut event_loop) = ggez::ContextBuilder::new("Bullet Hell", "Tai & Aramis")
        .window_setup(window_setup)
        .build()?;

    let state = &mut MainState::new(&mut ctx);
    event::run(&mut ctx, &mut event_loop, state)
}
