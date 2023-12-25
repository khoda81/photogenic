use wasm_bindgen::prelude::*;

use color_space::{CompareCie2000, Rgb, ToRgb};
use rand::{seq::SliceRandom, Rng};

pub type Color = Rgb;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct World {
    colors: Vec<Color>,
}

impl World {
    pub fn new(colors: impl IntoIterator<Item = Color>) -> Self {
        Self {
            colors: colors.into_iter().collect(),
        }
    }

    pub fn with_random_colors(count: usize) -> Self {
        let mut rng = rand::thread_rng();

        Self::new(
            (0..count)
                .map(|_| {
                    color_space::Rgb::new(
                        rng.gen_range(0.0..256.0),
                        rng.gen_range(0.0..256.0),
                        rng.gen_range(0.0..256.0),
                    )
                    // color_space::Lab::new(
                    //     rng.gen_range(0.0..256.0),
                    //     rng.gen_range(-256.0..256.0),
                    //     rng.gen_range(-256.0..256.0),
                    // )
                    // .to_rgb()
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
            .map(|(cur, next)| similarity(cur, next))
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
    pub fn log_str(s: &str);
}

impl Gene {
    fn new(count: usize) -> Self {
        Gene {
            indices: (0..count).collect(),
        }
    }

    fn mutate(&mut self, world: &World) {
        self.mutate_by_rotation();
        self.mutate_by_reversing();

        // let rng = &mut rand::thread_rng();
        // let num_rotations = rng.gen_range(1..self.permutation.len());

        // for _ in 0..num_rotations {
        //     let idx1 = rng.gen_range(0..self.permutation.len());
        //     let idx2 = rng.gen_range(idx1..self.permutation.len());

        //     self.permutation.swap(idx1, idx2);
        // }
    }

    fn mutate_by_rotation(&mut self) {
        let slice = self.select_random_slice();

        let rotation_amount = rand::thread_rng().gen_range(1..slice.len());
        slice.rotate_left(rotation_amount)
    }

    fn mutate_by_reversing(&mut self) {
        let slice = self.select_random_slice();
        slice.reverse()
    }

    fn select_random_slice(&mut self) -> &mut [usize] {
        let rng = &mut rand::thread_rng();
        let idx1 = rng.gen_range(0..self.indices.len() - 2);
        let idx2 = rng.gen_range(idx1 + 2..=self.indices.len());

        &mut self.indices[idx1..idx2]
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

fn similarity(color1: Color, color2: Color) -> f64 {
    let distance = color1.compare_cie2000(&color2);

    1.0 / (distance + 0.0001)
}
