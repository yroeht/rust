use rand::Rng;

pub fn gen(dim1 :i32, dim2 :i32) {
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
