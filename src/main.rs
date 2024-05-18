use bevy::{math::bool, prelude::*};
use bevy_ascii_terminal::*;
use rand::Rng;

const WIDTH: usize = 114;
const HEIGHT: usize = 64;

fn setup(mut commands: Commands) {
    let mut terminal = Terminal::new([WIDTH, HEIGHT]).with_border(Border::single_line());

    let mut eng = rand::thread_rng();
    let mut grid: Vec<Vec<bool>> = vec![vec![false; WIDTH]; HEIGHT];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let alive = eng.gen_bool(0.5);
            grid[y][x] = alive;

            if alive {
                terminal.put_char(
                    [x as i32, y as i32].pivot(Pivot::TopLeft),
                    'X'.fg(Color::WHITE),
                );
            }
        }
    }

    println!("{:?}", grid);

    commands.spawn((TerminalBundle::from(terminal), AutoCamera));
}

fn life() {}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, life)
        .run();
}
