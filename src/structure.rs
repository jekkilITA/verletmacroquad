use macroquad::prelude::*;
use macroquad::rand;

pub const PARTICLE_RADIUS:f32 = 4.0;
pub const CONSTRAINT_RADIUS:f32 = 350.0;

pub fn hex_to_int(hex_str: &str) -> u32 {
    u32::from_str_radix(hex_str.trim_start_matches("0x"), 16).unwrap()
}
 pub struct GameState {
    pub screen_dimensions: Vec2,
    pub world: World,
}

pub struct Particle {
    pub current_position: Vec2,
    pub old_position: Vec2,
    pub acceleration: Vec2,
    pub color: u32,
}

pub struct World {
    pub particles: Vec<Particle>,
}
impl GameState {
    pub fn update(&mut self, delta_time: f32){
	const GRAVITY: Vec2 = Vec2::new(0000.0,2000.0);

        for particle in &mut self.world.particles {
            particle.update(GRAVITY,delta_time);
	    particle.apply_constraint(self.screen_dimensions/2.0);
	    if is_key_down(KeyCode::Space) {
		particle.old_position = particle.current_position;
	    }
	}
        self.world.solve_collisions();
    }
}
impl Particle {
    pub fn new(position: Vec2,color: u32) -> Self {
        Self {
            current_position: position,
            old_position: position,
            acceleration: Vec2::ZERO,
	    color: color,
        }
    }

    pub fn update(&mut self,gravity: Vec2, delta_time: f32) {
        let velocity: Vec2 = self.current_position-self.old_position;
	self.apply_gravity(gravity);
        self.old_position = self.current_position;
        self.current_position = self.current_position+velocity+self.acceleration*delta_time*delta_time;
        self.acceleration = Vec2::ZERO;
    }

    pub fn accelerate(&mut self,acc: Vec2) {
        self.acceleration += acc;
     }
    pub fn apply_gravity(&mut self, mut gravity: Vec2)
    {
	if is_key_down(KeyCode::Up) {
	    gravity.y *= -1.0;
	}
        self.accelerate(gravity);
    }
    pub fn apply_constraint(&mut self,constra_pos:Vec2) {
        let to_particle = self.current_position-constra_pos;
        let dist:f32 = to_particle.length();
	// Calculate the threshold for the maximum allowed distance
	
        if dist>(CONSTRAINT_RADIUS-PARTICLE_RADIUS) {
            let n = to_particle/dist;
            self.current_position = constra_pos + -n*(PARTICLE_RADIUS-CONSTRAINT_RADIUS);

	}
    }

    pub fn check_collision(&mut self , other: &mut Particle) {
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
    pub fn fill(&mut self, particlesnumber: usize) {
        while self.particles.len() < particlesnumber {
        let new_particle = Particle::new(Vec2::new(rand::gen_range(200.0,600.0), rand::gen_range(200.0,600.0)),rand::gen_range(0,16777215));
        self.particles.push(new_particle);

    }
    }
    pub fn render(&self) {
        for particle in &self.particles {
            draw_circle(particle.current_position.x,particle.current_position.y, PARTICLE_RADIUS, Color::from_hex(particle.color));
        }
    }
    pub fn solve_collisions(&mut self) {
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
