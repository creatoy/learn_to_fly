use rand::{Rng, RngCore};

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn random(rng: &mut impl RngCore, layer_struct: &[usize]) -> Self {
        assert!(layer_struct.len() > 1);

        let layers = layer_struct
            .windows(2)
            .map(|n| Layer::random(rng, n[0], n[1]))
            .collect();

        Self { layers }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }
}

#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    fn random(rng: &mut impl RngCore, inputs: usize, neurons: usize) -> Self {
        let neurons = (0..neurons).map(|_| Neuron::random(rng, inputs)).collect();

        Self { neurons }
    }

    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }
}

#[derive(Debug)]
struct Neuron {
    weights: Vec<f32>,
    bias: f32,
}

impl Neuron {
    fn random(rng: &mut impl RngCore, inputs: usize) -> Self {
        // let mut rng = rand::thread_rng();

        let bias = rng.gen_range((-1.0)..=1.0);
        let weights = (0..inputs).map(|_| rng.gen_range((-1.0)..=1.0)).collect();

        Self { weights, bias }
    }

    fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(self.weights.len(), inputs.len());

        let mut output: f32 = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum();

        output += self.bias;

        output.max(0.0)
    }

    fn back_propagate(&self, inputs: &[f32], error: f32) -> (Vec<f32>, f32) {
        let mut delta = 0.0;
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn random() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let neuron = Neuron::random(&mut rng, 4);

        assert_relative_eq!(
            neuron.weights.as_slice(),
            [0.67383957, 0.8181262, 0.26284897, 0.5238807].as_ref()
        );
        assert_relative_eq!(neuron.bias, -0.6255188);
    }

    #[test]
    fn propagate() {
        let neuron = Neuron {
            bias: 0.5,
            weights: vec![-0.3, 0.8],
        };

        assert_relative_eq!(neuron.propagate(&[-10.0, -10.0]), 0.0);

        assert_relative_eq!(
            neuron.propagate(&[0.5, 1.0]),
            (-0.3 * 0.5) + (0.8 * 1.0) + 0.5
        );
    }
}
