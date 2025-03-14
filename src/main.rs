use raylib::prelude::*;

fn main() {
    let (mut rl, rt) = init()
        .size(1200, 1000)
        .build();

    let mut block1 = Block {
        pos: 150.0,
        vel: 0.0,
        size: 100.0,
        mass: 1.0
    };
    let mut block2 = Block {
        pos: 750.0,
        vel: -100.0,
        size: 400.0,
        mass: 100.0,
    };
    let mut collisions = 0;
    rl.set_target_fps(10000);

    while !rl.window_should_close() {
        let dt = rl.get_frame_time() as f64;
        let block1_np = dt*block1.vel + block1.pos;
        let block2_np = dt*block2.vel + block2.pos;
        let block1_range = (block1_np-block1.size/2.0)..(block1_np+block1.size/2.0);
        let block2_range = (block2_np-block2.size/2.0)..(block2_np+block2.size/2.0);

        if block1_range.contains(&block2_range.start) || block1_range.contains(&block2_range.end) {
            collisions += 1;

            let total_vel = block1.vel - block2.vel;
            let dist = (block2.pos - block2.size/2.0) - (block1.pos + block1.size/2.0);
            let t = dist / total_vel;

            assert!(t > 0.0);
            assert!(t <= dt);

            block1.pos += block1.vel * t;
            block2.pos += block2.vel * t;

            let rem_t = dt - t;

            let combined_mass = block1.mass + block2.mass;
            let block1_new_v = ((block1.mass - block2.mass) * block1.vel + 2.0 * block2.mass * block2.vel) / combined_mass;
            let block2_new_v = ((block2.mass - block1.mass) * block2.vel + 2.0 * block1.mass * block1.vel) / combined_mass;
            
            block1.vel = block1_new_v;
            block2.vel = block2_new_v;

            update_wall_block(&mut block1, rem_t, &mut collisions);
            block2.pos += block2.vel * rem_t;            
        } else {
            update_wall_block(&mut block1, dt, &mut collisions);
            block2.pos = block2_np;
        }

        let mut d = rl.begin_drawing(&rt);
        d.clear_background(Color::BLACK);

        block1.draw(&mut d, 1000);
        block2.draw(&mut d, 1000);

        d.draw_rectangle(0, 0, 100, 1000, Color::WHITE);
        d.draw_fps(10, 10);
        d.draw_text(&format!("{collisions}"), 10, 30, 20, Color::GREEN);
    }
}


fn update_wall_block(block: &mut Block, t: f64, collisions: &mut usize) {
    let new_pos = block.pos + block.vel * t;
    if new_pos >= block.size/2.0 {
        block.pos = new_pos;
        return;
    }
    *collisions += 1;

    let coll_t = (block.pos - block.size/2.0) / -block.vel;

    assert!(coll_t > 0.0);
    assert!(coll_t <= t);

    let rem_t = t - coll_t;

    block.vel = -block.vel;
    // FIXME: we need recursive collision checking like collide and slide :/ (do the collisions on line 30-53 here as well)
    // ignoring rem_t is fine here since the velocities are correct but we *should* use it
    // block.pos = block.size / 2.0 + block.vel * rem_t;
}

#[derive(Debug, Clone, Copy)]
struct Block {
    pos: f64,
    vel: f64,
    size: f64,
    mass: f64,
}
impl Block {
    fn draw(&self, d: &mut RaylibDrawHandle, h: i32) {
        d.draw_rectangle((self.pos+100.0-self.size/2.0) as i32, h - self.size as i32, self.size as i32, self.size as i32, Color::WHEAT);
    }
}