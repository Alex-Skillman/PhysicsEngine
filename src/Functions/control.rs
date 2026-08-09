use crate::structs::structs::*;
use macroquad::input::KeyCode::R;
use macroquad::input::KeyCode::D;
use macroquad::input::KeyCode::A;
use macroquad::input::KeyCode::J;
use macroquad::input::KeyCode::K;
use macroquad::input::KeyCode::Down;
use macroquad::input::KeyCode::Up;
use macroquad::input::KeyCode::LeftShift;
use macroquad::input::MouseButton::Left;
use crate::*;

//Control functions

//Base functiion
pub fn is_mouse_inside_ball(mouse_x: f32, mouse_y: f32, ball: &Ball) -> bool {
    //Detection of the mouse inside of a ball

    let dx = mouse_x - ball.x;
    let dy = mouse_y - ball.y;
    (dx * dx + dy * dy) <= ball.radius * ball.radius
}
//Moving ball
pub fn set_ball_to_mouse(mouse_x: f32, mouse_y: f32, ball: &mut Ball) {
    //Changes the speed that the ball reacts 0.0 for no movement 1.0 for snapping
    //Closer to 0.0 the looser it is and worse at latching on to mouse
    let follow_speed: f32 = 0.9;

    //Finds distance of ball to mouse
    let dx = mouse_x - ball.x;
    let dy = mouse_y - ball.y;

    //Scale the velocity change by the distance which will support throwing of the balls
    ball.x_velocity = dx * 0.3;
    ball.y_velocity = dy * 0.3;
    ball.x += (mouse_x - ball.x) * follow_speed;
    ball.y += (mouse_y - ball.y) * follow_speed;
}
//Spawning balls on mouse
pub fn ball_spawning(mouse_x: f32, mouse_y: f32, balls: &mut Vec<Ball>) {
    //Balls spawning once per frame when RMB clicked
    //Generates a lot of balls very fast


    if is_mouse_button_down(MouseButton::Right) == true {
        balls.push(generate_ball(mouse_x, mouse_y));
    }
}
//Moving ball with mouse
pub fn ball_movement(held_ball: &mut Option<usize>, mouse_x: f32, mouse_y: f32, balls: &mut Vec<Ball>) {
    //Ball movement


    //Adds any balls when mouse is pressed on it to the held_ball vector
    if is_mouse_button_pressed(MouseButton::Left) {
        //.iter() gives a  reference to each ball in Balls
        //.position() returns the position of a ball that is under the mouse
        *held_ball = balls
            .iter()
            .position(|ball| is_mouse_inside_ball(mouse_x, mouse_y, ball));
    }
    //Balls can be moved when pressed inside of
    if is_mouse_button_down(MouseButton::Left) == true {
        //The index i gives a mutable reference to the specific held ball
        if let Some(i) = held_ball {
            set_ball_to_mouse(mouse_x, mouse_y, &mut balls[*i]);
        } else {
            *held_ball = None;
        }
    }

}
//Removal of ball under mouse
pub fn ball_removal(held_ball: &mut Option<usize>, mouse_x: f32, mouse_y: f32, balls: &mut Vec<Ball>) {
    //Ball Removal on LMB + 'R'


    if is_key_down(R) && is_mouse_button_down(Left) {
        if let Some(i) = balls
            .iter()
            .position(|ball| is_mouse_inside_ball(mouse_x, mouse_y, ball))
        {
            balls.remove(i);
            *held_ball = None;
        }
    }
}
//Remove all ball
pub fn remove_all_balls(balls: &mut Vec<Ball>) {
    //Removes all balls when L-Shift + 'R' pressed

    if is_key_down(R) && is_key_down(LeftShift) {
        balls.clear();
    }
}
//Attract balls
pub fn ball_attraction(balls: &mut Vec<Ball>, mouse_x: f32, mouse_y: f32) {
    if is_key_down(A) && is_mouse_button_down(Left) {
        //Radius where balls are not attracted
        let max_distance: f32 = 225.0;
        //Changes the amount of change the ball experiences
        let pull_strenth: f32 = 2.0;
        //Used to dampen ball movement overtime
        let dampen_const: f32 = 0.90;

        for ball in balls.iter_mut() {
            //Find dx and dy to vary the pulling power
            let dx: f32 = mouse_x - ball.x;
            let dy: f32 = mouse_y - ball.y;
            
            //Find ball distance to exclude balls from
            let distance = ((dx*dx)+(dy*dy)).sqrt();

            if distance <= max_distance && distance > 0.001 {
                //Normalize the dx and dy by distance to let me scale by a power and not just by distance
                let ndx: f32 = dx / distance;
                let ndy: f32 = dy / distance;

                //Change the ball velocity by the pull strength
                ball.x_velocity += ndx * pull_strenth;
                ball.y_velocity += ndy * pull_strenth;

                //Multiply the ball velocity by the dampening constant to get a dampening effect
                ball.x_velocity *= dampen_const;
                ball.y_velocity *= dampen_const;
            }
        }
    }
}
//Dispell balls
pub fn ball_dispelation(balls: &mut Vec<Ball>, mouse_x: f32, mouse_y: f32) {
    if is_key_down(D) && is_mouse_button_down(Left) {
        //Radius where balls are not attracted
        let max_distance: f32 = 225.0;
        //Changes the amount of change the ball experiences
        let pull_strenth: f32 = 2.0;

        for ball in balls.iter_mut() {
            //Find dx and dy to vary the pulling power
            let dx: f32 = mouse_x - ball.x;
            let dy: f32 = mouse_y - ball.y;
            
            //Find ball distance to exclude balls from
            let distance = ((dx*dx)+(dy*dy)).sqrt();

            if distance <= max_distance && distance > 0.001 {
                //Normalize the dx and dy by distance to let me scale by a power and not just by distance
                let ndx: f32 = dx / distance;
                let ndy: f32 = dy / distance;

                //Change the ball velocity by the pull strength
                ball.x_velocity -= ndx * pull_strenth;
                ball.y_velocity -= ndy * pull_strenth;
            }
        }
    }
}
//Change radius based on input while ball held
pub fn radius_modification(held_ball: &mut Option<usize>,balls: &mut Vec<Ball>) {
    //Const for the amount that the size changes per frame
    let change_speed: f32 = 0.1;

    if let Some(i) = held_ball { //Gets the reference to the position of the ball being held
        if let Some(ball) = balls.get_mut(*i) { //Gets the ball and gives me a way to change radius
            if is_key_down(Up) || is_key_down(K) {
                ball.radius += change_speed;
            }
            if is_key_down(Down) || is_key_down(J) {
                ball.radius -= change_speed;
            }
        }
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_ball(x: f32, y: f32, radius: f32) -> Ball {
        Ball {
            x,
            y,
            x_velocity: 0.0,
            y_velocity: 0.0,
            radius,
            mass: 1.0,
            restitution: 0.0,
            ball_color: macroquad::color::WHITE,

        }
    }

    //Testing is_mouse inside ball
    #[test]
    fn mouse_directly_ball_center() {
        let ball1 = make_ball(0.0, 0.0, 15.0);
        let mouse_x: f32 = 0.0;
        let mouse_y: f32 = 0.0;

        assert!(is_mouse_inside_ball(mouse_x, mouse_y, &ball1));
    }
    #[test]
    fn mouse_slightly_off_center() {
        let ball1 = make_ball(0.0, 0.0, 15.0);
        let mouse_x: f32 = 5.0;
        let mouse_y: f32 = 5.0;

        assert!(is_mouse_inside_ball(mouse_x, mouse_y, &ball1));
    }
    #[test]
    fn mouse_directly_on_edge() {
        let ball1 = make_ball(0.0, 0.0, 10.0);
        let mouse_x: f32 = 5.0;
        let mouse_y: f32 = 5.0;

        assert!(is_mouse_inside_ball(mouse_x, mouse_y, &ball1));
    }
    #[test]
    fn mouse_way_off_ball() {
        let ball1 = make_ball(0.0, 0.0, 15.0);
        let mouse_x: f32 = 30.0;
        let mouse_y: f32 = 30.0;

        assert!(!is_mouse_inside_ball(mouse_x, mouse_y, &ball1));
    }
}
