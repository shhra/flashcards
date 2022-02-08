#[derive(Debug, Clone)]
pub struct Stats {
    pub interval: i64,
    pub num_reps: i16,
    pub difficultly: f64,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            interval: 0,
            num_reps: 0,
            difficultly: 1.3,
        }
    }

    pub fn repeat(&mut self, grade: i8) {
        /* Handle correct response */
        if grade >= 2 {
            match self.num_reps {
                0 => self.interval = 1,
                1 => self.interval = 4,
                _ => self.interval = (self.interval as f64 * self.difficultly).round() as i64, // Confirm rounding later.
            };
            self.num_reps += 1;
        } else
        /* Incorrect Reponse */
        {
            self.num_reps = 0;
            self.interval = 1;
        }
        let diff = 4.0 - grade as f64;
        self.difficultly = self.difficultly + (0.1 - diff * (0.08 + diff * 0.02));
        if self.difficultly <= 1.3 {
            self.difficultly = 1.3;
        }
    }
}
