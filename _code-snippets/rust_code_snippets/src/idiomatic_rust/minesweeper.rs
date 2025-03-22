const DIRECTIONS: [(isize, isize); 8] = [
  (-1, -1),
  (-1, 0),
  (-1, 1),
  (0, -1),
  (0, 1),
  (1, -1),
  (1, 0),
  (1, 1),
];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
  minefield
    .iter()
    .enumerate()
    .map(|(i, &row)| {
      row
        .as_bytes()
        .iter()
        .enumerate()
        .map(|(j, &cell)| {
          if cell == b'*' {
            '*' // return * if cell is a mine
          } else {
            match DIRECTIONS
              .iter()
              .map(|&(dx, dy)| (i as isize + dx, j as isize + dy))
              .filter(|&(x, y)| {
                x >= 0 && x < minefield.len() as isize && y >= 0 && y < row.len() as isize
              }) // filter out of bounds
              .filter(|&(x, y)| minefield[x as usize].as_bytes()[y as usize] == b'*') // count mines
              .count()
            {
              0 => ' ',                              // empty cell
              count => (count as u8 + b'0') as char, // convert count to char
            }
          }
        })
        .collect()
    })
    .collect()
}
pub fn annotate2(minefield: &[&str]) -> Vec<String> {
  let mut result: Vec<String> = vec![];

  for i in 0..minefield.len() {
    let row = minefield[i];
    let mut tmp_row = String::new();
    for j in 0..minefield[i].len() {
      if row.as_bytes()[j] == b'*' {
        // if cell is a mine
        tmp_row.push('*');
        continue;
      }

      let mut count = 0;
      for (dx, dy) in &DIRECTIONS {
        let ni = i as isize + dx;
        let nj = j as isize + dy;
        if ni >= 0 && ni < minefield.len() as isize && nj >= 0 && nj < row.len() as isize {
          if minefield[ni as usize].as_bytes()[nj as usize] == b'*' {
            count += 1;
          }
        }
      }
      if count > 0 {
        tmp_row.push((count as u8 + b'0') as char); // convert count to char
      } else {
        tmp_row.push(' '); // empty cell
      }
    }
    result.push(tmp_row);
  }
  result
}
