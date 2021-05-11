use std::fmt;
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

struct Tree {
    cell_index: usize,
    size: usize,
    is_mine: bool,
    is_dormant: bool,
}

type Forest = Vec<Tree>;

fn get_forest(cells: &mut Cells) -> Forest {
    let mut forest = vec![];
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let number_of_trees = parse_input!(input_line, i32); // the current amount of trees
    for tree_index in 0..number_of_trees as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(' ').collect::<Vec<_>>();
        let cell_index = parse_input!(inputs[0], usize); // location of this tree
        let size = parse_input!(inputs[1], usize); // size of this tree: 0-3
        let is_mine = parse_input!(inputs[2], i32) == 1; // 1 if this is your tree
        let is_dormant = parse_input!(inputs[3], i32) == 1; // 1 if this tree is dormant
        cells[cell_index].tree_index = Some(tree_index);
        forest.push(Tree {
            cell_index,
            size,
            is_mine,
            is_dormant,
        });
    }
    forest
}

enum Action {
    Grow(i32),
    Seed(i32, i32),
    Complete(i32),
    Wait,
}

impl From<&String> for Action {
    fn from(s: &String) -> Self {
        let inputs = s.split(' ').collect::<Vec<_>>();
        match inputs[0] {
            "GROW" => Action::Grow(parse_input!(inputs[1], i32)),
            "SEED" => Action::Seed(parse_input!(inputs[1], i32), parse_input!(inputs[2], i32)),
            "COMPLETE" => Action::Complete(parse_input!(inputs[1], i32)),
            "WAIT" => Action::Wait,
            _ => {
                // panic!("Wrong action input");
                Action::Wait
            }
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Action::Grow(i) => format!("GROW {}", i),
                Action::Seed(i, j) => format!("SEED {} {}", i, j),
                Action::Complete(i) => format!("COMPLETE {}", i),
                Action::Wait => String::from("WAIT"),
            }
        )
    }
}

type ActionList = Vec<Action>;

fn get_actionlist() -> ActionList {
    let mut action_list = vec![];
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let number_of_possible_actions = parse_input!(input_line, i32); // all legal actions
    for _ in 0..number_of_possible_actions as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let possible_action = input_line.trim_matches('\n').to_string(); // try printing something from here to start with
        action_list.push(Action::from(&possible_action));
    }
    action_list
}

struct GameContext {
    day: i32,
    nutrients: i32,
    sun: i32,
    score: i32,
    op_sun: i32,
    op_score: i32,
    op_is_waiting: bool,
}

fn get_game_context() -> GameContext {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let day = parse_input!(input_line, i32); // the game lasts 24 days: 0-23
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let nutrients = parse_input!(input_line, i32); // the base score you gain from the next COMPLETE action
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(' ').collect::<Vec<_>>();
    let sun = parse_input!(inputs[0], i32); // your sun points
    let score = parse_input!(inputs[1], i32); // your current score
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(' ').collect::<Vec<_>>();
    let op_sun = parse_input!(inputs[0], i32); // opponent's sun points
    let op_score = parse_input!(inputs[1], i32); // opponent's score
    let op_is_waiting = parse_input!(inputs[2], i32) == 1; // whether your opponent is asleep until the next day

    GameContext {
        day,
        nutrients,
        sun,
        score,
        op_sun,
        op_score,
        op_is_waiting,
    }
}

struct Cell {
    index: usize,
    richness: usize,
    neighbors_ids: Vec<i32>,
    tree_index: Option<usize>,
}

type Cells = Vec<Cell>;

fn get_area() -> Cells {
    let mut area = vec![];
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let number_of_cells = parse_input!(input_line, i32); // 37
    for _ in 0..number_of_cells as usize {
        let mut input_line = String::new();
        let mut neighbors_ids = vec![];
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(' ').collect::<Vec<_>>();
        let index = parse_input!(inputs[0], usize); // 0 is the center cell, the next cells spiral outwards
        let richness = parse_input!(inputs[1], usize); // 0 if the cell is unusable, 1-3 for usable cells

        // let _neigh_0 = parse_input!(inputs[2], i32); // the index of the neighbouring cell for each direction
        // let _neigh_1 = parse_input!(inputs[3], i32);
        // let _neigh_2 = parse_input!(inputs[4], i32);
        // let _neigh_3 = parse_input!(inputs[5], i32);
        // let _neigh_4 = parse_input!(inputs[6], i32);
        // let _neigh_5 = parse_input!(inputs[7], i32);
        for neigh in inputs.iter().skip(2) {
            neighbors_ids.push(parse_input!(neigh, i32));
        }
        area.push(Cell {
            index,
            richness,
            neighbors_ids,
            tree_index: None,
        })
    }
    area
}

