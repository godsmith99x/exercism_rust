
fn main() {
    let test_case: &[&str] = &[
        "111",
        "1*1",
        "111",
    ];

    let test_case2: &[&str] = &[
        "   ",
        " * ",
        "   ",
    ];

    fn create_2d_vec(input: &[&str]) -> Vec<Vec<char>> {
        let mut workspace: Vec<Vec<_>> = Vec::new();

        for row in input {
            let row_array: Vec<char> = row.chars().collect();
            workspace.push(row_array);
            }

        workspace
    }

    println!("{:#?}", create_2d_vec(test_case))
}
