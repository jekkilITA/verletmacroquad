use macroquad::prelude::*;
use macroquad::rand;

const PARTICLE_RADIUS:f32 = 3.0;
const CONSTRAINT_RADIUS:f32 = 300.0;
const PARTICLES_NUMBER: u32 = 400;//max 250
const STEPS_FOR_FRAME: u8 = 8;

fn hex_to_int(hex_str: &str) -> u32 {
    u32::from_str_radix(hex_str.trim_start_matches("0x"), 16).unwrap()
}

struct GameState {
    screen_dimensions: Vec2,
    world: World,
}



struct Particle {
    current_position: Vec2,
    old_position: Vec2,
    acceleration: Vec2,
     color: u32,
}

struct World {
    particles: Vec<Particle>,
}
impl GameState {
    fn update(&mut self, delta_time: f32){
	const GRAVITY: Vec2 = Vec2::new(0.0,1000.0);
        for particle in &mut self.world.particles {
            particle.update(GRAVITY,delta_time);
	    particle.apply_constraint(self.screen_dimensions/2.0);
	}
        self.world.solve_collisions();
    }
}
impl Particle {
    fn new(position: Vec2,color: u32) -> Self {
        Self {
            current_position: position,
            old_position: position,
            acceleration: Vec2::ZERO,
	    color: color,
        }
    }

    fn update(&mut self,gravity: Vec2, delta_time: f32) {
        let velocity: Vec2 = self.current_position-self.old_position;
	self.apply_gravity(gravity);
        self.old_position = self.current_position;
        self.current_position = self.current_position+velocity+self.acceleration*delta_time*delta_time;
        self.acceleration = Vec2::ZERO;
    }

    fn accelerate(&mut self,acc: Vec2) {
        self.acceleration += acc;
     }
    fn apply_gravity(&mut self, gravity: Vec2)
    {
        self.accelerate(gravity);
    }
    fn apply_constraint(&mut self,constra_pos:Vec2) {
        let to_particle = self.current_position-constra_pos;
        let dist:f32 = to_particle.length();
	// Calculate the threshold for the maximum allowed distance
	
        if dist>(CONSTRAINT_RADIUS-PARTICLE_RADIUS) {
            let n = to_particle/dist;
            self.current_position = constra_pos + -n*(PARTICLE_RADIUS-CONSTRAINT_RADIUS);
	}
    }

    fn check_collision(&mut self , other: &mut Particle) {
	let collision_axis: Vec2 = self.current_position - other.current_position;
	let dist: f32 =collision_axis.length();
	if dist < PARTICLE_RADIUS*2.0 {
	   let  n: Vec2 = collision_axis.normalize();
	    let delta:f32 = PARTICLE_RADIUS*2.0-dist;
	    self.current_position += 0.5*delta*n;
	    other.current_position -= 0.5*delta*n;
	}
    }
}


impl World {
    fn fill(&mut self, particlesnumber: usize) {
        while self.particles.len() < particlesnumber {
        let new_particle = Particle::new(Vec2::new(rand::gen_range(200.0,600.0), rand::gen_range(200.0,600.0)),hex_to_int("2337c6"));
        self.particles.push(new_particle);

    }
    }
    fn render(&self) {
        for particle in &self.particles {
            draw_circle(particle.current_position.x,particle.current_position.y, PARTICLE_RADIUS, Color::from_hex(particle.color));
        }
    }
    fn solve_collisions(&mut self) {
        let num_particles: usize = self.particles.len();

        for i in 0..num_particles {
            for j in (i+1)..num_particles {
		let (p1,p2) = self.particles.split_at_mut(j);
                let particle1 = &mut p1[i];
                let particle2 = &mut p2[0];
                particle1.check_collision(particle2);
            }
        }
    }
}


#[macroquad::main("Verlet")]
async fn main() {
    let world = World {
        particles: Vec::new(),
    };
    let mut game = GameState {
	screen_dimensions: Vec2::new(800.0,600.0),
	world: world,
    };
    game.world.fill(PARTICLES_NUMBER.try_into().unwrap());
    set_fullscreen(true);
    let mut delta_time: f32;
    let mut fps:f32;
    loop {
	game.screen_dimensions = Vec2::new(screen_width(),screen_height());
	let constraint_position:Vec2 = game.screen_dimensions/2.0;
	delta_time = get_frame_time();
        fps = (10.0/delta_time).round()/10.0;
	let frame_time = get_frame_time()*1000.0;
        draw_poly(constraint_position.x,constraint_position.y,96,CONSTRAINT_RADIUS,0.0,WHITE);
	for _step in 0..STEPS_FOR_FRAME {
	game.update(delta_time*(1.0/STEPS_FOR_FRAME as f32));
	}
	game.world.render();
        draw_text(&mut fps.to_string(), 700.0, 20.0, 30.0, DARKGRAY);
	draw_text(&mut frame_time.to_string(), 700.0, 50.0, 30.0, DARKGRAY);
	next_frame().await;
    }
}
