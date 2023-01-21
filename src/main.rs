use lazy_static::lazy_static;
use std::sync::{Mutex};
/*
lazy_static! {
    static ref ARRAY: Mutex<Vec<u8>> = Mutex::new(vec![]);
}
*/
use bevy::prelude::*;
use rand::Rng;
//use bevy_inspector_egui::WorldInspectorPlugin;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

const TILES_X: f32 = 16.;
const TILES_Y: f32 = 9.;
const TILE_OFFSET: f32 = 5.;

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const FOOD_COLOR: Color = Color::RED;
const EMPTY_TILE_COLOR: Color = Color::rgb(0.4, 0.4, 0.4);

#[derive(Component)]
struct SnakeHead {
    vel: Vec2,
    score: u8,
    pos: Vec2,
    speed: Timer,
    tail: Vec<Vec2>,
}

#[derive(Component)]
struct Food {
    pos: Vec2,
}

#[derive(Component)]
struct Tile {
    pos: Vec2,
}

impl SnakeHead {
    fn velocity_y(&mut self, y: f32) {
        self.vel.y = y;
        self.vel.x = 0.;
    }
    fn velocity_x(&mut self, x: f32) {
        self.vel.x = x;
        self.vel.y = 0.;
    }
    fn add_score(&mut self) {
        self.score += 1;
    }
    fn update_pos(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
    }
}

lazy_static! {
    static ref SNAKE_HEAD: Mutex<SnakeHead> = Mutex::new(SnakeHead {
        vel: Vec2::new(0., 0.),
        score: 1,
        pos: Vec2::new((TILES_X as i32 / 2) as f32, (TILES_Y as i32 / 2) as f32),
        speed: Timer::from_seconds(0.5, TimerMode::Repeating),
        tail: vec![]
    });
}

lazy_static! {
    static ref FOOD: Mutex<Food> = Mutex::new(Food {
        pos: Vec2::new(0.,0.),
    });
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_tiles)
        .add_startup_system(food_start_pos)
        .add_system(snake_dead)
        .add_system(update_tiles)
        .add_system(eat_food)
        .add_system(snake_head_movement)
        //.add_system(eat_food)
        //.add_plugin(WorldInspectorPlugin::new())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WIDTH,
                height: HEIGHT,
                title: "Bevy Tower Defense".to_string(),
                resizable: false,
                ..Default::default()
            },
            ..default()
        }))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_tiles(mut commands: Commands) {
    for y_tall in 1..(TILES_Y as i32 + 1) {
        let y_tile: f32 = y_tall as f32;
        for x_tall in 1..(TILES_X as i32 + 1) {
            let x_tile: f32 = x_tall as f32;
            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: EMPTY_TILE_COLOR,
                        ..Default::default()
                    },
                    transform: Transform {
                        scale: Vec3::new(
                            WIDTH / TILES_X - TILE_OFFSET * 2.,
                            HEIGHT / TILES_Y - TILE_OFFSET * 2.,
                            0.,
                        ),
                        translation: Vec3::new(
                            WIDTH / TILES_X * x_tile - WIDTH / 2. - WIDTH / TILES_X / 2.,
                            HEIGHT / TILES_Y * y_tile - HEIGHT / 2. - HEIGHT / TILES_Y / 2.,
                            0.0,
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Tile {
                    pos: Vec2::new(x_tile, y_tile),
                });
        }
    }
}

fn snake_dead(){
    let mut snake = SNAKE_HEAD.lock().unwrap();
    if snake.tail.len() <= 1{
        return;
    }
    if snake.tail.contains(&snake.pos){
        //death
        snake.pos = Vec2::new((TILES_X as i32/2) as f32, (TILES_Y as i32/2) as f32);
        snake.score = 1;
        snake.tail = Vec::new();
    }
}

