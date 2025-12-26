use rand::Rng;

use crate::game::difficulty::Difficulty;

const MAX_VALUE: u8 = 75;

pub struct SecretNumber {
    value: u8,
    hint: Option<String>,
}

impl SecretNumber {
    pub fn new(difficulty: Difficulty) -> Self {
        let mut rng = rand::thread_rng();

        let value = match difficulty {
            Difficulty::Easy => rng.gen_range(1..=25),
            Difficulty::Normal => rng.gen_range(1..=50),
            Difficulty::Hard => rng.gen_range(1..=75),
        };

        Self { value, hint: None }
    }

    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn hint(&self) -> Option<&str> {
        self.hint.as_deref()
    }

    pub fn generate_hint(&mut self) {
        if self.hint.is_some() {
            return;
        }

        let mut rng = rand::thread_rng();
        let min = loop {
            let random = rng.gen_range(1..=5);
            break self.value.saturating_sub(random).max(1);
        };
        let max = loop {
            let random = rng.gen_range(1..=5);
            break (self.value + random).min(MAX_VALUE);
        };

        self.hint = Some(format!("El numero secreto esta entre: {} - {}", min, max))
    }
}
