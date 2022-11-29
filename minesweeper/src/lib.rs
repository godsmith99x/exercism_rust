pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let ans = minefield.iter().map(|&r| r.to_string()).collect::<Vec<_>>();
    ans
}

fn create_2d_vec(input: &[&str]) -> Vec<Vec<char>> {
    let mut temp_vec: Vec<Vec<_>> = Vec::new();

    for row in input {
        let row_array: Vec<char> = row.chars().collect();
        temp_vec.push(row_array);
        }

    temp_vec
}