fn update_tiles(mut tile_query: Query<(&mut Sprite, &Tile)>) {
    let snake = SNAKE_HEAD.lock().unwrap();
    for (mut sprite, tile) in &mut tile_query {
        if snake.pos == tile.pos {
            sprite.color = SNAKE_HEAD_COLOR;
        }else if FOOD.lock().unwrap().pos == tile.pos {
            sprite.color = FOOD_COLOR;
        }else if snake.tail.contains(&tile.pos){
            sprite.color = SNAKE_HEAD_COLOR;
        }else{
            sprite.color = EMPTY_TILE_COLOR;
        }
        //println!("{}", new_food_pos(&snake));
    }
}

fn snake_head_movement(keys: Res<Input<KeyCode>>, time: Res<Time>) {
    let mut snake = SNAKE_HEAD.lock().unwrap();
    snake.speed.tick(time.delta());
    //println!("Snake Movement System was called!");



    if keys.just_pressed(KeyCode::W) {
        snake.velocity_y(1.);
    } else if keys.just_pressed(KeyCode::S) {
        snake.velocity_y(-1.);
    } else if keys.just_pressed(KeyCode::A) {
        snake.velocity_x(-1.);
    } else if keys.just_pressed(KeyCode::D) {
        snake.velocity_x(1.);
    }

    //println!("{}",snake.score);

    println!("{:?}", snake.tail);

    println!("{}", snake.pos);

    //makes sure that the snake doesn't go past the screen
    if TILES_X < snake.vel.x+snake.pos.x || TILES_Y < snake.vel.y+snake.pos.y || snake.pos.x+snake.vel.x <= 0. || snake.pos.y+snake.vel.y <= 0.{
        return;
    }

    if snake.speed.just_finished() {
        let pos = snake.pos;
        snake.tail.push(pos);
        if snake.score < snake.tail.len() as u8 {
            snake.tail.remove(0);        
        }
        snake.update_pos();
    }
}

/*
fn new_food_pos() -> Vec2 {
    let mut rng = rand::thread_rng();
    //let snake = SNAKE_HEAD.lock().unwrap();

    let mut new_pos = Vec2::new(
        rng.gen_range(1..TILES_X as i32) as f32,
        rng.gen_range(1..TILES_Y as i32) as f32,
    );
    
    loop {
        if new_pos == snake.pos || snake.tail.contains(&new_pos) {
            new_pos = Vec2::new(rng.gen_range(1.0..TILES_X), rng.gen_range(1.0..TILES_Y));
        } else {
            break;
        }
    }
    return new_pos;
}*/

fn food_start_pos(){
    let mut rng = rand::thread_rng();
    let snake = SNAKE_HEAD.lock().unwrap();

    let start_pos = {    let mut new_pos = Vec2::new(
        rng.gen_range(1..TILES_X as i32) as f32,
        rng.gen_range(1..TILES_Y as i32) as f32,
    );
    
    loop {
        if new_pos == snake.pos || snake.tail.contains(&new_pos) {
            new_pos = Vec2::new(rng.gen_range(1.0..TILES_X), rng.gen_range(1.0..TILES_Y));
        } else {
            break;
        }
    }
    new_pos};

    FOOD.lock().unwrap().pos = start_pos;

    
}

fn eat_food(){
    let mut rng = rand::thread_rng();
    let mut snake = SNAKE_HEAD.lock().unwrap();
    let mut food = FOOD.lock().unwrap();

    if food.pos == snake.pos{
        snake.add_score();

        let next_pos = {    let mut new_pos = Vec2::new(
            rng.gen_range(1..TILES_X as i32) as f32,
            rng.gen_range(1..TILES_Y as i32) as f32,
        );
        
        loop {
            if new_pos == snake.pos || snake.tail.contains(&new_pos) {
                new_pos = Vec2::new(rng.gen_range(1.0..TILES_X), rng.gen_range(1.0..TILES_Y));
            } else {
                break;
            }
        }
        new_pos};

        food.pos = next_pos;
    }
}
