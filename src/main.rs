
use std::collections::HashMap;

// row 0
// row 1
// row 2
// row 3
// row 4
// row 5
// col   0  1  2  3  4 5

const DEBUG : bool = false;

macro_rules! debug {

    ($($arg:tt)*) => {
        if DEBUG {
            println!($($arg)*);
        }
    }
}

// Data structure for each car
#[derive(Clone, Hash, Eq, PartialEq)]
struct Car {
    horizontal: bool,
    is_target: bool,
    row: u32,
    col: u32,
    size: u32
}

// Data structure to represent the game state
#[derive(Clone, Hash, Eq, PartialEq)]
struct Board {
    cars: Vec<Car>
}

// Create the sample board, provided as level 1 demo online
fn sample_board() -> Board {
    let red = Car {
        horizontal: true,
        is_target: true,
        row: 2,
        col: 0,
        size: 2
    };
    let teal = Car {
        horizontal: false,
        is_target: false,
        row: 1,
        col: 2,
        size: 2
    };
    let bus = Car {
        horizontal: false,
        is_target: false,
        row: 1,
        col: 4,
        size: 3
    };
    let violet = Car {
        horizontal: false,
        is_target: false,
        row: 2,
        col: 5,
        size: 2
    };
    let truck = Car {
        horizontal: true,
        is_target: false,
        row: 3,
        col: 0,
        size: 3
    };
    let car1 = Car {
        horizontal: false,
        is_target: false,
        row: 4,
        col: 0,
        size: 2 
    };
    let car2 = Car {
        horizontal: false,
        is_target: false,
        row: 4,
        col: 1,
        size: 2 
    };
    let car3 = Car {
        horizontal: false,
        is_target: false,
        row: 4,
        col: 2,
        size: 2 
    };
    let car4 = Car {
        horizontal: true,
        is_target: false,
        row: 4,
        col: 4,
        size: 2 
    };
    let car5 = Car {
        horizontal: true,
        is_target: false,
        row: 5,
        col: 4,
        size: 2 
    };


    let puzzle = Board {
        cars: vec![red, teal, bus, violet, truck, car1, car2, car3, car4, car5]
    };
    return puzzle;
}

// Check if the target car is in the right place to win the game
fn is_winning(board : &Board) -> bool {

    for car in board.cars.iter() {
        if !car.is_target {
            continue;
        }

        return car.size + car.col == 6;
    }

    panic!("could not find target car!");
}

// For a given car on the board, see how many spaces it can move
// (either forward or backward)
fn space_for_move(board : &Board, car : &Car, forward: bool) -> u32 {

    if car.horizontal {
        if forward {
            let max_space = 6-car.size-car.col;
            for i in 1..=max_space {
                if find_car_at(board, car.row, car.col + car.size + i - 1).is_some() {
                    return i - 1;
                }
            }
            return max_space;
        } else {
            let max_space = car.col;
            for i in 1..=max_space {
                if find_car_at(board, car.row, car.col - i).is_some() {
                    return i - 1;
                }
            }
            return max_space;
        }
 
    } else {
        if forward {
            let max_space = 6-car.size-car.row;
            for i in 1..=max_space {
                if find_car_at(board, car.row + car.size + i - 1, car.col).is_some() {
                    return i - 1;
                }
            }
            return max_space;
        } else {
            let max_space = car.row;
            for i in 1..=max_space {
                if find_car_at(board, car.row - i, car.col).is_some() {
                    return i - 1;
                }
            }
            return max_space;
        }

    }
}

// For a given board position, find all possible subsequent states
fn moves_from(board : &Board) -> Vec<Board> {

    let mut output = Vec::new();

    // iterate through cars
        // for each car, see if we can move it left/right/up/down
        // and by how much
    for (index,car) in board.cars.iter().enumerate() {
        let forward_space = space_for_move(board, &car, true);
        let backward_space = space_for_move(board, &car, false);
        let start_delta = -(backward_space as i32);
        let end_delta = forward_space as i32;

        for delta in start_delta..=end_delta {
            // moving a car 0 spaces doesn't count as a move
            if delta == 0 {
                continue;
            }

            // create a new board
            let mut new_board = (*board).clone();
            let new_car = &mut new_board.cars[index];

            // move the car horizontally or vertically
            if car.horizontal {
                new_car.col = ((new_car.col as i32) + delta) as u32;
            } else {
                new_car.row = ((new_car.row as i32) + delta) as u32;
            }

            // add new board to the outputs
            output.push(new_board);
        }
    }

    return output;
}

