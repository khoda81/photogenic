use photogenic::initiate_algorithm;

fn main() {
    let num_colors = 64;
    let mut algo = initiate_algorithm(num_colors);
    algo.populate(100);

    loop {
        for _ in 0..10 {
            algo.step();
        }

        let (_, fitness) = algo.fittest().unwrap();
        println!("Gen: {}, Fittest: {fitness}", algo.mutation_rate);
    }
}
