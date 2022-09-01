use rusty_engine::prelude::*;
use rand::prelude::*;

struct GameState {
    high_score: u32,
    score: u32,
    car_index: i32,
    spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            high_score: 0,
            score: 0,
            car_index: 0,
            spawn_timer: Timer::from_seconds(2.0, true),
        }
    }
}

fn main() {
    let mut game = Game::new();
    
    game.audio_manager.play_music(MusicPreset::WhimsicalPopsicle, 0.25);

    let player = game.add_sprite("player", SpritePreset::RollingBallBlue);
    player.translation = Vec2::new(0.0, 0.0);
    player.collision = true;
    
    let score = game.add_text("score", "Score: 0");
    score.translation = Vec2::new(520.0, 340.0);

    let high_score = game.add_text("high_score", "High Score: 0");
    high_score.translation = Vec2::new(-520.0, 340.0);
    
    game.add_logic(game_logix);
    game.run(GameState::default());
}

fn game_logix(engine: &mut Engine, game_state: &mut GameState) {
    
    let score = engine.texts.get_mut("score").unwrap();
    score.translation.x = engine.window_dimensions.x / 2.0 - 80.0;
    score.translation.y = engine.window_dimensions.y / 2.0 - 30.0;

    let high_score = engine.texts.get_mut("high_score").unwrap();
    high_score.translation.x = -engine.window_dimensions.x / 2.0 + 110.0;
    high_score.translation.y = engine.window_dimensions.y / 2.0 - 30.0;
    
    for event in engine.collision_events.drain(..) {
        if event.state == CollisionState::Begin && event.pair.one_starts_with("player") {
            for label in [event.pair.0, event.pair.1] {
                if label != "player" {
                    engine.audio_manager.play_sfx(SfxPreset::Minimize1, 0.5);
                    engine.sprites.remove(&label);
                }
            }
            game_state.score += 1;
            // updating text
            let score = engine.texts.get_mut("score").unwrap();
            score.value = format!("Score: {}", game_state.score);
            
            if game_state.score > game_state.high_score {
                game_state.high_score = game_state.score;
                let high_score = engine.texts.get_mut("high_score").unwrap();
                high_score.value = format!("High Score: {}", game_state.high_score);
            }
        }
    }
    
    // quit game
    if engine.keyboard_state.pressed(KeyCode::Q) {
        engine.should_exit = true;
    }
    
    // reset score
    if engine.keyboard_state.pressed(KeyCode::R) {
        game_state.score = 0;
        let score = engine.texts.get_mut("score").unwrap();
        score.value = "Score: 0".to_string();
    }
    
    // show collision
    if engine.keyboard_state.pressed(KeyCode::C) {
        engine.show_colliders = true;
    }
    
    if engine.keyboard_state.pressed(KeyCode::V) {
        engine.show_colliders = false;
    }
    
    // Movements
    let player = engine.sprites.get_mut("player").unwrap();
    const MOVEMENT_SPEED: f32 = 100.0;

    // up
    if engine.keyboard_state.pressed_any(&[KeyCode::W, KeyCode::Up]) {
        player.translation.y += MOVEMENT_SPEED * engine.delta_f32;
    }

    // down
    if engine.keyboard_state.pressed_any(&[KeyCode::S, KeyCode::Down]) {
        player.translation.y -= MOVEMENT_SPEED * engine.delta_f32;
    }

    // left
    if engine.keyboard_state.pressed_any(&[KeyCode::A, KeyCode::Left]) {
        player.translation.x -= MOVEMENT_SPEED * engine.delta_f32;
    }

    // right
    if engine.keyboard_state.pressed_any(&[KeyCode::D, KeyCode::Right]) {
        player.translation.x += MOVEMENT_SPEED * engine.delta_f32;
    }
    
    // timer to respawn
    if game_state.spawn_timer.tick(engine.delta).just_finished() {
        let label = format!("car{}", game_state.car_index);
        game_state.car_index += 1;
        let car1 = engine.add_sprite(label.clone(), SpritePreset::RollingBallRed);
        car1.translation.x = thread_rng().gen_range(-550.0..550.0);
        car1.translation.y = thread_rng().gen_range(-320.0..320.0);
        car1.collision = true;
    }
}