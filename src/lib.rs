mod photogenic;
mod utils;

use color_space::{FromRgb, ToRgb};
use photogenic::{Gene, World};
use rand::seq::SliceRandom;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct GeneticAlgorithm {
    world: World,
    population: Vec<Gene>,
    generation_idx: usize,
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
        let iter = self.world.generate_population(count);

        self.population.extend(iter)
    }

    pub fn step(&mut self) {
        let population_size = self.population.len();
        let world = &self.world;
        let mut population: Vec<_> = self
            .population
            .drain(..)
            .map(move |gene| {
                let fitness = world.fitness(&gene);
                (gene, fitness)
            })
            .collect();

        // .sort_by(|a, b| self.world.fitness(a).total_cmp(&self.world.fitness(b)));
        population.sort_by(|(_, f_a), (_, f_b)| f_a.total_cmp(f_b).reverse());

        // Take the best half
        population.truncate(population_size * 10 / 100);

        let rng = &mut rand::thread_rng();

        while population.len() + self.population.len() < population_size {
            let (gene, _) = population
                .choose_weighted(rng, |&(_, fitness)| fitness)
                .expect("fitness should all be positive");

            let mut gene = gene.clone();
            world.mutate(&mut gene);

            self.population.push(gene);
        }

        self.population
            .extend(population.into_iter().map(|(gene, _)| gene));

        self.generation_idx += 1;
    }
}

#[wasm_bindgen]
pub fn initiate_algorithm(num_colors: usize) -> GeneticAlgorithm {
    // let initial = color_space::Lab::from_rgb(&color_space::Rgb::from_hex(0xA14A76));
    // let r#final = color_space::Lab::from_rgb(&color_space::Rgb::from_hex(0xFFD046));
    // let colors = (0..num_colors)
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
    //     });

    // let world = World::new(colors);
    let world = World::new([
        color_space::Rgb::from_hex(0xf5b420),
        color_space::Rgb::from_hex(0xf9ab4e),
        color_space::Rgb::from_hex(0xee5c2b),
        color_space::Rgb::from_hex(0xdd2a2b),
        color_space::Rgb::from_hex(0xde4559),
        color_space::Rgb::from_hex(0x912f39),
        color_space::Rgb::from_hex(0x67981d),
        color_space::Rgb::from_hex(0x1b4a1c),
        color_space::Rgb::from_hex(0x347498),
        color_space::Rgb::from_hex(0x212845),
        color_space::Rgb::from_hex(0x1e1a17),
        color_space::Rgb::from_hex(0x171914),
    ]);
    // let world = World::with_color_count(num_colors);

    GeneticAlgorithm::new(world)
}

#[wasm_bindgen]
pub fn render(ctx: &CanvasRenderingContext2d, width: u32, height: u32) -> Result<(), JsValue> {
    // let num_colors = width;
    let num_colors = 20;
    let population_size = 100;
    let generations = 1000;

    let mut algo = initiate_algorithm(num_colors);

    // Generate initial population of random genes
    algo.populate(population_size);

    // Evolution loop
    (0..generations).for_each(|_| algo.step());

    render_best(ctx, &algo, width, height);

    Ok(())
}

#[wasm_bindgen]
pub fn render_best(
    ctx: &CanvasRenderingContext2d,
    algo: &GeneticAlgorithm,
    width: u32,
    height: u32,
) {
    let (best_gene, fitness) = algo.fittest().unwrap();
    let colors: Vec<_> = algo.world.iter_colors(best_gene).collect();
    let gen_idx = algo.generation_idx;

    // Calculate the width of each bar
    let bar_width = 10.0;
    let bar_height = 100.0;

    let total_width = bar_width * colors.len() as f64;
    let x = (width as f64 - total_width) / 2.0;
    let y = (height as f64 - bar_height) / 2.0;

    ctx.clear_rect(0.0, 0.0, width as f64, height as f64);

    // Draw fitness as text
    ctx.set_fill_style(&JsValue::from_str("black")); // Set text color
    ctx.set_font("16px Arial"); // Set font size and type
    ctx.fill_text(&format!("Fitness: {fitness:.2}"), 10.0, 20.0)
        .expect("Failed to draw text");

    ctx.fill_text(&format!("Gen: {gen_idx}"), 10.0, 40.0)
        .expect("Failed to draw text");

    // Loop through each RGB color and draw a vertical bar
    for (index, color) in colors.into_iter().enumerate() {
        let color_space::Rgb { r, g, b } = color;

        // Set the fill style to the current RGB color
        ctx.set_fill_style(&JsValue::from_str(&format!("rgb({r}, {g}, {b})")));

        // Draw the vertical bar
        ctx.fill_rect(x + index as f64 * bar_width, y, bar_width, bar_height);
    }

    // ctx.move_to(0.0, 0.0);
    // ctx.line_to(width as f64, height as f64);
    // ctx.stroke();
}
