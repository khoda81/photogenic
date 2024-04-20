use std::{borrow::BorrowMut, collections::BTreeSet, iter::FromIterator};

use color_space::{CompareCie2000, Rgb};
use rand::{seq::SliceRandom, Rng};
use thiserror::Error;

pub type Color = Rgb;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct World {
    colors: Vec<Color>,
}

impl World {
    pub fn new(colors: impl IntoIterator<Item = impl Into<Color>>) -> Self {
        Self {
            colors: colors.into_iter().map(Into::into).collect(),
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
                })
                .collect::<Vec<_>>(),
        )
    }

    pub fn generate_population(&self) -> impl Iterator<Item = Gene> + '_ {
        std::iter::repeat_with(move || {
            let mut gene = Gene::new(self.colors.len());
            gene.indices.shuffle(&mut rand::thread_rng());
            gene
        })
    }

    pub fn iter_colors<'b: 'a, 'a>(&'a self, gene: &'b Gene) -> impl Iterator<Item = Color> + '_ {
        gene.indices.iter().map(move |&idx| self.colors[idx])
    }

    pub fn fitness(&self, gene: &Gene) -> f64 {
        self.color_pairs(gene)
            .map(|(cur, next)| similarity(cur, next))
            .sum()
    }

    fn color_pairs<'a: 'c, 'b: 'c, 'c>(
        &'a self,
        gene: &'b Gene,
    ) -> impl Iterator<Item = (Color, Color)> + 'c {
        self.iter_colors(gene).zip(self.iter_colors(gene).skip(1))
    }

    pub fn mutate(&self, gene: &mut Gene) {
        gene.mutate()
    }
}

// TODO: store the log prob
#[derive(Copy, Clone, Debug, Default, PartialEq)]
struct Bernoulli(f64);

#[derive(Copy, Clone, Debug, Default, PartialEq, Error)]
#[error("{0} is not a valid probability (it should be in range [0, 1])")]
struct InvalidProbabilityError(f64);

impl Bernoulli {
    pub fn new(prob: f64) -> Result<Self, InvalidProbabilityError> {
        if (0.0..=1.0).contains(&prob) {
            Ok(Bernoulli(prob))
        } else {
            Err(InvalidProbabilityError(prob))
        }
    }

    pub fn mutate(&mut self) {
        // You can adjust the mutation range as needed
        let mutation_range = 0.05;

        // Generate a random value within the mutation range
        let mutation = rand::thread_rng().gen_range(-mutation_range..mutation_range);

        // TODO: mutate as a log prob
        // Apply the mutation to the original value
        self.0 = (self.0 + mutation).max(0.0).min(1.0);
    }

    pub fn sample(&self) -> bool {
        rand::thread_rng().gen_bool(self.0)
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Probs {
    rotation: Bernoulli,
    reverse: Bernoulli,
    random_rotate: Bernoulli,
}

impl Probs {
    fn new() -> Self {
        Probs {
            rotation: Bernoulli::new(0.2).unwrap(),
            reverse: Bernoulli::new(0.7).unwrap(),
            random_rotate: Bernoulli::new(0.2).unwrap(),
        }
    }

    fn mutate(&mut self) {
        // Mutate each field with a small random value
        self.rotation.mutate();
        self.reverse.mutate();
        self.random_rotate.mutate();
    }

    fn crossover(&self, other: &Self) -> Self {
        let mut rng = rand::thread_rng();

        Probs {
            rotation: if rng.gen_bool(0.5) {
                self.rotation
            } else {
                other.rotation
            },
            reverse: if rng.gen_bool(0.5) {
                self.reverse
            } else {
                other.reverse
            },
            random_rotate: if rng.gen_bool(0.5) {
                self.random_rotate
            } else {
                other.random_rotate
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct Gene {
    // TODO: use a linked list for faster mutation
    indices: Vec<usize>,
    probs: Probs,
}

trait RandomSubslice {
    fn select_random_subslice_mut(&mut self) -> &mut Self;
}

impl<T> RandomSubslice for [T] {
    fn select_random_subslice_mut(&mut self) -> &mut [T] {
        let rng = &mut rand::thread_rng();
        let idx1 = rng.gen_range(0..self.len() - 2);
        let idx2 = rng.gen_range(idx1 + 2..=self.len());

        &mut self[idx1..idx2]
    }
}

impl Gene {
    fn new(count: usize) -> Self {
        Gene {
            indices: (0..count).collect(),
            probs: Probs::new(),
        }
    }

    fn mutate(&mut self) {
        if self.probs.rotation.sample() {
            self.rotate_random_subsequence()
        } else if self.probs.reverse.sample() {
            self.reverse_random_subsequence()
        } else {
            self.swap_random_positions()
        }
    }

    fn rotate_random_subsequence(&mut self) {
        let slice = self.indices.select_random_subslice_mut();
        let rng = &mut rand::thread_rng();

        let rotation_amount = if self.probs.random_rotate.sample() {
            rng.gen_range(1..slice.len())
        } else {
            1
        };

        slice.rotate_left(rotation_amount)
    }

    fn reverse_random_subsequence(&mut self) {
        self.indices.select_random_subslice_mut().reverse()
    }

    fn swap_random_positions(&mut self) {
        let rng = &mut rand::thread_rng();
        let idx1 = rng.gen_range(0..self.indices.len());
        let idx2 = rng.gen_range(0..self.indices.len());

        self.indices.swap(idx1, idx2)
    }
    // A function to perform crossover between two parents to create a new child gene
    pub fn crossover(parent_1: &Self, parent_2: &Self) -> Self {
        // Determine the length of the gene indices
        let len = parent_1.indices.len();

        // Initialize a random number generator
        let mut rng = rand::thread_rng();

        // Select a random subset of indices from the first parent's gene
        let start = rng.gen_range(0..len);
        let end = rng.gen_range(start..len);
        let selected_slice = &parent_1.indices[start..end];

        // Create a set to store the selected indices
        let mut picked_indices = BTreeSet::from_iter(selected_slice.iter().copied());

        // Initialize a vector to store the child gene indices
        let mut indices = Vec::with_capacity(len);

        // Filter and combine indices from the second parent and the selected indices from the first parent
        let mut filtered_indices = parent_2
            .indices
            .iter()
            .filter(|&idx| picked_indices.insert(*idx));

        // Extend the vector with indices from the second parent before the selected slice
        indices.extend(filtered_indices.borrow_mut().take(start).copied());

        // Add the selected slice from the first parent to the vector
        indices.extend_from_slice(selected_slice);

        // Extend the vector with remaining indices from the second parent after the selected slice
        indices.extend(filtered_indices.copied());

        // Create and return a new gene with the combined indices and crossover of probabilities
        Gene {
            indices,
            probs: parent_1.probs.crossover(&parent_2.probs),
        }
    }

    pub fn probs(&self) -> Probs {
        self.probs
    }
}

fn similarity(color1: Color, color2: Color) -> f64 {
    let distance = color1.compare_cie2000(&color2);

    1.0 / (distance + 0.0001)
}
