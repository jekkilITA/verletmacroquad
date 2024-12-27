use macroquad::prelude::*;

mod structure;
use structure::*;

const PARTICLES_NUMBER: u32 = 500;//max 250 debug / 1250 release
const STEPS_FOR_FRAME: u16 = 128;

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
    let mut constraint_position: Vec2;
    let mut frame_time: f32;
    let mut last_emission_timer: f32 = 0.0;
    let mut current_timer: f32;
    let min_emission_time: f32 = 0.1;
    let mut stopped = false;
    loop {
	clear_background(BLACK);
	game.screen_dimensions = Vec2::new(screen_width(),screen_height());
	delta_time = get_frame_time();
        fps = (10.0/delta_time).round()/10.0;
	frame_time = delta_time*1000.0;
	constraint_position = game.screen_dimensions/2.0;
	draw_poly(constraint_position.x,constraint_position.y,96,game.screen_dimensions.y/2.195,0.0,WHITE);

	current_timer = get_time() as f32;
	if is_mouse_button_down(MouseButton::Left)&&((current_timer-last_emission_timer)>min_emission_time) {
	    let new_particle = Particle::new(mouse_position().into(),hex_to_int("ff00ff"));
	    game.world.particles.push(new_particle);
	    last_emission_timer = get_time() as f32;
 	}
	if is_key_pressed(KeyCode::P) {
	    stopped = !stopped;
	}
	if !stopped {
	    for _step in 0..STEPS_FOR_FRAME {
		game.update(delta_time*(1.0/STEPS_FOR_FRAME as f32));
	    }
	}
	game.world.render();
        draw_text(&mut fps.to_string(), game.screen_dimensions.x*0.9625, 20.0, 30.0, WHITE);
	draw_text(&mut frame_time.round().to_string(), game.screen_dimensions.x*0.9625, 50.0, 30.0, WHITE);
	draw_text(&mut game.world.particles.len().to_string(),game.screen_dimensions.x*0.9625, 70.0, 30.0, WHITE);
	next_frame().await;
    }
}
