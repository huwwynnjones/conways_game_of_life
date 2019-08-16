use ggez;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event;
use ggez::graphics::{clear, draw, present, Color, DrawMode, MeshBuilder, Rect};
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use rand;
use std::time::Duration;
use std::{fmt, fmt::Write};

#[derive(Clone, Debug, PartialEq)]
enum State {
    Alive,
    Dead,
}

#[derive(Clone, Debug, PartialEq)]
struct Grid {
    cells: Vec<Vec<State>>,
}

impl Grid {
    fn seed(size: usize, living_cells: Vec<(usize, usize)>) -> Grid {
        let row = vec![State::Dead; size];
        let mut cells = vec![row; size];

        for position in living_cells {
            cells[position.0][position.1] = State::Alive;
        }

        Grid { cells }
    }

    fn random_grid(size: usize) -> Grid {
        let mut cells = Vec::new();

        for _ in 0..size {
            let mut row = Vec::new();
            for _ in 0..size {
                if rand::random() {
                    row.push(State::Alive)
                } else {
                    row.push(State::Dead)
                }
            }
            cells.push(row)
        }

        Grid { cells }
    }

    fn next_generation(&self) -> Grid {
        let mut new_cells = Vec::new();

        for (row_idx, row) in self.cells.iter().enumerate() {
            let mut new_row = Vec::new();
            for (col_idx, state) in row.iter().enumerate() {
                new_row.push(state_based_on_neighbours(
                    (row_idx, col_idx),
                    &self.cells,
                    state,
                ))
            }
            new_cells.push(new_row)
        }

        Grid { cells: new_cells }
    }
}

fn state_based_on_neighbours(
    current_position: (usize, usize),
    cells: &[Vec<State>],
    current_state: &State,
) -> State {
    let neighbours_directions = [
        Direction::N,
        Direction::NE,
        Direction::E,
        Direction::SE,
        Direction::S,
        Direction::SW,
        Direction::W,
        Direction::NW,
    ];

    let nmb_alive_neighbours = neighbours_directions
        .iter()
        .map(|neighbours_direction| {
            neighbours_state(current_position, cells, neighbours_direction.translation())
        })
        .filter(|state| *state == State::Alive)
        .count();

    match current_state {
        State::Alive => match nmb_alive_neighbours {
            2 | 3 => State::Alive,
            _ => State::Dead,
        },
        State::Dead => match nmb_alive_neighbours {
            3 => State::Alive,
            _ => State::Dead,
        },
    }
}

fn neighbours_state(
    current_position: (usize, usize),
    cells: &[Vec<State>],
    translation: (i32, i32),
) -> State {
    let new_position = (
        current_position.0 as i32 + translation.0,
        current_position.1 as i32 + translation.1,
    );
    let size = cells.len() as i32;
    if (new_position.0 < 0)
        | (new_position.1 < 0)
        | (new_position.0 == size)
        | (new_position.1 == size)
    {
        State::Dead
    } else {
        cells[new_position.0 as usize][new_position.1 as usize].clone()
    }
}

enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    fn translation(&self) -> (i32, i32) {
        match self {
            Direction::N => (-1, 0),
            Direction::NE => (-1, 1),
            Direction::E => (0, 1),
            Direction::SE => (1, 1),
            Direction::S => (1, 0),
            Direction::SW => (1, -1),
            Direction::W => (0, -1),
            Direction::NW => (-1, -1),
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = String::new();
        for row in &self.cells {
            for cell in row {
                write!(text, "{:?} ", cell)?
            }
            text.push_str("\n")
        }
        write!(f, "{}", text)
    }
}

struct MainState {
    grid: Grid,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let seeded_grid = Grid::random_grid(50);
        let s = MainState { grid: seeded_grid };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let next_gen = self.grid.next_generation();
        self.grid = next_gen;
        ggez::timer::sleep(Duration::from_millis(500));
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let width = 10.0;
        let height = 10.0;

        let mut x = 0.0;
        let mut y = 0.0;

        let grey = Color::from_rgb(77,77,77);
        let blue = Color::from_rgb(51, 153, 255);

        let mut grid_builder = MeshBuilder::new();

        for row in self.grid.cells.iter() {
            for cell in row {
                let colour = match cell {
                    State::Alive => blue,
                    State::Dead => grey,
                };
                grid_builder.rectangle(DrawMode::fill(), Rect::new(x, y, width, height), colour);
                x += width;
            }
            x = 0.0;
            y += height;
        }

        let grid = grid_builder.build(ctx)?;

        draw(ctx, &grid, (na::Point2::new(10.0, 10.0),))?;

        present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("conways game of life", "huw")
        .window_setup(WindowSetup::default().title("Conway's Game of Life"))
        .window_mode(WindowMode::default().dimensions(520.0, 520.0));
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blinker_test() {
        let blinker_start = Grid {
            cells: vec![
                vec![State::Dead, State::Alive, State::Dead],
                vec![State::Dead, State::Alive, State::Dead],
                vec![State::Dead, State::Alive, State::Dead],
            ],
        };

        let blinker_end = Grid {
            cells: vec![
                vec![State::Dead, State::Dead, State::Dead],
                vec![State::Alive, State::Alive, State::Alive],
                vec![State::Dead, State::Dead, State::Dead],
            ],
        };

        assert_eq!(blinker_start.next_generation(), blinker_end);
    }
}