// Check which car is at a certain position on the board
fn find_car_at(board : &Board, row: u32, col: u32) -> Option<u32> {
    
    for (index, car) in board.cars.iter().enumerate() {
        if car.horizontal {
            if car.row != row {
                continue;
            }
            if col < car.col {
                continue;
            }         
            if col > car.col + car.size - 1 {
                continue;
            }
            return Some(index as u32);
        } else {
            if car.col != col {
                continue;
            }
            if row < car.row {
                continue;
            }
            if row > car.row + car.size - 1 {
                continue;
            }
            return Some(index as u32);
        }
    }

    return None;
}

// Print the board to the console
fn print_board(board : &Board) {
    
    println!("--------------------");
    for row in 0..=5 {
        print!("|");
        for col in 0..=5 {
            let car_index = find_car_at(board, row, col);
            match car_index {
                None => print!("   "),
                Some(i) => print!(" {} ", i)
            }
        }
        println!("|");
    }
    println!("--------------------");
}

// Used for debugging the mapping during the breadth first
// search that shows the shortest path to a given board position.
fn print_path_map(mapping : &HashMap<Board, Option<Board>>) {
    println!("Showing mapping");
    for (k,v) in mapping {
        println!("------------------------------");
        print_board(&k);
        println!("---->");
        match v {
            None => println!("None"),
            Some(b) => print_board(&b)
        }
        println!("------------------------------");
    }
}

// Convert the shortest path mapping into a vector that shows the steps in the solution
fn build_solution_vector(solution : &Board, mapping : HashMap<Board, Option<Board>>) -> Vec<Board> {

    let mut output = Vec::new();
    let mut current = solution;
    output.push(current.clone());

    if DEBUG {
        print_path_map(&mapping);
    }

    while mapping.contains_key(current) {
        match mapping.get(current) {
            None => panic!("can't find shortest path for solution"),
            Some(None) => { 
                break; 
            }
            Some(Some(b)) => {
                current = b;
                output.push(current.clone());
            }
        }
    }

    return output;
}


// Perform a breadth first search to find the shortest solution to the puzzle
fn find_solution(board : &Board) -> Option<Vec<Board>> {

    // queue of boards to explore
    let mut queue = Vec::new();
    queue.push(board.clone());

    // queue of boards to explore on next iteration
    let mut next_queue = Vec::new();

    // keep track of what's been path_map in the past
    // and the shortest path back to the root
    let mut path_map = HashMap::new();
    path_map.insert(board.clone(), None);

    let mut round = 0;

    while !queue.is_empty() {

        debug!("---> starting from here!");
        let current = queue.remove(0);
        if DEBUG {
            print_board(&current);
        }

        // Add subsequent boards to the queue
        let moves = moves_from(&current);

        for new_board in moves {

            // Check if we've been here before
            if path_map.contains_key(&new_board) {
                continue;
            }

            if DEBUG {
                // print new boards we find
                print_board(&new_board);
            }

            // Add the board to the path mapping
            path_map.insert(new_board.clone(), Some(current.clone()));

            // Check if this board is winning
            if is_winning(&new_board) {
                return Some(build_solution_vector(&new_board, path_map));
            }

            // Save for next iteration
            next_queue.push(new_board);
        }

        if queue.is_empty() {
            round += 1;
            debug!("========  Round {}  =========", round);
            queue = next_queue;
            next_queue = Vec::new();
        }
    }

    return None;
}


// Solve the sample board and print the solution.
fn main() {
    let board = sample_board();
    print_board(&board);

    let solution = find_solution(&board);
    match solution {
        None => println!("No solution found!"),
        Some(mut boards) => {
            boards.reverse();
            debug!("Got a solution!");
            for board in boards {
                print_board(&board);
            }
        }
    }
}
