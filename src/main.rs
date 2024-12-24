use macroquad::prelude::*;
use macroquad::rand;

const PARTICLE_RADIUS:f32 = 15.0;
const CONSTRAINT_POSITION:Vec2 = Vec2::new(400.0,300.0);
const CONSTRAINT_RADIUS:f32 = 300.0;
const PARTICLES_NUMBER: u32 = 10;//max 1500


struct Particle {
    current_position: Vec2,
    old_position: Vec2,
    acceleration: Vec2,
}

struct World {
    particles: Vec<Particle>,
}

impl Particle {
    fn new(position: Vec2) -> Self {
        Self {
            current_position: position,
            old_position: position,
            acceleration: Vec2::ZERO,
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
    fn apply_constraint(&mut self,) {
        let to_particle = self.current_position-CONSTRAINT_POSITION;
        let dist:f32 = to_particle.length();
	// Calculate the threshold for the maximum allowed distance
	
        if dist>(CONSTRAINT_RADIUS-PARTICLE_RADIUS) {
            let n = to_particle/dist;
            self.current_position = CONSTRAINT_POSITION + -n*(PARTICLE_RADIUS-CONSTRAINT_RADIUS);
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
    fn update(&mut self, delta_time: f32) {
        const GRAVITY: Vec2 = Vec2::new(0.0,1000.0);
        for particle in &mut self.particles {
            particle.update(GRAVITY,delta_time);
	    particle.apply_constraint();
	}
        self.solve_collisions();
    }
    fn fill(&mut self, particlesnumber: usize) {
        while self.particles.len() < particlesnumber {
        let new_particle = Particle::new(Vec2::new(rand::gen_range(200.0,600.0), rand::gen_range(200.0,600.0)));
        self.particles.push(new_particle);

    }
    }
    fn render(&self) {
        for particle in &self.particles {
            draw_circle(particle.current_position.x,particle.current_position.y, PARTICLE_RADIUS, BLUE);
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
    let mut world = World {
        particles: Vec::new(),
    };

    world.fill(PARTICLES_NUMBER.try_into().unwrap());
    let mut delta_time: f32;
    let mut fps:f32;
    loop {
	delta_time = get_frame_time();
        fps = (10.0/delta_time).round()/10.0;
	let frame_time = get_frame_time()*1000.0;
        draw_poly(CONSTRAINT_POSITION.x,CONSTRAINT_POSITION.y,96,CONSTRAINT_RADIUS,0.0,WHITE);
	world.update(delta_time);

	world.render();
        draw_text(&mut fps.to_string(), 700.0, 20.0, 30.0, DARKGRAY);
	draw_text(&mut frame_time.to_string(), 700.0, 50.0, 30.0, DARKGRAY);
	next_frame().await;
    }
}
