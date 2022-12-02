#[allow(unused_variables)]

fn main() {
    let test_case1: &[&str] = &[
        "111",
        "1*1",
        "111",
    ];

    let test_case2: &[&str] = &[
        "   ",
        " * ",
        "   ",
    ];

    let test_case3: &[&str] = &[
        "   ",
        "   ",
        "   ",
    ];

    let test_case4: &[&str] = &[
    ];

    fn create_2d_vec(input: &[&str]) -> Vec<Vec<char>> {
        let mut temp_vec: Vec<Vec<_>> = Vec::new();

        for row in input {
            let row_array: Vec<char> = row.chars().collect();
            temp_vec.push(row_array);
            }

        temp_vec
    }

    let working_case1 = create_2d_vec(test_case1);
    let working_case2 = create_2d_vec(test_case2);
    let working_case3 = create_2d_vec(test_case3);
    let working_case4 = create_2d_vec(test_case4);

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

    let working_case_mines = locate_mines(working_case1);

    fn calculate_neighbors(mine_index: (i32, i32)) -> Vec<(i32, i32)> {

        let mut neighbor_template: Vec<(i32, i32)> = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

        let neighbors_working = neighbor_template.iter().map(|&t| );

    }

    // fn locate_neighbors(mine_indicies: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    //     unimplemented!()
    // }

    println!("{:#?} \n", locate_mines(working_case1));
    // println!("{:#?} \n", working_case2);
    // println!("{:#?} \n", working_case3);
    // println!("{:#?} \n", working_case4);
}
