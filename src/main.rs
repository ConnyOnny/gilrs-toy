extern crate gilrs;
extern crate piston_window;

use piston_window::*;

struct Player {
    position: (f64,f64),
    gamepad_id: usize,
    deadzone: f64,
    color: [f32;4],
}

impl Player {
    fn control(&mut self, g : &gilrs::Gilrs) {
        let (dx,dy) = g.gamepad(self.gamepad_id).state().left_stick;
        self.position.0 += apply_deadzone(self.deadzone, dx as f64);
        self.position.1 -= apply_deadzone(self.deadzone, dy as f64);
    }
    fn render(&self, e : &Event, w : &mut PistonWindow) {
        w.draw_2d(e, |c, g| {
            clear([1.0; 4], g);
            rectangle(self.color,
                      [0.0, 0.0, 10.0, 10.0],
                      c.transform.trans(self.position.0,self.position.1), g);
        });
    }
}

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
    let window_size = (800,600);
    let deadzone = 0.2;
    let mut players = vec![Player{position:(0.,0.),gamepad_id:0,deadzone:deadzone,color:[0.,0.,1.,1.]}];
    let mut gilrs_obj = gilrs::Gilrs::new();
    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [window_size.0, window_size.1])
        .exit_on_esc(true).build().unwrap();
    while let Some(e) = window.next() {
        for _ in gilrs_obj.poll_events() {}
        for p in players.iter_mut() {
            p.control(&gilrs_obj);
            p.render(&e, &mut window);
        }
    }
}
