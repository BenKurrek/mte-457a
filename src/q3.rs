use crate::q2::fitness_function;

pub fn run_q3() {
    let x0 = vec![
        vec![1.0, 2.0, -1.0, 1.0], // First particle
        vec![-1.0, 1.0, 1.0, -1.0], // Second particle
        vec![2.0, -1.0, 2.0, 0.0], // Third particle
    ];
    let mut local_best = x0.clone();
    let mut global_best = vec![1.0, 2.0, -1.0, 1.0];
    update_global_best(&x0, &mut global_best);
    update_local_best(&x0, &mut local_best);

    let v0 = vec![vec![0.0; 4];3];
    print_output(&x0, &v0, &get_fitness(&x0), &local_best, &global_best);

    let v1 = vec![
        calculate_velocity(&x0[0], &v0[0], &local_best[0], &global_best),
        calculate_velocity(&x0[1], &v0[1], &local_best[1], &global_best),
        calculate_velocity(&x0[2], &v0[2], &local_best[2], &global_best),
    ];
    let x1 = vec![
        x0[0].iter().zip(v1[0].iter()).map(|(x, v)| x + v).collect::<Vec<f64>>(),
        x0[1].iter().zip(v1[1].iter()).map(|(x, v)| x + v).collect::<Vec<f64>>(),
        x0[2].iter().zip(v1[2].iter()).map(|(x, v)| x + v).collect::<Vec<f64>>(),
    ];
    update_global_best(&x1, &mut global_best);
    update_local_best(&x1, &mut local_best);
    print_output(&x1, &v1, &get_fitness(&x1), &local_best, &global_best);

    let v2 = vec![
        calculate_velocity(&x1[0], &v1[0], &local_best[0], &global_best),
        calculate_velocity(&x1[1], &v1[1], &local_best[1], &global_best),
        calculate_velocity(&x1[2], &v1[2], &local_best[2], &global_best),
    ];
    let x2 = vec![
        x1[0].iter().zip(v2[0].iter()).map(|(x, v)| x + v).collect::<Vec<f64>>(),
        x1[1].iter().zip(v2[1].iter()).map(|(x, v)| x + v).collect::<Vec<f64>>(),
        x1[2].iter().zip(v2[2].iter()).map(|(x, v)| x + v).collect::<Vec<f64>>(),
    ];
    update_global_best(&x2, &mut global_best);
    update_local_best(&x2, &mut local_best);
    print_output(&x2, &v2, &get_fitness(&x2), &local_best, &global_best);

}

fn calculate_velocity(cur_x: &Vec<f64>, cur_v: &Vec<f64>, local_best: &Vec<f64>, global_best: &Vec<f64>) -> Vec<f64> {
    let vel_component = cur_v.iter().map(|v| v * 0.1).collect::<Vec<f64>>();
    let local_component = local_best.iter().zip(cur_x.iter()).map(|(l, x)| (l - x) * 0.1).collect::<Vec<f64>>();
    let global_component = global_best.iter().zip(cur_x.iter()).map(|(g, x)| (g - x) * 0.1).collect::<Vec<f64>>();

    vel_component.iter().zip(local_component.iter()).zip(global_component.iter()).map(|((v, l), g)| v + l + g).collect::<Vec<f64>>()
}

fn update_local_best(cur_x: &Vec<Vec<f64>>, local_best: &mut Vec<Vec<f64>>) {
    for i in 0..cur_x.len() {
        if fitness_function(&cur_x[i]) > fitness_function(&local_best[i]) {
            (*local_best)[i] = cur_x[i].clone();
        }
    } 
}

fn update_global_best(cur_x: &Vec<Vec<f64>>, global_best: &mut Vec<f64>) {
    for i in 0..cur_x.len() {
        if fitness_function(&cur_x[i]) > fitness_function(&global_best) {
            *global_best = cur_x[i].clone();
        }
    } 
}

fn get_fitness(cur_x: &Vec<Vec<f64>>) -> Vec<f64>{
    let mut f = vec![];
    for i in 0..cur_x.len() {
        f.push(fitness_function(&cur_x[i]));
    } 

    f
}

fn print_output(cur_x: &Vec<Vec<f64>>, cur_v: &Vec<Vec<f64>>, f: &Vec<f64>, local_best: &Vec<Vec<f64>>, global_best: &Vec<f64>) {
    for i in 0..cur_x.len() {
        println!("X{}: {:#?} V{}: {:#?} L{}: {:#?} F{}: {:?}", i, cur_x[i], i, cur_v[i], i, f[i], i, local_best[i]);
    }

    println!("Global Best: {:#?}", global_best);
}