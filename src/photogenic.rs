use wasm_bindgen::prelude::*;

use color_space::{CompareCie2000, Rgb};
use rand::{
    distributions::{Distribution, WeightedIndex},
    seq::SliceRandom,
    Rng,
};

pub type Color = Rgb;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct World {
    colors: Vec<Color>,
}

impl World {
    pub fn new(colors: impl Into<Vec<Color>>) -> Self {
        Self {
            colors: colors.into(),
        }
    }

    pub fn with_color_count(count: usize) -> Self {
        let mut rng = rand::thread_rng();

        Self::new(
            (0..count)
                .map(|_| {
                    color_space::Rgb::new(
                        rng.gen_range(0.0..256.0), // r
                        rng.gen_range(0.0..256.0), // g
                        rng.gen_range(0.0..256.0), // b
                    )
                })
                .collect::<Vec<_>>(),
        )
    }

    pub fn generate_population(&self, count: usize) -> impl Iterator<Item = Gene> + '_ {
        (0..count).map(move |_| {
            let mut gene = Gene::new(self.colors.len());

            gene.indices.shuffle(&mut rand::thread_rng());
            gene
        })
    }

    pub fn iter_colors<'b: 'a, 'a>(&'a self, gene: &'b Gene) -> impl Iterator<Item = Color> + '_ {
        gene.indices.iter().map(move |&idx| self.colors[idx])
    }

    pub fn fitness(&self, gene: &Gene) -> f64 {
        self.iter_colors(gene)
            .zip(self.iter_colors(gene).skip(1))
            .map(|(curr, next)| similarity(curr, next))
            .sum()
    }

    pub fn mutate(&self, gene: &mut Gene) {
        gene.mutate(self)
    }
}

#[derive(Clone, Debug)]
pub struct Gene {
    indices: Vec<usize>,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_weights(s: &[f64]);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_str(s: &str);
}

impl Gene {
    fn new(count: usize) -> Self {
        Gene {
            indices: (0..count).collect(),
        }
    }

    fn mutate(&mut self, world: &World) {
        self.mutate_by_rotation(world);

        // let rng = &mut rand::thread_rng();
        // let num_rotations = rng.gen_range(1..self.permutation.len());

        // for _ in 0..num_rotations {
        //     let idx1 = rng.gen_range(0..self.permutation.len());
        //     let idx2 = rng.gen_range(idx1..self.permutation.len());

        //     self.permutation.swap(idx1, idx2);
        // }
    }

    fn mutate_by_rotation(&mut self, world: &World) {
        let rng = &mut rand::thread_rng();
        let mut idx1 = rng.gen_range(0..self.indices.len());
        let color1 = world.colors[self.indices[idx1]];

        let mut weights: Vec<_> = (0..self.indices.len())
            .map(|idx| similarity(color1, world.colors[self.indices[idx]]))
            .collect();

        weights[idx1] = 0.0;

        let weighted = match WeightedIndex::new(weights) {
            Ok(weighted) => weighted,
            Err(_) => {
                log_str("found non negative similarity");
                return;
            }
        };

        let idx2 = weighted.sample(rng);
        let step = if idx1 <= idx2 { 1 } else { -1_isize as usize };
        while idx1 != idx2 {
            let next_idx = idx1.wrapping_add(step);
            self.indices.swap(idx1, next_idx);

            idx1 = next_idx;
        }
        // if idx1 <= idx2 {
        //     for idx in idx1..idx2 - 1 {
        //         self.indices.swap(idx, idx + 1);
        //     }
        // } else {
        //     for idx in (idx2 + 1..idx1).rev() {
        //         self.indices.swap(idx, idx - 1);
        //     }
        // }
    }

    // pub fn crossover(&self, other: &Gene<'a>) -> Gene<'a> {
    //     let mut rng = rand::thread_rng();
    //     let mut child_permutation = vec![0; self.permutation.len()];

    //     // Determine the segment to copy from the first parent
    //     let start_point = rng.gen_range(0..self.permutation.len());
    //     let end_point = rng.gen_range(start_point + 1..=self.permutation.len());

    //     // Copy the segment from the first parent to the child
    //     child_permutation[start_point..end_point].clone_from_slice(&self.permutation[start_point..end_point]);

    //     // Fill in the remaining positions with genes from the second parent
    //     let mut remaining_positions: Vec<usize> = (0..self.permutation.len()).filter(|&i| i < start_point || i >= end_point).collect();
    //     let mut remaining_genes = other.permutation.iter().filter(|&gene| !child_permutation.contains(gene)).cycle();

    //     for i in 0..self.permutation.len() {
    //         if i < start_point || i >= end_point {
    //             if let Some(gene) = remaining_positions.pop().map(|p| remaining_genes.nth(p).unwrap()) {
    //                 child_permutation[i] = *gene;
    //             }
    //         }
    //     }

    //     Gene {
    //         colors: self.colors,
    //         permutation: child_permutation,
    //     }
    // }
}

fn similarity(color1: Rgb, color2: Rgb) -> f64 {
    let distance = color1.compare_cie2000(&color2);

    150.0 - distance
}
