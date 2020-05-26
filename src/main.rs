use {
    ggez::{event, graphics},
    std::time::Duration,
};

const SKY_COLOR: (u8, u8, u8) = (154, 188, 245);
const SHIP_SPEED: f32 = 200.0;
const LASER_SPEED: f32 = 900.0;
const BULLET_SPEED: f32 = 100.0;
const SKY_CORE_SPEED: f32 = 10.0;
const LASER_COOLDOWN: Duration = Duration::from_millis(100);
const SHIP_OFFSET_FROM_BOTTOM: f32 = 100.0;

struct MainState {
    ship: game_test::Ship,
    lasers: Vec<game_test::Laser>,
    sky_core: game_test::SkyCore,
    bullets: Vec<game_test::Bullet>,
    cooled_down: bool,
    shoot_s: cb::Sender<()>,
    cooldown_r: cb::Receiver<()>,
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
}

impl MainState {
    fn new(ctx: &mut ggez::Context) -> Self {
        use std::thread;

        let (shoot_s, shoot_r) = cb::unbounded();
        let (cooldown_s, cooldown_r) = cb::unbounded();

        thread::spawn(move || {
            loop {
                // A laser has been shot. We reply once the cooldown is finished.
                if shoot_r.recv().is_ok() {
                    thread::sleep(LASER_COOLDOWN);

                    // If we can’t send values on the channel then the player gets an unfair
                    // advantage -- no cooldown on the laser. This is unacceptable, so we stop the
                    // whole game.
                    cooldown_s.send(()).unwrap();
                }
            }
        });

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
            cooled_down: true,
            shoot_s,
            cooldown_r,
            state: State::Playing,
        }
    }

    fn update_playing(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        use {
            game_test::{Health, Hitbox, Position, StepDistance, TakeDamage, Velocity},
            ggez::input::keyboard,
            rand::Rng,
        };

        // The longer between each frame, the further things move. This compensates for any
        // changes in FPS.
        let delta_time = ggez::timer::delta(ctx).as_secs_f32();

        let adjusted_ship_speed = SHIP_SPEED * delta_time;
        let adjusted_laser_speed = LASER_SPEED * delta_time;
        let adjusted_bullet_speed = BULLET_SPEED * delta_time;
        let adjusted_sky_core_speed = SKY_CORE_SPEED * delta_time;

        let keys = keyboard::pressed_keys(ctx);

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

        //
        // Lasers
        //

        // We use try_recv() rather than recv() to ensure that this doesn’t block. After all, we
        // have other things to do (e.g. move the ship) while the laser is cooling down.
        if self.cooldown_r.try_recv().is_ok() {
            // The cooldown thread has sent a message (meaning that the cooldown is complete).
            self.cooled_down = true;
        }

        // Fire lasers with space if the cooldown has finished.
        if keys.contains(&keyboard::KeyCode::Space) && self.cooled_down {
            self.lasers.push(self.ship.shoot(ctx));

            // Notify the cooldown thread to start the cooldown as the ship has just fired a
            // laser.
            self.cooled_down = false;
            self.shoot_s.send(()).unwrap();
        }

        // Make lasers move up the screen.
        for laser in &mut self.lasers {
            laser.pos_mut().y -= adjusted_laser_speed;
        }

        // Let the Sky Core’s take damage for every laser that hits it.
        for laser in &self.lasers {
            if laser.hitbox().overlaps(&self.sky_core.hitbox()) {
                self.sky_core.take_damage(laser);
            }
        }

        // Remove all lasers that are ‘above’ the screen. We do this because otherwise these
        // lasers (which you can’t see anyway) are going to be continually re-rendered again and
        // again.
        self.lasers.retain(|laser| laser.pos().y >= 0.0);

        //
        // Sky Core
        //

        // Shoot a bullet from the Sky Core one in ten cycles.
        if rand::thread_rng().gen_range(0, 10) == 0 {
            self.bullets.push(self.sky_core.shoot(ctx));
        }

        self.sky_core.step_distance(adjusted_sky_core_speed);

        //
        // Bullets
        //

        for bullet in &mut self.bullets {
            bullet.step_distance(adjusted_bullet_speed);
        }

        let ship_hitbox = self.ship.hitbox();

        // The ship takes damage for every bullet that hits it.
        for bullet in &self.bullets {
            if bullet.hitbox().overlaps(&ship_hitbox) {
                self.ship.take_damage(bullet);
            }
        }

        // Only keep bullets that are above the bottom of the screen and haven’t hit the ship.
        self.bullets.retain(|bullet| {
            let above_bottom_of_screen = bullet.pos().y <= graphics::screen_coordinates(ctx).h;
            let hit_ship = bullet.hitbox().overlaps(&ship_hitbox);
            above_bottom_of_screen && !hit_ship
        });

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

        let ship_health_bar = self.ship.health_bar(ctx)?;
        graphics::draw(ctx, &ship_health_bar, (game_test::Point::new(0.0, 0.0),))?;

        let sky_core_health_bar = self.sky_core.health_bar(ctx)?;
        graphics::draw(
            ctx,
            &sky_core_health_bar,
            (game_test::Point::new(0.0, 0.0),),
        )?;

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
            };

            graphics::Text::new(text)
        };

        graphics::draw(ctx, &text, (game_test::Point::new(0.0, 0.0),))?;

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