fn compute_score(
    cells: &[Cell],
    context: &GameContext,
    forest: &[Tree],
) -> (i32, Vec<i32>, Vec<i32>) {
    // richness 0 は無視できる
    // richness 1, 2, 3にあるサイズ0の木の数
    // サイズ1, 2, 3の木の数
    // スコア
    let mut scoring_factors = vec![0; 7];
    for tree in forest.iter() {
        if tree.is_mine {
            if tree.size == 0 {
                scoring_factors[cells[tree.cell_index].richness - 1] += 1;
            } else {
                scoring_factors[tree.size + 2] += 1
            }
        }
    }
    // seedがたくさんあればsunpoint集めターンも必要？
    let mut scoring_coeffs = if context.day < 2 {
        vec![0, 0, 5, 10, 10, 20, 0] // GROW
    } else if context.day < 7 {
        if context.day % 2 == 0 {
            vec![0, 0, 0, 5, 15, 20, 0] // GROW
        } else {
            vec![1, 20, 500, 1, 0, 0, 0] // SEED
        }
    } else if context.day < 10 {
        vec![1, 1, 1, 10, 50, 20, 0] // grow
    } else if context.day < 18 {
        if context.day % 4 == 0 {
            vec![2, 3, 10, 13, 15, 19, 0] // seed
        } else if context.day % 5 == 0 {
            vec![1, 1, 1, 8, 12, 15, 30] // complete
        } else {
            vec![0, 3, 4, 13, 15, 19, 0] // grow
        }
    } else {
        vec![0, 0, 0, 1, 2, 5, 100000]
    };
    let seed_limit = if context.day < 10 {
        7
    } else if context.day < 20 {
        3
    } else {
        1
    };
    if scoring_factors[..3].iter().sum::<i32>() > seed_limit {
        for coeff in scoring_coeffs[..3].iter_mut() {
            *coeff = 0;
        }
    }
    (
        scoring_factors
            .iter()
            .zip(scoring_coeffs.iter())
            .map(|(e, coeff)| e * coeff)
            .sum(),
        scoring_factors,
        scoring_coeffs,
    )
}
#[derive(Debug, Clone, Copy)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn cube_distance(&self, other: &Cube) -> i32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) / 2
    }
}

