extern crate gilrs;
extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::Rng;

fn random_bright_color() -> [f32;4] {
    let mut rng = rand::thread_rng();
    let mut ret = [1.0, rng.gen_range(0.,1.), rng.gen_range(0.,1.), 1.0];
    let swap_idx = rng.gen_range(0,3);
    if swap_idx != 0 {
        ret[0] = ret[swap_idx];
        ret[swap_idx] = 1.0;
    }
    ret
}

struct Player {
    position: (f64,f64),
    old_position: (f64,f64),
    gamepad_id: usize,
    deadzone: f64,
    color: [f32;4],
    jump_button_was_pressed: bool,
}

impl Player {
    fn new(position: (f64,f64), gamepad_id: usize, deadzone: f64, color: [f32;4]) -> Player {
        Player {
            position: position,
            old_position: position,
            gamepad_id: gamepad_id,
            deadzone: deadzone,
            color: color,
            jump_button_was_pressed: false,
        }
    }
    fn control(&mut self, g : &gilrs::Gilrs) {
        self.old_position = self.position;
        let gps = g.gamepad(self.gamepad_id).state();
        let (dx,dy) = gps.left_stick;
        let mut multiplier = 1.;
        if gps.btn_south {
            if !self.jump_button_was_pressed {
                self.jump_button_was_pressed = true;
                multiplier = 100.;
            }
        } else {
            self.jump_button_was_pressed = false;
        }
        self.position.0 += apply_deadzone(self.deadzone, dx as f64) * multiplier;
        self.position.1 -= apply_deadzone(self.deadzone, dy as f64) * multiplier;
    }
    fn render(&self, e : &Event, w : &mut PistonWindow) {
        w.draw_2d(e, |c, g| {
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

fn get_player_count(g : &gilrs::Gilrs) -> usize {
    let mut i=0;
    while g.connected_gamepad(i).is_some() {
        i+=1;
    }
    i
}

fn reset_game(window: &mut PistonWindow, players: &mut Vec<Player>, e: &Event) {
    window.draw_2d(e, |_, g| {
        clear([0.0; 4], g);
    });
    players.iter_mut().map(|p| {
            p.color = random_bright_color();
        }).count(); // the count() is only to consume the iterator, so it actually iterates
}

fn main() {
    let window_size = (1700,900);
    let deadzone = 0.2;
    let mut gilrs_obj = gilrs::Gilrs::new();
    let mut players = Vec::new();
    for i in 0..get_player_count(&gilrs_obj) {
        players.push(Player::new(((200*(i+1)) as f64,100.), i, deadzone, [1.0,0.,0.,1.0]));
    }
    let mut window: PistonWindow =
        WindowSettings::new("Awesome Game", [window_size.0, window_size.1])
        .exit_on_esc(true).build().unwrap();
    let mut is_init = false; // Maybe there is a nicer way to do this. I hope, there is.
    while let Some(e) = window.next() {
        for _ in gilrs_obj.poll_events() {} // poll all events, so gilrs can update its internal state
        if !is_init {
            reset_game(&mut window, &mut players, &e);
            is_init = true;
        }
        if players.iter().any(|p| gilrs_obj.gamepad(p.gamepad_id).state().btn_east) {
            reset_game(&mut window, &mut players, &e);
        }
        for p in players.iter_mut() {
            p.control(&gilrs_obj);
            p.render(&e, &mut window);
        }
        let mut killed : Vec<usize> = Vec::new();
        for p in players.iter() {
            for (qi,q) in players.iter().enumerate() {
                if p.gamepad_id == q.gamepad_id {
                    continue;
                }
                let a = p.position.0 < q.position.0;
                let b = p.old_position.0 < q.position.0;
                let c = p.position.1 < q.position.1;
                let d = p.old_position.1 < q.position.1;
                if a != b && c != d {
                    // jumped over
                    killed.push(qi);
                }
            }
        }
        for k in killed.iter() {
            players[*k].color = [0.,0.,0.,1.];
        }
    }
}
