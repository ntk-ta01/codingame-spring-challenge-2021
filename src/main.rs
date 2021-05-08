use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let wait_command = String::from("WAIT");

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let number_of_cells = parse_input!(input_line, i32); // 37

    let mut cells = vec![0; number_of_cells as usize];

    for _ in 0..number_of_cells as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(' ').collect::<Vec<_>>();
        let index = parse_input!(inputs[0], usize); // 0 is the center cell, the next cells spiral outwards
        let richness = parse_input!(inputs[1], i32); // 0 if the cell is unusable, 1-3 for usable cells
        let _neigh_0 = parse_input!(inputs[2], i32); // the index of the neighbouring cell for each direction
        let _neigh_1 = parse_input!(inputs[3], i32);
        let _neigh_2 = parse_input!(inputs[4], i32);
        let _neigh_3 = parse_input!(inputs[5], i32);
        let _neigh_4 = parse_input!(inputs[6], i32);
        let _neigh_5 = parse_input!(inputs[7], i32);

        cells[index] = richness;
    }

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let _day = parse_input!(input_line, i32); // the game lasts 24 days: 0-23
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let _nutrients = parse_input!(input_line, i32); // the base score you gain from the next COMPLETE action
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(' ').collect::<Vec<_>>();
        let _sun = parse_input!(inputs[0], i32); // your sun points
        let _score = parse_input!(inputs[1], i32); // your current score
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(' ').collect::<Vec<_>>();
        let _opp_sun = parse_input!(inputs[0], i32); // opponent's sun points
        let _opp_score = parse_input!(inputs[1], i32); // opponent's score
        let _opp_is_waiting = parse_input!(inputs[2], i32); // whether your opponent is asleep until the next day
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let number_of_trees = parse_input!(input_line, i32); // the current amount of trees
        for _ in 0..number_of_trees as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(' ').collect::<Vec<_>>();
            let _cell_index = parse_input!(inputs[0], i32); // location of this tree
            let _size = parse_input!(inputs[1], i32); // size of this tree: 0-3
            let _is_mine = parse_input!(inputs[2], i32); // 1 if this is your tree
            let _is_dormant = parse_input!(inputs[3], i32); // 1 if this tree is dormant
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let number_of_possible_moves = parse_input!(input_line, i32);
        let mut command = String::from("WAIT");
        let mut get_max_bonus = -1;
        for _ in 0..number_of_possible_moves as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let possible_move = input_line.trim_matches('\n').to_string();
            if possible_move != wait_command && possible_move.starts_with("COMPLETE") {
                let mut iter = possible_move.split_whitespace();
                iter.next();
                let index = if let Some(num) = iter.next() {
                    eprintln!("{}", num);
                    num.parse().unwrap()
                } else {
                    -1
                };
                if index != -1 && get_max_bonus < cells[index as usize] {
                    command = possible_move;
                    get_max_bonus = cells[index as usize];
                }
            }
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        // GROW cellIdx | SEED sourceIdx targetIdx | COMPLETE cellIdx | WAIT <message>

        println!("{}", command);
    }
}
