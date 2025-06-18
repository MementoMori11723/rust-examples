use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;

struct Ball {
    position: Vector2,
    speed: f32,
    radius: f32,
    color: Color,
}

struct KeyMap<'a> {
    key: KeyboardKey,
    axis: &'a str,
    add: bool,
}

impl Ball {
    fn apply_movement(&mut self, axis: &str, add: bool) {
        match axis {
            "x" => self.position.x = self.calculate_speed(self.position.x, add),
            "y" => self.position.y = self.calculate_speed(self.position.y, add),
            _ => eprintln!("Invalid Axis!"),
        }
    }

    fn calculate_speed(&self, value: f32, add: bool) -> f32 {
        if add {
            value + self.speed
        } else {
            value - self.speed
        }
    }

    fn keymap<'a>(&self) -> [KeyMap<'a>; 4] {
        let keys = [KEY_LEFT, KEY_RIGHT, KEY_UP, KEY_DOWN];
        keys.map(|key| {
            let (axis, add) = match key {
                KEY_LEFT => ("x", false),
                KEY_RIGHT => ("x", true),
                KEY_UP => ("y", false),
                KEY_DOWN => ("y", true),
                _ => ("x", true),
            };
            KeyMap { key, axis, add }
        })
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Sample Game")
        .vsync()
        .build();

    let mut ball = Ball {
        position: Vector2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
        speed: 3.0,
        radius: 40.0,
        color: Color::RED,
    };

    while !rl.window_should_close() {
        for keymap in ball.keymap().iter() {
            if rl.is_key_down(keymap.key) {
                ball.apply_movement(&keymap.axis, keymap.add);
            }
        }

        if rl.is_key_pressed(KEY_SPACE) {
            ball.color = match ball.color {
                c if c == Color::RED => Color::BLUE,
                _ => Color::RED,
            };
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_circle(
            ball.position.x as i32,
            ball.position.y as i32,
            ball.radius,
            ball.color,
        );
    }
}
