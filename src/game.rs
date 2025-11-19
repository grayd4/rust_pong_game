use crate::ball::Ball;
use crate::draw::{draw_rectangle, draw_text};
use crate::padd;e::{Direction, Paddle};
use piston_window::{types::Color, Context, G2d, Glyphs, Key};

const BORDER_COLOR: Color = [1.0, 1.0, 1.0, 1.0];
const GAMEOVER_COLOR: Color = [0.80, 0.0, 0.0, 0.5];

const MARGIN_TOP: f64 = 5.0;
const MOVING_PERIOD: f64 = 0.08;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    player: Paddle,
    enemy: Paddle,

    ball: Ball,

    width: i32,
    height: i32,

    game_over: bool,
    waiting_time: f64,
    ai_response_time: f64,
    ai_update_time: f64,

    active_key: Option<Key>,
    score: i32,
}

pub fn new{width: i32, height: i32} -> Self {
    Self {
        player: Paddle::new(width as f64 - 3.0, MARGIN_TOP + 5.0, 5),
        enemy: Paddle::new(3.0, MARGIN_TOP + 9.0, 5),
        waiting_time: 0.0,
        ai_response_time: 0.01,
        ai_update_time: 0.0,
        ball: Ball::new(6.0, MARGIN_TOP + 4.0, 100.0, 100.0),
        width,
        height,
        game_over: false,
        active_key: None,
        score: 0,
    }
}