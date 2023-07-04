use rand::prelude::*;

use crate::board::Board;

use super::agent::PlayerTrait;
pub struct RandomAgent {
    rng: Box<dyn RngCore>,
}

impl RandomAgent {
    fn new(rng: Box<dyn RngCore>) -> Self {
        Self { rng }
    }
}

impl PlayerTrait for RandomAgent {
    fn play(&mut self, _board: &Board) -> crate::board::Col {
        self.rng.as_mut().gen_range(0..8)
    }
}

#[cfg(test)]
mod tests {
    use approx::Relative;

    use super::*;
    #[test]
    fn test_creation() {
        let seedable = Box::new(StdRng::seed_from_u64(32));
        let _agent = RandomAgent::new(seedable);
        let random_from_thread = Box::new(thread_rng());
        let _agent = RandomAgent::new(random_from_thread);
    }

    #[test]
    fn test_true_random() {
        const N: usize = 1_000_000usize;
        const ONE_EIGHT: f64 = 1. / 8.;

        let mut rng = thread_rng();
        let board = Board::from_rng(&mut rng);
        let mut agent = RandomAgent::new(Box::new(rng));
        let arr = (0..N).fold([0usize; 8], |mut acc, _numb| {
            acc[agent.play(&board) as usize] += 1;
            acc
        });
        assert!(arr
            .into_iter()
            .map(|el| el as f64 / N as f64)
            .all(|el| Relative::default().epsilon(0.01).eq(&el, &ONE_EIGHT)));
    }
}
