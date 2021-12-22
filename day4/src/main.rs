mod board;

use core::num;

use board::Board;

fn main() -> anyhow::Result<()> {
    let (numbers, boards) = utility::read_input_four(None)?;

    let mut boards: Vec<Board> = boards
        .iter()
        .map(|values| Board::from_raw_values(values))
        .collect::<Result<Vec<Board>, _>>()?;

    'outer: for number in numbers.iter() {
        for board in boards.iter_mut() {
            board.check_value(*number);
            if board.solved {
                println!("{:?}", board.sum * number);
                break 'outer;
            }
        }
    }

    let mut last_solved = 0;
    for number in numbers {
        for board in boards.iter_mut() {
            if !board.solved {
                board.check_value(number);
                if board.solved {
                    last_solved = board.sum * number;
                }
            }
        }
    }
    println!("{:?}", last_solved);

    Ok(())
}
