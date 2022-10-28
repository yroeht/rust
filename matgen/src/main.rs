use rand::Rng;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} dim1 dim2", args[0]);
        panic!();
    }
    let dim1 = args[1].parse::<i32>().unwrap();
    let dim2 = args[2].parse::<i32>().unwrap();

    let mut rng = rand::thread_rng();
    for _ in 0..dim2 {
        let mut vals: Vec<f32> = Vec::new();
        for _ in 0..dim1 {
            vals.push(rng.gen::<f32>());
        }
        let vals_strv: Vec<String> = vals.iter().map(|x| x.to_string()).collect();
        println!("{}", vals_strv.join(","));
    }
}
