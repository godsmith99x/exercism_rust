#[allow(unused_variables)]

fn main() {
    let test_case1: &[&str] = &["111", "1*1", "111"];
    let test_case2: &[&str] = &["   ", " * ", "   "];
    let test_case3: &[&str] = &["   ", "   ", "   "];
    let test_case4: &[&str] = &[];
    let test_case5: &[&str] = &[" * ", "  *", "*  ", " * "];

    //1. Convert input into a 2D vector
    fn create_2d_vec(input: &[&str]) -> Vec<Vec<char>> {
        let mut temp_vec: Vec<Vec<_>> = Vec::new();

        for row in input {
            let row_array: Vec<char> = row.chars().collect();
            temp_vec.push(row_array);
        }

        temp_vec
    }

    let working_case1: Vec<Vec<char>> = create_2d_vec(test_case1);
    let working_case2: Vec<Vec<char>> = create_2d_vec(test_case2);
    let working_case3: Vec<Vec<char>> = create_2d_vec(test_case3);
    let working_case4: Vec<Vec<char>> = create_2d_vec(test_case4);
    let working_case5: Vec<Vec<char>> = create_2d_vec(test_case5);

    //2. Search each cell of the 2D array for "*"s and record their indices
    fn locate_mines(working_vec: &Vec<Vec<char>>) -> Vec<(i32, i32)> {
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

    let mine_locations = locate_mines(&working_case5);

    //3. Calculate the neighbor coordinates for each mine

    fn find_row_len(working_vec: &Vec<Vec<char>>) -> i32 {
        let row_len: i32 = working_vec[0].len() as i32;
        row_len
    }

    fn find_col_len(working_vec: &Vec<Vec<char>>) -> i32 {
        let col_len: i32 = working_vec.iter().count() as i32;
        col_len
    }

    let row_len: i32 = find_row_len(&working_case5);
    let col_len: i32 = find_col_len(&working_case5);

    fn add_indicies(point1: &(i32, i32), point2: &(i32, i32)) -> (i32, i32) {
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

    fn calculate_neighbors(
        mine_index: &(i32, i32),
        row_len: &i32,
        col_len: &i32,
    ) -> Vec<(i32, i32)> {
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
            .map(|&p| add_indicies(&p, &mine_index))
            .collect();

        let mut neighbors_valid: Vec<(i32, i32)> = Vec::new();

        for point in neighbors_working {
            if is_valid(&point, &row_len, &col_len) {
                neighbors_valid.push(point.clone())
            };
        }

        neighbors_valid
    }

    println!("{:#?} \n", row_len);
    println!("{:#?} \n", col_len);
    println!("{:#?} \n", calculate_neighbors(&mine_locations[0], &row_len, &col_len));
    // println!("{:#?} \n", working_case2);
    // println!("{:#?} \n", working_case3);
    // println!("{:#?} \n", working_case4);
}
