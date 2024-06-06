use engine::{
    self,
    graphics::{self, Renderer},
    math, Engine,
};

pub struct Mario {
    player: Player,
    level: Vec<Layer>,
}

pub enum Layer {
    Image(graphics::Sprite),
    Entity(Vec<String>),
    Tilemap(Tilemap),
}

pub struct Tilemap {
    tileset: Tileset,
    width: u32,
    height: u32,
    data: Vec<u32>,
}

pub struct Tileset {
    sprite: graphics::Sprite,
    tile_size: u32,
}

pub struct Player {
    sprite: graphics::Sprite,
    position: math::V2,
    power: Power,
}

pub enum Power {
    None,
    Big,
    Fire,
}

impl engine::Game for Mario {
    fn init(engine: &mut Engine) -> Self {
        let player_image = engine.renderer.load_image("mario.png").unwrap();
        let sprite = graphics::Sprite {
            image: player_image,
            origin: graphics::Origin::BOTTOM,
        };
        //let blocks = engine.load_sprite("blocks.png").unwrap();

        Self {
            player: Player {
                position: engine::math::V2 { x: 0.0, y: 0.0 },
                power: Power::None,
                sprite,
            },
            level: vec![],
        }
    }

    fn update(&mut self, engine: &mut Engine, delta: f32) -> () {}

    fn render(&self) -> Vec<(&graphics::Sprite, Vec<graphics::Instance>)> {
        vec![(
            &self.player.sprite,
            vec![graphics::Instance {
                position: self.player.position.as_array(),
                origin: self.player.sprite.origin.as_array(),
                depth: 0.0,
                rotation: 0.0,
                scale: 1.0,
                frame: 0,
            }],
        )]
    }
}

fn main() {
    pollster::block_on(engine::run::<Mario>());
}
