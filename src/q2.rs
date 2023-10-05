
pub fn run_q2() {
    let x0 = vec![1.0, 2.0, -1.0, 1.0];
    let f0 = fitness_function(&x0);

    let x1 = find_next_neighbour(&x0);
    let f1 = fitness_function(&x1);

    let x2 = find_next_neighbour(&x1);
    let f2 = fitness_function(&x2);

    println!("X0: {:?} Fitness0: {:?}", x0, f0);
    println!("X1: {:?} Fitness1: {:?}", x1, f1);
    println!("X2: {:?} Fitness2: {:?}", x2, f2);
}

pub fn fitness_function(x: &Vec<f64>) -> f64 {
    // get the norm of the vector
    let norm: f64 = x.iter().map(|xj| xj.powi(2)).sum::<f64>().sqrt();
    norm / (x[0].abs().log10() + 1.0) + x[2].abs()
}

fn find_next_neighbour(x: &Vec<f64>) -> Vec<f64> {
    let iter_values = vec![0.1, -0.1, -0.1, 0.1];
    // Add the iter values to the current x
    let mut new_x = x.clone();
    for i in 0..x.len() {
        new_x[i] += iter_values[i];
        // Round the value to 2 decimal places
        new_x[i] = (new_x[i] * 100.0).round() / 100.0;
    }
    new_x
}