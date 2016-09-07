extern crate gilrs;
extern crate piston_window;

use piston_window::*;

fn apply_deadzone(dead : f64, mut value : f64) -> f64 {
    let s = value.signum();
    if s == -1.0 {
        value = -value;
    }
    if value <= dead {
        0_f64
    } else {
        (value-dead)*(1_f64+dead)*s
    }
}

fn main() {
    let deadzone = 0.2;
    let mut gilrs_obj = gilrs::Gilrs::new();
    let mut x = 0_f64;
    let mut y = 0_f64;
    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();
    while let Some(e) = window.next() {
        for _ in gilrs_obj.poll_events() {}
        let (dx,dy) = gilrs_obj.gamepad(0).state().left_stick;
        x += apply_deadzone(deadzone, dx as f64);
        y -= apply_deadzone(deadzone, dy as f64);
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 10.0, 10.0],
                      c.transform.trans(x,y), g);
        });
    }
}