fn set_coordinates(center: &Cube) -> Vec<Cube> {
    const DIRECTIONS: [[i32; 3]; 6] = [
        [1, -1, 0],
        [1, 0, -1],
        [0, 1, -1],
        [-1, 1, 0],
        [-1, 0, 1],
        [0, -1, 1],
    ];
    let mut coordinates = vec![*center];
    let mut distance = 1;
    for direction in DIRECTIONS.iter() {
        let x = center.x + direction[0] * distance;
        let y = center.y + direction[1] * distance;
        let z = center.z + direction[2] * distance;
        coordinates.push(Cube { x, y, z });
    }
    for i in 0..6 {
        let x = coordinates[i + 1].x + DIRECTIONS[i][0] * distance;
        let y = coordinates[i + 1].y + DIRECTIONS[i][1] * distance;
        let z = coordinates[i + 1].z + DIRECTIONS[i][2] * distance;
        coordinates.push(Cube { x, y, z });
        let x = coordinates[i + 1].x + DIRECTIONS[(i + 1) % 6][0] * distance;
        let y = coordinates[i + 1].y + DIRECTIONS[(i + 1) % 6][1] * distance;
        let z = coordinates[i + 1].z + DIRECTIONS[(i + 1) % 6][2] * distance;
        coordinates.push(Cube { x, y, z });
    }
    distance = 2;
    for i in 0..6 {
        let x = coordinates[i + 1].x + DIRECTIONS[i][0] * distance;
        let y = coordinates[i + 1].y + DIRECTIONS[i][1] * distance;
        let z = coordinates[i + 1].z + DIRECTIONS[i][2] * distance;
        coordinates.push(Cube { x, y, z });
        let x = coordinates[2 * i + 7].x + DIRECTIONS[(i + 1) % 6][0];
        let y = coordinates[2 * i + 7].y + DIRECTIONS[(i + 1) % 6][1];
        let z = coordinates[2 * i + 7].z + DIRECTIONS[(i + 1) % 6][2];
        coordinates.push(Cube { x, y, z });
        let x = coordinates[i + 1].x + DIRECTIONS[(i + 1) % 6][0] * distance;
        let y = coordinates[i + 1].y + DIRECTIONS[(i + 1) % 6][1] * distance;
        let z = coordinates[i + 1].z + DIRECTIONS[(i + 1) % 6][2] * distance;
        coordinates.push(Cube { x, y, z });
    }
    coordinates
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut cells = get_area();

    let center = Cube { x: 0, y: 0, z: 0 };
    let coordinates = set_coordinates(&center);

    // game loop
    loop {
        let mut command = String::from("WAIT");
        for cell_i in cells.iter_mut() {
            cell_i.tree_index = None;
        }

        let context = get_game_context(); // Get input context
        let forest = get_forest(&mut cells); // Get input forest
        let action_list = get_actionlist(); // List of possible actions
        let (now_score, scoring_f, scoring_coeffs) = compute_score(&cells, &context, &forest);
        let mut max_score = 0;

        // for s in action_list.iter() {
        //     eprintln!("Action : {}", s);
        // }

        if action_list.len() > 1 {
            for cell_i in cells.iter() {
                if let Some(tidx) = cell_i.tree_index {
                    if forest[tidx].is_mine && !forest[tidx].is_dormant {
                        let mut seedables = vec![];
                        let cube_i = coordinates[cell_i.index];
                        for cell_j in cells.iter() {
                            let cube_j = coordinates[cell_j.index];
                            let dist = cube_i.cube_distance(&cube_j) as usize;
                            if 2 <= dist && dist <= forest[tidx].size {
                                seedables.push(cell_j.index as i32);
                            }
                        }
                        if forest[tidx].size > 0
                            && scoring_f[..3].iter().sum::<i32>() <= context.sun
                        {
                            for neigh in seedables.iter() {
                                if *neigh == -1 {
                                    continue;
                                }
                                let neigh = *neigh as usize;
                                if cells[neigh].richness == 0 || cells[neigh].tree_index != None {
                                    continue;
                                }
                                // 打った場合の評価
                                let cube_i = coordinates[cell_i.index];
                                let cube_j = coordinates[neigh];
                                let dist = cube_i.cube_distance(&cube_j);
                                let add_score = (3 - cube_i.cube_distance(&center))
                                    * scoring_coeffs[cells[neigh].richness - 1]
                                    * dist
                                    * dist
                                    * dist
                                    * dist;
                                if max_score < now_score + add_score {
                                    max_score = now_score + add_score;
                                    command = format!("SEED {} {}", cell_i.index, neigh);
                                }
                            }
                        }
                        // 次にGROWを打った場合の得点を計算する
                        // todo:GROWさせたときの影の影響を考える
                        let tree = &forest[tidx];
                        let add_score = match tree.size {
                            0 => {
                                if context.sun < 1 + scoring_f[3] {
                                    -50000
                                } else {
                                    scoring_coeffs[3]
                                }
                            }
                            1 => {
                                if context.sun < 3 + scoring_f[4] {
                                    -50000
                                } else {
                                    scoring_coeffs[4]
                                }
                            }
                            2 => {
                                if context.sun < 7 + scoring_f[5] {
                                    -50000
                                } else {
                                    scoring_coeffs[5]
                                }
                            }
                            _ => -50000,
                        };
                        if max_score < now_score + add_score {
                            max_score = now_score + add_score;
                            command = format!("GROW {}", cell_i.index);
                        }

                        // 次にCOMPLETEを打った場合の得点を計算する
                        let tree_limit = if context.day < 18 {
                            3
                        } else if context.day < 21 {
                            1
                        } else {
                            0
                        };
                        if scoring_f[3..6].iter().sum::<i32>() > tree_limit
                            && tree.size == 3
                            && 4 < context.sun
                        {
                            let add_score =
                                (context.nutrients + cell_i.richness as i32) * scoring_coeffs[6];
                            if max_score < now_score + add_score {
                                max_score = now_score + add_score;
                                command = format!("COMPLETE {}", cell_i.index);
                            }
                        }
                    }
                }
            }
        }
        // GROW cellIdx | SEED sourceIdx targetIdx | COMPLETE cellIdx | WAIT <message>
        println!("{}", command);
    }
}
