use bevy::prelude::*;
use bevy_ascii_terminal::*;
use rand::Rng;

const WIDTH: usize = 114;
const HEIGHT: usize = 64;

const REFRESH_RATE: f64 = 1.0;

const LIFE_CHAR: char = 'X';
const LIFE_PROBABILTY: f64 = 0.1;

#[derive(Resource)]
struct GameState {
    life: Vec<Vec<bool>>,
}

fn initial_life() -> Vec<Vec<bool>> {
    let mut eng = rand::thread_rng();
    let mut grid: Vec<Vec<bool>> = vec![vec![false; WIDTH]; HEIGHT];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let alive = eng.gen_bool(LIFE_PROBABILTY);
            grid[y][x] = alive;
        }
    }

    return grid;
}

fn setup(mut commands: Commands, mut game_state: ResMut<GameState>) {
    let mut terminal = Terminal::new([WIDTH, HEIGHT]).with_border(Border::single_line());

    for (y, line) in game_state.life.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            if *cell {
                terminal.put_char(
                    [x as i32, y as i32].pivot(Pivot::TopLeft),
                    LIFE_CHAR.fg(Color::WHITE),
                );
            }
        }
    }

    commands.spawn((TerminalBundle::from(terminal), AutoCamera));
}

fn life(time: Res<Time>) {
    println!("Time: {:?}", time);
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugin))
        .insert_resource(GameState {
            life: initial_life(),
        })
        .insert_resource(Time::<Fixed>::from_hz(REFRESH_RATE))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, life)
        .run();
}
