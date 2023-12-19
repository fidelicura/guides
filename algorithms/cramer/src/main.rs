fn cramer(matrix: [[i32; 3]; 3], free: [i32; 3]) {
    // initialize empty matrices
    let general_matrix: [[i32; 3]; 3] = matrix;
    let matrix_x: [[i32; 3]; 3] = fill_matrix(general_matrix, free, FillSide::Left);
    let matrix_y: [[i32; 3]; 3] = fill_matrix(general_matrix, free, FillSide::Center);
    let matrix_z: [[i32; 3]; 3] = fill_matrix(general_matrix, free, FillSide::Right);
    println!("# MATRICES #");
    println!("g = {:?}", general_matrix);
    println!("x = {:?}", matrix_x);
    println!("y = {:?}", matrix_y);
    println!("z = {:?}", matrix_z);
    println!("# MATRICES #\n");

    // calculate deltas
    let delta: i32 = calculate_delta(general_matrix);
    let delta_x: i32 = calculate_delta(matrix_x);
    let delta_y: i32 = calculate_delta(matrix_y);
    let delta_z: i32 = calculate_delta(matrix_z);
    println!("# DELTAS #");
    println!("Δg = {}", delta);
    println!("Δx = {}", delta_x);
    println!("Δy = {}", delta_y);
    println!("Δz = {}", delta_z);
    println!("# DELTAS #\n");

    // calculate monomials
    let x: i32 = delta_x / delta;
    let y: i32 = delta_y / delta;
    let z: i32 = delta_z / delta;
    println!("# RESULTS #");
    println!("x = {x}");
    println!("y = {y}");
    println!("z = {z}");
    println!("# RESULTS #");
}

enum FillSide {
    Left,
    Center,
    Right,
}

fn fill_matrix(general: [[i32; 3]; 3], free: [i32; 3], side: FillSide) -> [[i32; 3]; 3] {
    let mut matrix = general;

    // swap free terms with according position by x/y/z as left/center/right
    match side {
        FillSide::Left => {
            for i in 0..3 {
                matrix[i][0] = free[i];
            }
        }
        FillSide::Center => {
            for i in 0..3 {
                matrix[i][1] = free[i];
            }
        }
        FillSide::Right => {
            for i in 0..3 {
                matrix[i][2] = free[i];
            }
        }
    }

    matrix
}

fn calculate_delta(matrix: [[i32; 3]; 3]) -> i32 {
    // calculation patterns
    let positive_pattern: [[[usize; 2]; 3]; 3] = [
        [[0, 0], [1, 1], [2, 2]],
        [[0, 1], [1, 2], [2, 0]],
        [[0, 2], [1, 0], [2, 1]],
    ];
    let negative_pattern: [[[usize; 2]; 3]; 3] = [
        [[2, 0], [1, 1], [0, 2]],
        [[2, 1], [1, 2], [0, 0]],
        [[2, 2], [1, 0], [0, 1]],
    ];

    // calculate by patterns according to operation type
    let positive: i32 = positive_pattern
        .iter()
        .fold(0, |mut outer_acc, pattern_triplet| {
            outer_acc += pattern_triplet.iter().fold(1, |mut inner_acc, &[i, j]| {
                inner_acc *= matrix[i][j];
                inner_acc
            });

            outer_acc
        });
    let result: i32 = negative_pattern
        .iter()
        .fold(positive, |mut outer_acc, pattern_triplet| {
            outer_acc -= pattern_triplet.iter().fold(1, |mut inner_acc, &[i, j]| {
                inner_acc *= matrix[i][j];
                inner_acc
            });

            outer_acc
        });

    // return delta
    result
}

fn main() {
    let matrix: [[i32; 3]; 3] = [[2, 5, 4], [1, 3, 2], [2, 10, 9]];
    let free: [i32; 3] = [30, 150, 110];
    cramer(matrix, free);
}
