pub fn annotate(minefield: &[&str]) -> Vec<String> {
    //1. Convert input into a 2D vector
    let working_vec = create_2d_vec(minefield);

    //2. Search each cell of the 2D array for "*"s and record their indices
    let mine_locations = locate_mines(working_vec);

    //3. Calculate the neighbor indicies for each mine

}

fn create_2d_vec(input: &[&str]) -> Vec<Vec<char>> {
    let mut temp_vec: Vec<Vec<_>> = Vec::new();

    for row in input {
        let row_array: Vec<char> = row.chars().collect();
        temp_vec.push(row_array);
        }

    temp_vec
}

fn locate_mines(working_vec: Vec<Vec<char>>) -> Vec<(i32, i32)> {

    let mut mine_indicies: Vec<(i32, i32)> = Vec::new();

    for (row, vector) in working_vec.iter().enumerate() {
        for (column, character) in vector.iter().enumerate() {
            if character == &'*' {
                mine_indicies.push((row as i32, column as i32));
            }
        }
    }

    mine_indicies
}
