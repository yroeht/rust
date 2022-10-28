fn read_matrix(filename: &String) -> (usize, Vec<Vec<i32>>) {
    let matrix1_str = std::fs::read_to_string(filename)
        .expect("Trying to read from matrix csv");
    let mut ret: Vec<Vec<i32>> = Vec::new();

    let mut dim = 0;
    for line in matrix1_str.lines() {
        if line.len() == 0 {
            continue;
        }
        let mut line_vals = Vec::new();
        for val in line.split(",").collect::<Vec<&str>>().iter() {
            line_vals.push(val.parse::<i32>().unwrap());
        }
        if dim != 0 && dim != line_vals.len() {
            panic!("dimensions mismatch");
        }
        dim = line_vals.len();
        ret.push(line_vals);
    }
    (dim, ret)
}

fn dump_matrix(matrix: &Vec<Vec<i32>>) {
    for line in matrix {
        let strs: Vec<String> = line.iter().map(|i| i.to_string()).collect();
        println!("{}", strs.join(","));
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} matrix1.csv matrix2.csv", args[0]);
        panic!();
    }
    let (dim1_1, matrix1) = read_matrix(&args[1]);
    let (dim2_1, matrix2) = read_matrix(&args[2]);
    let dim1_2 = matrix1.len();
    let dim2_2 = matrix2.len();

    println!("Loaded matrices of dimensions ({}, {}), ({}, {}).",
        dim1_1, dim1_2, dim2_1, dim2_2);
    println!("matrix1:");
    dump_matrix(&matrix1);
    println!("matrix2:");
    dump_matrix(&matrix2);
    println!("Let's multiply some matrices!");

    let mut res: Vec<Vec<i32>> = Vec::new(); // dim1_2 * dim2_1
    for i in 0..dim1_2 {
        let mut line: Vec<i32> = Vec::new();
        for j in 0..dim2_1 {
            let mut val = 0i32;
            for x in 0..dim1_1 {
                val += matrix1[i][x] * matrix2[x][j];
            }
            line.push(val);
        }
        res.push(line);
    }
    dump_matrix(&res);
}