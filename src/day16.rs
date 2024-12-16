use std::collections::{BinaryHeap, HashMap, HashSet};

pub fn solve() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input/16.txt")?;

    println!("Day16 Part1: {}", part1(&input)?);
    println!("Day16 Part2: {}", part2(&input)?);

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn cw(self) -> Direction {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }

    fn ccw(self) -> Direction {
        match self {
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
        }
    }

    fn step(self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Up => (x, y - 1),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    x: usize,
    y: usize,
    direction: Direction,
    steps: Vec<(usize, usize)>,
    score: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

#[tracing::instrument(skip_all)]
pub fn part1(input: &str) -> anyhow::Result<String> {
    let grid: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut s = (0, 0);
    let mut e = (0, 0);

    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == 'S' {
                s = (x, y);
            } else if c == 'E' {
                e = (x, y);
            }
        }
    }

    let mut heap = BinaryHeap::new();
    heap.push(State {
        x: s.0,
        y: s.1,
        direction: Direction::Right,
        steps: Vec::new(),
        score: 0,
    });
    let mut visited = HashSet::new();

    while let Some(state) = heap.pop() {
        if state.x == e.0 && state.y == e.1 {
            return Ok(state.score.to_string());
        }

        if !visited.insert((state.x, state.y, state.direction)) {
            continue;
        }

        let (x, y) = state.direction.step(state.x, state.y);
        if grid[y][x] != '#' {
            heap.push(State {
                x,
                y,
                direction: state.direction,
                steps: Vec::new(),
                score: state.score + 1,
            });
        }

        heap.push(State {
            x: state.x,
            y: state.y,
            direction: state.direction.cw(),
            steps: Vec::new(),
            score: state.score + 1000,
        });

        heap.push(State {
            x: state.x,
            y: state.y,
            direction: state.direction.ccw(),
            steps: Vec::new(),
            score: state.score + 1000,
        });
    }

    Ok("".to_string())
}

#[tracing::instrument(skip_all)]
pub fn part2(input: &str) -> anyhow::Result<String> {
    let grid: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut s = (0, 0);
    let mut e = (0, 0);

    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == 'S' {
                s = (x, y);
            } else if c == 'E' {
                e = (x, y);
            }
        }
    }

    let mut heap = BinaryHeap::new();
    heap.push(State {
        x: s.0,
        y: s.1,
        direction: Direction::Right,
        steps: vec![(s.0, s.1)],
        score: 0,
    });
    let mut visited = HashMap::new();
    let mut min_score = None;
    let mut steps = HashSet::new();

    while let Some(state) = heap.pop() {
        if state.x == e.0 && state.y == e.1 {
            if let Some(min_score) = min_score {
                if state.score > min_score {
                    break;
                }
            } else {
                min_score = Some(state.score);
            }

            for step in state.steps {
                steps.insert(step);
            }

            continue;
        }

        if let Some(&score) = visited.get(&(state.x, state.y, state.direction)) {
            if state.score > score {
                continue;
            }
        } else {
            visited.insert((state.x, state.y, state.direction), state.score);
        }

        let (x, y) = state.direction.step(state.x, state.y);
        if grid[y][x] != '#' {
            let mut steps = state.steps.clone();
            steps.push((x, y));

            heap.push(State {
                x,
                y,
                direction: state.direction,
                steps,
                score: state.score + 1,
            });
        }

        heap.push(State {
            x: state.x,
            y: state.y,
            direction: state.direction.cw(),
            steps: state.steps.clone(),
            score: state.score + 1000,
        });

        heap.push(State {
            x: state.x,
            y: state.y,
            direction: state.direction.ccw(),
            steps: state.steps,
            score: state.score + 1000,
        });
    }

    Ok(steps.len().to_string())
}
