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

struct MainState {
    ship: game_test::Ship,
    lasers: Vec<game_test::Laser>,
    sky_core: game_test::SkyCore,
    bullets: Vec<game_test::Bullet>,
    cooled_down: bool,
    shoot_s: cb::Sender<()>,
    cooldown_r: cb::Receiver<()>,
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

        Self {
            ship: game_test::Ship::new(game_test::Point::new(0.0, 0.0), ctx),
            lasers: vec![],
            sky_core: game_test::SkyCore::new(ctx),
            bullets: vec![],
            cooled_down: true,
            shoot_s,
            cooldown_r,
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        use {
            game_test::{Hitbox, Position, StepDistance, TakeDamage, Velocity},
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
            if game_test::are_overlapping(laser.hitbox(), self.sky_core.hitbox()) {
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
        if rand::thread_rng().gen_range(0, 5) == 0 {
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
            if game_test::are_overlapping(bullet.hitbox(), ship_hitbox) {
                self.ship.take_damage(bullet);
            }
        }

        // Only keep bullets that are above the bottom of the screen and haven’t hit the ship.
        self.bullets.retain(|bullet| {
            let above_bottom_of_screen = bullet.pos().y <= graphics::screen_coordinates(ctx).h;
            let hit_ship = game_test::are_overlapping(bullet.hitbox(), ship_hitbox);
            above_bottom_of_screen && !hit_ship
        });

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
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

        graphics::present(ctx)?;
        Ok(())
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
