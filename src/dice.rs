pub mod dice {

    use rand::prelude::*;

    #[derive(Copy, Clone)]
    pub struct Die {
        sides: i64,
    }

    impl Die {

        pub fn new(sides: i64) -> Self {
            Die {
                sides
            }
        }

        pub fn roll(self) -> i64 {
            let mut rng = thread_rng();
            rng.gen_range(1..=self.sides)
        }
    }

}
