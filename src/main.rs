use bevy::prelude::*;
use bevy_ascii_terminal::*;
use rand::Rng;

const WIDTH: usize = 114;
const HEIGHT: usize = 64;

const REFRESH_RATE: f64 = 1.0;

const LIFE_CHAR: char = 'X';
const LIFE_PROBABILTY: f64 = 0.5;

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

fn setup(mut commands: Commands, game_state: ResMut<GameState>) {
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

fn count_alive_neighbours(grid: &Vec<Vec<bool>>, x: usize, y: usize) -> u8 {
    let directions: Vec<(i8, i8)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut count = 0;

    for direction in directions.iter() {
        let nx = x as i8 + direction.0;
        let ny = y as i8 + direction.1;

        if nx < 0 || ny < 0 {
            continue;
        }

        if nx >= WIDTH as i8 || ny >= HEIGHT as i8 {
            continue;
        }

        if grid[ny as usize][nx as usize] {
            count += 1;
        }
    }

    return count;
}

fn generation(mut game_state: ResMut<GameState>) {
    let current_life = game_state.life.clone();

    for (y, line) in current_life.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            let alive_neighbours = count_alive_neighbours(&current_life, x, y);

            game_state.life[y][x] = match (cell, alive_neighbours) {
                (true, 0) => false,
                (true, 1) => false,
                (true, 2) => true,
                (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };
        }
    }
}

fn rerender(mut q_term: Query<&mut Terminal>, game_state: Res<GameState>) {
    let mut terminal = q_term.single_mut();
    terminal.clear();

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
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugin))
        .insert_resource(GameState {
            life: initial_life(),
        })
        .insert_resource(Time::<Fixed>::from_hz(REFRESH_RATE))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (generation, rerender.after(generation)))
        .run();
}
