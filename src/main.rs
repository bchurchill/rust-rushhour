
// row 0
// row 1
// row 2
// row 3
// row 4
//  col  0  1  2  3  4

#[derive(Clone)]
struct Car {
    horizontal: bool,
    is_target: bool,
    row: u32,
    col: u32,
    size: u32
}

#[derive(Clone)]
struct Board {
    cars: Vec<Car>
}

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
    let puzzle = Board {
        cars: vec![red, teal, bus, violet, truck]
    };
    return puzzle;
}

fn space_for_move(board : &Board, car : &Car, forward: bool) -> u32 {

    if car.horizontal {
        if forward {
            println!("vertical size={}, col={}", car.size, car.col);
            let max_space = 6-car.size-car.col;
            for i in 1..max_space {
                if find_car_at(board, car.row, car.col + car.size + i - 1).is_some() {
                    return i - 1;
                }
            }
            return max_space;
        } else {
            let max_space = car.col;
            for i in 1..max_space {
                if find_car_at(board, car.row, car.col - i).is_some() {
                    return i - 1;
                }
            }
            return max_space;
        }
 
    } else {
        if forward {
            let max_space = 6-car.size-car.row;
            for i in 1..max_space {
                if find_car_at(board, car.row + car.size + i - 1, car.col).is_some() {
                    return i - 1;
                }
            }
            return max_space;
        } else {
            let max_space = car.row;
            for i in 1..max_space {
                if find_car_at(board, car.row - i, car.col).is_some() {
                    return i - 1;
                }
            }
            return max_space;
        }

    }
}

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

        println!("car index {} can move {} to {}", index, start_delta, end_delta);
        for delta in start_delta..end_delta {
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


fn main() {
    let board = sample_board();
    print_board(&board);
    let options = moves_from(&board);
    for option in options {
        print_board(&option);
    }
}
