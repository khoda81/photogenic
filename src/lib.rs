mod photogenic;
mod utils;

use itertools::Itertools;
use photogenic::{Gene, World};
use rand::{
    distributions::{Distribution, WeightedIndex},
    Rng,
};
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct GeneticAlgorithm {
    world: World,
    population: Vec<Gene>,
    generation_idx: usize,
    pub mutation_rate: f64,
}

impl GeneticAlgorithm {
    pub fn new(world: World) -> Self {
        Self::with_population(world, vec![])
    }

    fn with_population(world: World, population: Vec<Gene>) -> GeneticAlgorithm {
        Self {
            world,
            population,
            generation_idx: 0,
            mutation_rate: 0.1,
        }
    }

    pub fn fittest(&self) -> Option<(&Gene, f64)> {
        self.population
            .iter()
            .map(|gene| (gene, self.world.fitness(gene)))
            .max_by(|a, b| a.1.total_cmp(&b.1))
    }
}

#[wasm_bindgen]
impl GeneticAlgorithm {
    pub fn populate(&mut self, count: usize) {
        let new_population = self.world.generate_population(count);
        self.population.extend(new_population)
    }

    pub fn set_population_size(&mut self, new_size: usize) {
        if let Some(best) = self.population.first().cloned() {
            self.population.resize(new_size, best);
        } else {
            self.populate(new_size)
        }
    }

    pub fn step(&mut self) {
        let population_size = self.population.len();

        let population: Vec<_> = std::mem::take(&mut self.population)
            .into_iter()
            .map(|gene| {
                let fitness = self.world.fitness(&gene);
                (gene, fitness)
            })
            .collect();

        let (&(_, min_fitness), (best, _)) = population
            .iter()
            .minmax_by(|(_, f_a), (_, f_b)| f64::total_cmp(f_a, f_b))
            .into_option()
            .expect("the world should be populated before calling step");

        // Keep the best alive
        self.population.push(best.clone());

        let weights = population
            .iter()
            .map(|&(_, fitness)| fitness - min_fitness + 0.0001);

        let sampler = WeightedIndex::new(weights).expect("fitness should all be positive");

        let rng = &mut rand::thread_rng();
        while self.population.len() < population_size {
            // Select two random parents based on the fitness score
            let (parent_1, _) = &population[sampler.sample(rng)];
            let (parent_2, _) = &population[sampler.sample(rng)];

            let mut gene = Gene::crossover(parent_1, parent_2);

            // Mutate it with probability=mutation_rate
            if rng.gen_bool(self.mutation_rate) {
                self.world.mutate(&mut gene);
            }

            self.population.push(gene);
        }

        self.generation_idx += 1;
    }
}

#[wasm_bindgen]
pub fn initiate_algorithm(num_colors: usize) -> GeneticAlgorithm {
    // use color_space::{FromRgb, ToRgb};
    // let initial = color_space::Lab::from_rgb(&color_space::Rgb::from_hex(0));
    // let r#final = color_space::Lab::from_rgb(&color_space::Rgb::from_hex(0xFFFFFF));
    // let colors: Vec<_> = (0..num_colors)
    //     .map(|idx| idx as f64 / num_colors as f64)
    //     .map(|position| {
    //         let color_space::Lab {
    //             l: l0,
    //             a: a0,
    //             b: b0,
    //         } = initial;

    //         let color_space::Lab { l, a, b } = r#final;

    //         color_space::Lab {
    //             l: l * position + l0 * (1.0 - position),
    //             a: a * position + a0 * (1.0 - position),
    //             b: b * position + b0 * (1.0 - position),
    //         }
    //         .to_rgb()
    //     })
    //     .collect();

    // let world = World::new(colors);
    // let world = World::new([
    //     color_space::Rgb::from_hex(0xf5b420),
    //     color_space::Rgb::from_hex(0xf9ab4e),
    //     color_space::Rgb::from_hex(0xee5c2b),
    //     color_space::Rgb::from_hex(0xdd2a2b),
    //     color_space::Rgb::from_hex(0xde4559),
    //     color_space::Rgb::from_hex(0x912f39),
    //     color_space::Rgb::from_hex(0x67981d),
    //     color_space::Rgb::from_hex(0x1b4a1c),
    //     color_space::Rgb::from_hex(0x347498),
    //     color_space::Rgb::from_hex(0x212845),
    //     color_space::Rgb::from_hex(0x1e1a17),
    //     color_space::Rgb::from_hex(0x171914),
    // ]);
    let world = World::with_random_colors(num_colors);

    GeneticAlgorithm::new(world)
}

#[wasm_bindgen]
pub fn render_best(
    ctx: &CanvasRenderingContext2d,
    algo: &GeneticAlgorithm,
    width: f64,
    height: f64,
) {
    let (best_gene, fitness) = algo.fittest().unwrap();
    let colors: Vec<_> = algo.world.iter_colors(best_gene).collect();
    let gen_idx = algo.generation_idx;

    let num_colors = colors.len() as f64;

    // Calculate the width of each bar
    let bar_width = (width / num_colors).min(10.0);
    let bar_height = 200.0;

    let total_width = bar_width * num_colors;
    let x = (width - total_width) / 2.0;
    let y = (height - bar_height) / 2.0;

    ctx.clear_rect(0.0, 0.0, width, height);

    ctx.set_fill_style(&JsValue::from_str("black")); // Set text color
    ctx.set_font("16px Arial"); // Set font size and type
    ctx.fill_text(&format!("Fitness: {fitness:.2}"), 10.0, 20.0)
        .expect("Failed to draw text");

    ctx.fill_text(&format!("Gen: {gen_idx}"), 10.0, 40.0)
        .expect("Failed to draw text");

    // ctx.fill_text(&format!("{:.2?}", best_gene.probs()), 10.0, 60.0)
    //     .expect("Failed to draw text");

    // Loop through each RGB color and draw a vertical bar
    for (index, color) in colors.into_iter().enumerate() {
        let color_space::Rgb { r, g, b } = color;

        // Set the fill style to the current RGB color
        ctx.set_fill_style(&JsValue::from_str(&format!("rgb({r}, {g}, {b})")));

        // Draw the vertical bar
        ctx.fill_rect(x + index as f64 * bar_width, y, bar_width, bar_height);
    }
}
