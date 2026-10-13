use crate::structs::structs::*;
use crate::*;

//Handles contact with border, bouncing settling, rolling settling, general speed decay, and wall friction
pub fn border_collision(ball: &mut Ball, window: &WindowData) {
    //Handles contact with all window borders, bounce settling, and window settling

    let bounce_settle_threshold: f32 = 0.001;
    let roll_settle_threshold: f32 = 0.1;
    let ground_friction: f32 = 0.9975;

    //Ball contact with the bottom window boarder
    if ball.y + ball.radius > window.height {
        ball.y = window.height - ball.radius;
        ball.y_velocity *= -ball.restitution;

        //Ball bounce settling when under the settle threashold
        if ball.y_velocity.abs() < bounce_settle_threshold {
            ball.y_velocity = 0.0;
        }

        //Ball roll settling when under the settle threashold
        if ball.x_velocity.abs() < roll_settle_threshold {
            ball.x_velocity = 0.0;
        }
    }

    //Ball friction with the floor
    if ball.y + ball.radius > window.height {
        ball.x_velocity *= ground_friction;
    }

    //Ball contact with the top window boarder
    if ball.y - ball.radius < 0.0 {
        ball.y = ball.radius;
        ball.y_velocity *= -ball.restitution;
    }

    // Ball contact with the side window boarder
    if ball.x - ball.radius < 0.0 {
        ball.x = ball.radius;
        ball.x_velocity *= -ball.restitution;
    } else if ball.x + ball.radius > window.width {
        ball.x = window.width - ball.radius;
        ball.x_velocity *= -ball.restitution;
    }
}
pub fn gravitational_acceleration(ball: &mut Ball, gravity: f32) {
    //Gradually increases falling speed and pushes the ball down based on the velocity

    let terminal_velocity = generate_terminal_velocity(ball);

    if ball.y_velocity < terminal_velocity * 50.0 {
        ball.y_velocity += gravity;
    }
    //Adds the current speed to change the position downward
    ball.y += ball.y_velocity;
}
pub fn horizontal_movement(ball: &mut Ball) {
    //Adds the horizontal speed to the horizontal position

    ball.x += ball.x_velocity
}
pub fn speed_decay(ball: &mut Ball) {
    //Has a slow speed decay to the x velocity that acts similar to the gravity forcing the ball into the ground

    ball.x_velocity *= 0.999;
}
