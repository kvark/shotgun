mod engine;

use rand::Rng as _;
use std::time::Duration;

const MOVE: f32 = 4.0;

fn main() {
    use baryon::window::{Event, Key};

    let window = baryon::window::Window::new().title("shotgun").build();
    let mut en = engine::Engine::new(&window);
    let (screen_width, screen_height) = en.screen_size();

    let ship_map = baryon::asset::SpriteMap {
        origin: [0, 0].into(),
        cell_size: [8, 8].into(),
    };
    let ship_image = en
        .context
        .load_image("assets/SpaceShooterAssetPack_Ships.png");

    let ship = en
        .spawn(engine::Kind::Player, ship_image, ship_map.at([1, 2].into()))
        .position(screen_width * 0.5, screen_height * 0.2)
        .finish();
    //en.control(mc::Control::ArrowKeys, ship);
    en.with(ship).stay_on_screen(true);

    let mut rng_enemy = rand::thread_rng();
    /*
    en.on_update(Duration::from_millis(500), move |g| {
        let x = rng_enemy.gen::<f32>() * screen_width;
        let vy = -10.0 - 5.0 * rng_enemy.gen::<f32>();
        let _enemy = g
            .create(mc::Kind::ENEMY, SHIP_TEXTURE, SHIP_MAP.at(5, 0))
            .position(x, screen_height + 20.0)
            .velocity(0.0, vy)
            .finish();
    });*/

    window.run(move |event| match event {
        Event::Resize { width, height } => {
            en.context.resize(width, height);
        }
        Event::Keyboard { key, pressed: true } => match key {
            Key::Escape => std::process::exit(0),
            Key::Up => {
                /*if let Some(entity) = self.input.arrow_control {
                    self.scene[entity].post_move([0.0, MOVE, 0.0].into());
                }*/
            }
            Key::Down => {
                /*if let Some(entity) = self.input.arrow_control {
                    self.scene[entity].post_move([0.0, -MOVE, 0.0].into());
                }*/
            }
            Key::Left => {
                /*if let Some(entity) = self.input.arrow_control {
                    self.scene[entity].post_move([-MOVE, 0.0, 0.0].into());
                }*/
            }
            Key::Right => {
                /*if let Some(entity) = self.input.arrow_control {
                    self.scene[entity].post_move([MOVE, 0.0, 0.0].into());
                }*/
            }
            _ => {}
        },
        Event::Draw => {
            //self.update();
            en.draw();
        }
        _ => {}
    })
}
