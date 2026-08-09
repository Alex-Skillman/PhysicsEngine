use crate::*;
use crate::structs::structs::*;

pub fn draw_screen_text(balls: &Vec<Ball>) {
    //Draw FPS on the screen
    draw_text(
        &format!("FPS: {}", get_fps()),
        10.0, //x position
        30.0, //y position
        40.0, //font size
        WHITE //Font color
    );

    //Draw ball counter on the screen
    draw_text(
        &format!("Balls: {}", balls.len()),
        10.0, //x position
        60.0, //y position
        40.0, //font size
        WHITE //Font color
    );
}
