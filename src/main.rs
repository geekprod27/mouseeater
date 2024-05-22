use mouce::Mouse;
use std::{thread, time::{self}};
use rand::Rng;

fn calculer_angle(x1: f64,y1: f64,x2: f64,y2: f64) -> f64{
    let deltax: f64 = (x1 - x2).into();
    let deltay: f64 = (y1 - y2).into();
    let angle = deltay.atan2(deltax);

    return angle;
}



fn init_eater(xcrash: f64, ycrash: f64)
{
    println!("Init eater at x {} y {}", xcrash, ycrash)
}

// fn move_eater(xcrash: f64, ycrash: f64)
// {
//     println!("eater at x {} y {}", xcrash, ycrash)
// }

fn main() {
    let mut eater_x:f64 = 1000.0;
    let mut eater_y:f64 = 1000.0;
    let mut status = 1; // 1 = charger la sourie 2 = balade
    init_eater(eater_x, eater_y);
    let mouse_manager = Mouse::new();
    let mut rng = rand::thread_rng();
    loop {
        match status {
            1=> { // charger la sourie
                let mut dist:f64 = 1000.0;
                while dist > 20.0 {
                    match mouse_manager.get_position() {
                        Ok((x, y)) => {
                            let angle = calculer_angle(eater_x, eater_y, x.into(), y.into());
                            eater_x = eater_x + (-10.0) * angle.cos();
                            eater_y = eater_y + (-10.0) * angle.sin();
                            // move_eater(eater_x, eater_y);
                            
                            dist = ((eater_x - x as f64).powi(2) + (eater_y - y as f64).powi(2)).sqrt();
                            thread::sleep(time::Duration::from_millis(10));
                        }
                        Err(error) => println!("{error}")
                    }
                }
                match mouse_manager.get_position() {
                    Ok((x, y)) => {
                        let angle = calculer_angle(x.into(), y.into(), eater_x, eater_y);
                        for i in 0..200 {
                            let new_x = x as f64+ (i*2) as f64 * angle.cos();
                            let new_y = y as f64+ (i*2) as f64 * angle.sin();
                            mouse_manager.move_to(new_x as usize, new_y as usize).expect("Never gonna");
                            thread::sleep(time::Duration::from_millis(1));
                        }
                    }
                    Err(error) => println!("{error}")
                }
                status = 2;
            }
            2=> {
                let target_x:f64 = rng.gen_range(10.0, 2000.0);
                let target_y:f64 = rng.gen_range(10.0, 2000.0);

                let mut dist:f64 = 1000.0;
                while dist > 20.0 {
                    let angle = calculer_angle(eater_x, eater_y, target_x, target_y);
                    eater_x = eater_x + (-10.0) * angle.cos();
                    eater_y = eater_y + (-10.0) * angle.sin();
                    // move_eater(eater_x, eater_y);
                    
                    dist = ((eater_x - target_x).powi(2) + (eater_y - target_y).powi(2)).sqrt();
                    thread::sleep(time::Duration::from_millis(10));
                }

                let nbr: i32 = rng.gen_range(0, 20);
                if  nbr > 18 { 
                    println!("statut 1");
                    status = 1; 
                }
            }
            _=>{
                println!("pas encore dev")
            }
        }
    }
}
