extern crate ggez;
use ggez::*;
extern crate rand;

use rand::Rng;

struct Game {
    snake: Vec<(f32, f32)>,
    apple: (f32, f32),
    direction: Direction,
    square_length: f32,
    plz_apple: bool,
    dead: bool
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const SQUARES: f32 = 45.0;
const FPS: u32 = 5;


impl Game {
    fn build_mesh(&mut self, ctx: &mut Context) -> GameResult<graphics::Mesh> {
        let mb = &mut graphics::MeshBuilder::new();
        for bod in &self.snake {
            let top_left = graphics::Point2::new(bod.0*self.square_length, bod.1*self.square_length);
            let top_right = graphics::Point2::new(bod.0*self.square_length + self.square_length, bod.1*self.square_length);
            let bottom_right = graphics::Point2::new(bod.0*self.square_length + self.square_length, bod.1*self.square_length + self.square_length);
            let bottom_left = graphics::Point2::new(bod.0*self.square_length, bod.1*self.square_length + self.square_length);
            mb.polygon(graphics::DrawMode::Fill, &[top_left, top_right, bottom_right, bottom_left]);
        }

        mb.build(ctx)
    }
    fn update_snake(&mut self) {
        let keymove = match self.direction{
            Direction::Right => (self.snake[0].0 + 1.0, self.snake[0].1),
            Direction::Left => (self.snake[0].0 - 1.0, self.snake[0].1),
            Direction::Down => (self.snake[0].0, self.snake[0].1 + 1.0),
            Direction::Up => (self.snake[0].0, self.snake[0].1 - 1.0),
        };

        if self.snake.contains(&keymove) || keymove.0 < 0.0 || 
            keymove.0 > 100.0 ||
            keymove.1 < 0.0 || 
            keymove.1 > 100.0 {
            self.dead = true;
            return;
        }

        if keymove == self.apple {
            self.plz_apple = true;
        } else {
            self.snake.pop();
        }
        self.snake.insert(0, keymove);
    }

    fn generate_apple(&mut self) {
        let mut rng = rand::thread_rng();
        self.apple = (rng.gen_range::<u32>(0, 10) as f32, rng.gen_range::<u32>(0, 10) as f32);
        while self.snake.contains(&self.apple) {
            self.apple = (rng.gen_range::<u32>(0, 10) as f32, rng.gen_range::<u32>(0, 10) as f32);
        }
    }
}

impl ggez::event::EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, FPS) {
            if self.dead {
                let _ = ctx.quit();
            }
            self.update_snake();

            if self.plz_apple {
                self.generate_apple();
                self.plz_apple = false;
                println!("PLZ APPLE");
            }
            println!("Snake: {:?}", self.snake);
            // println!("Apple: {:?}", self.snake);

        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::set_color(ctx, (0, 0, 255).into())?;

        let mesh = self.build_mesh(ctx)?;
        graphics::draw_ex(ctx, &mesh, Default::default())?;

        graphics::set_color(ctx, graphics::Color::new(0.1, 0.0, 0.4, 1.0))?;

        graphics::rectangle(ctx, 
            graphics::DrawMode::Fill, 
            graphics::Rect::new(self.apple.0*self.square_length, self.apple.1*self.square_length, self.square_length, self.square_length)
            )?;

        graphics::present(ctx);
        timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: event::Keycode, _keymod: event::Mod, _repeat: bool) {
        match keycode {
            event::Keycode::Left => self.direction = Direction::Left,
            event::Keycode::Right => self.direction = Direction::Right,
            event::Keycode::Up => self.direction = Direction::Up,
            event::Keycode::Down => self.direction = Direction::Down,
            _ => (),
        }
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("hello_ggez", "awesome_person", c).unwrap();
    let state = &mut Game { 
        snake: vec![(6.0, 5.0), (5.0, 5.0)], 
        apple: (100.0,100.0), 
        direction: Direction::Right,
        square_length: (ctx.conf.window_mode.width as f32)/SQUARES,
        dead: false,
        plz_apple: true,
        };
    
    event::run(ctx, state).unwrap();
}
