pub mod dice {

    use rand::prelude::*;

    #[derive(Debug, Copy, Clone)]
    pub struct Die {
        sides: i64,
    }

    #[derive(Debug, Default)]
    pub struct RollExpr {
        dice: Vec<Die>,
        modifier: i64,
    }

    #[derive(Debug, Default)]
    pub struct RollResult {
        pub rolls: Vec<(Die, i64)>,
        pub modifier: i64,
        pub total: i64,
    }

    impl Die {
        // Generic die builder
        pub fn new(sides: i64) -> Self {
            Die { sides }
        }

        // Pre-built instances for common dice
        pub fn d100() -> Self {
            Die { sides: 100 }
        }
        pub fn d20() -> Self {
            Die { sides: 20 }
        }
        pub fn d12() -> Self {
            Die { sides: 12 }
        }
        pub fn d10() -> Self {
            Die { sides: 10 }
        }
        pub fn d8() -> Self {
            Die { sides: 8 }
        }
        pub fn d6() -> Self {
            Die { sides: 6 }
        }
        pub fn d4() -> Self {
            Die { sides: 4 }
        }
        pub fn d2() -> Self {
            Die { sides: 2 }
        }

        // Roll!
        pub fn roll(self) -> i64 {
            let mut rng = thread_rng();
            rng.gen_range(1..=self.sides)
        }
    }

    impl RollExpr {
        pub fn add_die(&mut self, die: Die) {
            self.dice.push(die);
        }

        pub fn set_modifier(&mut self, modifier: i64) {
            self.modifier = modifier;
        }

        pub fn add_modifier(&mut self, modififer: i64) {
            self.modifier += modififer;
        }

        pub fn roll(&self) -> RollResult {
            let mut result = RollResult::default();
            for die in self.dice.iter() {
                let one_roll = die.roll();
                result.rolls.push((*die, one_roll));
                result.total += one_roll;
            }
            result.total += self.modifier;
            result.modifier = self.modifier;
            result
        }
    }
}
