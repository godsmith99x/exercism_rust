//Solution Procedure
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    //1. Convert input into a 2D vector
    let input_vec = create_2d_vec(minefield);

    //2. Search each cell of the 2D array for "*"s and record their locations
    let mine_locations = locate_mines(input_vec);

    //3. Calculate the locations of neighboring cells for each mine
    let row_len: i32 = find_row_len(&input_vec);
    let col_len: i32 = find_col_len(&input_vec);
    let mut neighbor_cells: Vec<(i32, i32)> = Vec::new();

    for mine in mine_locations {
        let neighbors = calculate_neighbors(&mine, &row_len, &col_len);
        neighbor_cells.append(&mut neighbors);

    }
}

// Helper Functions
fn create_2d_vec(input: &[&str]) -> Vec<Vec<char>> {
    let mut temp_vec: Vec<Vec<_>> = Vec::new();

    for row in input {
        let row_array: Vec<char> = row.chars().collect();
        temp_vec.push(row_array);
    }

    temp_vec
}

fn locate_mines(working_vec: Vec<Vec<char>>) -> Vec<(i32, i32)> {
    let mut mine_locations: Vec<(i32, i32)> = Vec::new();

    for (row, vector) in working_vec.iter().enumerate() {
        for (column, character) in vector.iter().enumerate() {
            if character == &'*' {
                mine_locations.push((row as i32, column as i32));
            }
        }
    }

    mine_locations
}

fn find_row_len(working_vec: &Vec<Vec<char>>) -> i32 {
    let row_len: i32 = working_vec[0].len() as i32;
    row_len
}

fn find_col_len(working_vec: &Vec<Vec<char>>) -> i32 {
    let col_len: i32 = working_vec.iter().count() as i32;
    col_len
}

fn add_points(point1: &(i32, i32), point2: &(i32, i32)) -> (i32, i32) {
    let row_translation = point1.0 + point2.0;
    let col_translation = point1.1 + point2.1;
    let new_point = (row_translation, col_translation);
    new_point
}

fn is_valid(point: &(i32, i32), row_len: &i32, col_len: &i32) -> bool {
    match point {
        point if point.0 < 0 => false,
        point if point.0 > *row_len => false,
        point if point.1 < 0 => false,
        point if point.1 > *col_len => false,
        _ => true,
    }
}

fn calculate_neighbors(mine_index: &(i32, i32), row_len: &i32, col_len: &i32) -> Vec<(i32, i32)> {
    let neighbor_template: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let neighbors_working: Vec<(i32, i32)> = neighbor_template
        .iter()
        .map(|&p| add_points(&p, &mine_index))
        .collect();

    let mut neighbors_valid: Vec<(i32, i32)> = Vec::new();

    for point in neighbors_working {
        if is_valid(&point, &row_len, &col_len) {
            neighbors_valid.push(point.clone())
        };
    }

    neighbors_valid
}
