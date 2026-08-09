use macroquad::color::Color;

pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub x_velocity: f32,
    pub y_velocity: f32,
    pub radius: f32,
    pub mass: f32,
    pub restitution: f32,
    pub ball_color: Color,
}
pub struct WindowData {
    pub width: f32,
    pub height: f32,
}
pub struct Coordinates {
    pub x: usize,
    pub y: usize,
}
pub struct Grid {
    pub cell_size: usize,
    pub cell_coordinates: Coordinates,
    pub ball_inside: Vec<usize>,
}
