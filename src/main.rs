use mouce::Mouse;
use std::{thread, time::{self, Duration}};

fn calculerAngle(x1: i32,y1: i32,x2: i32,y2: i32) -> f64{
    let deltax: f64 = (x1 - x2).into();
    let deltay: f64 = (y1 - y2).into();
    let angle = deltay.atan2(deltax);

    return angle;
}

fn move_to_angle(angle: f64, dist: i32, initial_x: i32, initial_y: i32) {
    let new_x = initial_x as f64 + dist as f64 * angle.cos();
    let new_y = initial_y as f64 + dist as f64 * angle.sin();
    let mouse_manager = Mouse::new();
    mouse_manager.move_to(new_x as usize, new_y as usize).expect("Never gonna");
}

fn crasheur(xcrash: i32, ycrash: i32) {
    let dist: [i32; 5] = [100, 200, 300, 400, 500];
    let mouse_manager = Mouse::new();
    match mouse_manager.get_position() {
        Ok((x, y)) => {
            let angle = calculerAngle(x, y, xcrash, ycrash);
            for i in 0..5 {
                move_to_angle(angle, dist[i], x, y);
                thread::sleep(time::Duration::from_millis(100));
            }
        }
        Err(error) => println!("{error}")
    }
}

fn main() {
    crasheur(0, 0)
}
