fn main() {
    println!("Hello, world!");
}

fn should_cell_live(board: &Vec<Vec<CellState>>, row: usize, col: usize) -> bool {
    false
}

fn should_cell_be_born(board: &Vec<Vec<CellState>>, row: usize, col: usize) -> bool {
    false
}

#[derive(Debug)]
enum CellState{
    Alive,
    Dead
}

impl CellState {
    /// Returns `true` if the cell state is [`Alive`].
    ///
    /// [`Alive`]: CellState::Alive
    #[must_use]
    fn is_alive(&self) -> bool {
        matches!(self, Self::Alive)
    }

    /// Returns `true` if the cell state is [`Dead`].
    ///
    /// [`Dead`]: CellState::Dead
    #[must_use]
    fn is_dead(&self) -> bool {
        matches!(self, Self::Dead)
    }
}

fn calc_next_board_state(board: &Vec<Vec<CellState>>) -> Vec<Vec<CellState>> {
    let mut output = Vec::<Vec<CellState>>::new();
    
    for row in 0..board.len() {
        let mut new_row = Vec::<CellState>::new();

        for col in 0..board[row].len() {
            let cell =  board[row][col];
            match cell {
                CellState:: Alive =>{
                    if should_cell_live(&board, row, col) {
                    new_row.push(CellState::Alive);
                } else {
                    new_row.push(CellState::Dead);
                }
                },
                CellState::Dead =>{
                    if should_cell_be_born(&board, row, col) {
                    new_row.push(CellState::Alive);
                } else {
                    new_row.push(CellState::Dead);
                }
                }
            }
 
            new_row.push(board[row][col]);
        }
        output.push(new_row);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::CellState::*;

    #[test]
    fn test_Alive() {
        let initial_state = vec![vec![Dead, Alive, Dead], vec![Dead, Dead, Alive], vec![Alive, Alive, Alive], vec![Dead, Dead, Dead]];

        let final_state = vec![vec![Dead, Dead, Dead], vec![Alive, Dead, Alive], vec![Dead, Alive, Alive], vec![Dead, Alive, Dead]];

        let next_state = calc_next_board_state(&initial_state);

        assert_eq!(next_state, final_state);
    }
}
