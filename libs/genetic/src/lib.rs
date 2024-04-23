use rand::{seq::SliceRandom, Rng, RngCore};

pub struct GeneticAlgorithm<S> {
    select_method: S,
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod,
{
    pub fn new(select_method: S) -> Self {
        Self { select_method }
    }

    pub fn evolve<I>(
        &self,
        rng: &mut impl RngCore,
        population: &[I],
        evaluation: impl Fn(&I) -> f32,
    ) -> Vec<I>
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                // TODO: selection
                let parent_a = self.select_method.select(rng, population);
                let parent_b = self.select_method.select(rng, population);
                // TODO: crossover
                // TODO: mutation
                todo!()
            })
            .collect()
    }
}

pub trait Individual {
    fn fitness(&self) -> f32;
}

pub trait SelectionMethod {
    fn select<'a, I>(&self, rng: &mut impl RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual;
}

pub struct RouletteWheelSelection;

impl RouletteWheelSelection {
    fn new() -> Self {
        Self
    }
}

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(&self, rng: &mut impl RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual,
    {
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("empty population")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeMap;
    use std::iter::FromIterator;

    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    struct TestIndividual {
        fitness: f32,
    }

    impl TestIndividual {
        fn new(fitness: f32) -> Self {
            Self { fitness }
        }
    }

    impl Individual for TestIndividual {
        fn fitness(&self) -> f32 {
            self.fitness
        }
    }

    #[test]
    fn roulette_wheel_selection() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let population = vec![
            TestIndividual::new(2.0),
            TestIndividual::new(1.0),
            TestIndividual::new(4.0),
            TestIndividual::new(3.0),
        ];

        let mut actual_histogram = BTreeMap::new();

        for _ in 0..1000 {
            let actual = RouletteWheelSelection::new().select(&mut rng, &population);
            *actual_histogram.entry(actual.fitness() as i32).or_insert(0) += 1;
        }

        let expected_histogram = BTreeMap::from_iter(vec![(1, 98), (2, 202), (3, 278), (4, 422)]);

        assert_eq!(actual_histogram, expected_histogram);
    }
}
