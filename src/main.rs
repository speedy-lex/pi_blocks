use raylib::prelude::*;

fn main() {
    let (mut rl, rt) = init()
        .vsync()
        .size(1200, 1000)
        .build();
    let rl_audio = RaylibAudio::init_audio_device().unwrap();
    let clack = rl_audio.new_sound_from_wave(&rl_audio.new_wave_from_memory(".wav", include_bytes!("clack.wav")).unwrap()).unwrap();

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
    let mut collisions: usize = 0;
    let mut is_next_block_block = true;

    while !rl.window_should_close() {
        let dt = rl.get_frame_time() as f64;
        let mut rem_t = dt;
        while rem_t > 0.0 {
            let collision_t = if is_next_block_block {
                get_block_block_collision_t(&block1, &block2)
            } else {
                get_wall_block_collision_t(&block1)
            };

            let collision = collision_t <= rem_t && collision_t >= 0.0;

            if collision {
                clack.play();
                rem_t -= collision_t;
                collisions += 1;

                block1.pos += block1.vel * collision_t;
                block2.pos += block2.vel * collision_t;

                if is_next_block_block {
                    collide_blocks(&mut block1, &mut block2);
                } else {
                    collide_wall_block(&mut block1);
                }
                is_next_block_block = !is_next_block_block;
            } else {
                block1.pos += block1.vel * rem_t;
                block2.pos += block2.vel * rem_t;
                break;
            }
            if collisions % 1000000 == 0 {
                println!("{rem_t}: {collisions}");
            }
        }

        let mut d = rl.begin_drawing(&rt);
        d.clear_background(Color::BLACK);

        block1.draw(&mut d, 1000);
        block2.draw(&mut d, 1000);

        d.draw_rectangle(0, 0, 100, 1000, Color::DARKBLUE);
        d.draw_fps(10, 10);
        d.draw_text(&format!("{collisions}"), 10, 30, 20, Color::GREEN);
    }
}

fn collide_wall_block(block: &mut Block) {
    block.vel = -block.vel; // fully reflect velocity
}
fn get_wall_block_collision_t(block: &Block) -> f64 {
    (block.pos - block.size/2.0) / -block.vel // distance of leading edge to wall / velocity
}

fn collide_blocks(block1: &mut Block, block2: &mut Block) {
    // derived from conservation of energy and momentum
    let combined_mass = block1.mass + block2.mass;
    let block1_new_v = ((block1.mass - block2.mass) * block1.vel + 2.0 * block2.mass * block2.vel) / combined_mass;
    let block2_new_v = ((block2.mass - block1.mass) * block2.vel + 2.0 * block1.mass * block1.vel) / combined_mass;
    
    block1.vel = block1_new_v;
    block2.vel = block2_new_v;
}
fn get_block_block_collision_t(block1: &Block, block2: &Block) -> f64 {
    let total_vel = block1.vel - block2.vel;
    let dist = (block2.pos - block2.size/2.0) - (block1.pos + block1.size/2.0);
    dist / total_vel
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
        d.draw_rectangle((self.pos+100.0-self.size/2.0) as i32, h - self.size as i32, self.size as i32, self.size as i32, Color::BLUE);
    }
}