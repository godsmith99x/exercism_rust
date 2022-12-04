#[allow(unused_variables)]

fn main() {
    let test_case1: &[&str] = &["111", "1*1", "111"];
    let test_case2: &[&str] = &["   ", " * ", "   "];
    let test_case3: &[&str] = &["   ", "   ", "   "];
    let test_case4: &[&str] = &[];
    let test_case5: &[&str] = &[" * ", "  *", "*  ", " * "];

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

    fn find_row_len(working_vec: &Vec<Vec<char>>) -> i32 {
        let row_len: i32 = working_vec[0].len() as i32;
        row_len
    }

    fn find_col_len(working_vec: &Vec<Vec<char>>) -> i32 {
        let col_len: i32 = working_vec.iter().count() as i32;
        col_len
    }

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

    let row_len: i32 = find_row_len(&working_case5);
    let col_len: i32 = find_col_len(&working_case5);
    let mine_points = locate_mines(&working_case5);

    fn add_indicies(point1: &(i32, i32), point2: &(i32, i32)) -> (i32, i32) {
        let row_translation = point1.0 + point2.0;
        let col_translation = point1.1 + point2.1;
        let new_point = (row_translation, col_translation);
        new_point
    }

    fn calculate_neighbors(mine_index: (i32, i32)) -> Vec<(i32, i32)> {
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

        let neighbors_working = neighbor_template
            .iter()
            .map(|&t| add_indicies(&t, &mine_index))
            .collect();

        neighbors_working
    }

    fn locate_neighbors(mine_indicies: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
        let mut neighbors_vec: Vec<(i32, i32)> = Vec::new();

        for mine_index in mine_indicies {
            let mut neighbors = calculate_neighbors(mine_index);
            neighbors_vec.append(&mut neighbors);
        }

        neighbors_vec
    }

    println!("{:#?} \n", row_len);
    println!("{:#?} \n", col_len);
    println!("{:#?} \n", locate_neighbors(mine_points));
    // println!("{:#?} \n", working_case2);
    // println!("{:#?} \n", working_case3);
    // println!("{:#?} \n", working_case4);
}
