mod engine;

use rand::Rng as _;
use std::time::{Duration, Instant};

const MOVE: f32 = 4.0;

fn main() {
    use baryon::window::{Event, Key};

    let window = baryon::window::Window::new().title("shotgun").build();
    let mut en = engine::Engine::new(&window);
    let (screen_width, screen_height) = en.screen_size();

    let tiles = baryon::asset::SpriteMap {
        origin: [0, 0].into(),
        cell_size: [8, 8].into(),
    };
    let ship_image = en
        .context
        .load_image("assets/SpaceShooterAssetPack_Ships.png");
    let bullet_image = en
        .context
        .load_image("assets/SpaceShooterAssetPack_Projectiles.png");

    let ship = en
        .spawn(engine::Kind::Player, ship_image, tiles.at([1, 2].into()))
        .position(screen_width * 0.5, screen_height * 0.2)
        .finish();
    en.with(ship).stay_on_screen(true);

    let mut rng_enemy = rand::thread_rng();
    let mut last_update = Instant::now();
    let mut last_spawn = last_update;

    window.run(move |event| match event {
        Event::Resize { width, height } => {
            en.context.resize(width, height);
        }
        Event::Keyboard { key, pressed: true } => match key {
            Key::Escape => std::process::exit(0),
            Key::Space => {
                let pos = en.with(ship).node.get_position();
                let _bullet = en
                    .spawn(engine::Kind::Bullet, bullet_image, tiles.at([0, 0].into()))
                    .position(pos.x, pos.y)
                    .velocity(0.0, 50.0)
                    .finish();
            }
            Key::Up => {
                en.with(ship).node.post_move([0.0, MOVE, 0.0].into());
            }
            Key::Down => {
                en.with(ship).node.post_move([0.0, -MOVE, 0.0].into());
            }
            Key::Left => {
                en.with(ship).node.post_move([-MOVE, 0.0, 0.0].into());
            }
            Key::Right => {
                en.with(ship).node.post_move([MOVE, 0.0, 0.0].into());
            }
            _ => {}
        },
        Event::Draw => {
            {
                let elapsed = last_update.elapsed();
                en.update(elapsed.as_secs_f32());
                last_update += elapsed;
            }
            {
                let spawn_period = Duration::from_millis(500);
                if last_spawn.elapsed() >= spawn_period {
                    let (screen_width, screen_height) = en.screen_size();
                    let x = rng_enemy.gen::<f32>() * screen_width;
                    let vy = -10.0 - 5.0 * rng_enemy.gen::<f32>();
                    let _enemy = en
                        .spawn(engine::Kind::Enemy, ship_image, tiles.at([5, 0].into()))
                        .position(x, screen_height + 20.0)
                        .velocity(0.0, vy)
                        .finish();
                    last_spawn += spawn_period;
                }
            }
            en.draw();
        }
        _ => {}
    })
}
