use crate::structs::structs::*;
use crate::*;

//Generation aspects of the simulator
pub fn generate_starting_balls(count: u64, width: f32, height: f32) -> Vec<Ball> {
    //Randomly generates a balls of a certain count inside of the inputted area

    (0..count)
        .map(|_| {
            let radius: f32 = gen_range(10.0, 20.0);

            Ball {
                x: gen_range(0.0, width),
                y: gen_range(0.0, height),
                x_velocity: gen_range(-3.0, 3.0),
                y_velocity: gen_range(-3.0, 3.0),
                radius: radius,
                mass: gen_range(0.01, 0.03) * radius * radius,
                restitution: 0.9,
                ball_color: generate_colors(),
            }
        })
        .collect()
}
pub fn generate_ball(x: f32, y: f32) -> Ball {
    //Generates a single ball per function call at a specificed coordinate

    let radius: f32 = gen_range(10.0, 20.0);

    Ball {
        x: x,
        y: y,
        x_velocity: gen_range(-3.0, 3.0),
        y_velocity: gen_range(-3.0, 3.0),
        radius: radius,
        mass: gen_range(0.01, 0.03) * radius * radius,
        restitution: 0.9,
        ball_color: generate_colors(),
    }
}
pub fn generate_colors() -> Color {
    //Generate a new color for each ball with a maximum opacity

    Color {
        r: gen_range(0.0, 1.0),
        g: gen_range(0.0, 1.0),
        b: gen_range(0.0, 1.0),
        a: 1.0,
    }
}
pub fn generate_terminal_velocity(ball: &mut Ball) -> f32 {
    let air_density: f32 = 1.225;
    let drag_coefficient: f32 = 0.47;
    let gravity_accel: f32 = 9.8; // real-world gravity, separate from your per-frame "gravity" constant
    let area: f32 = std::f32::consts::PI * ball.radius * ball.radius;

    (2.0 * ball.mass * gravity_accel / (air_density * drag_coefficient * area)).sqrt()
}
