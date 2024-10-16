// SRD rules that are needed to interpret character choices

pub mod srd {

    use std::fmt;
    
    // Enum value corresponds to the order in an unlabeled array
    #[repr(usize)]
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum AbilityType {
        Strength = 0,
        Dexterity = 1,
        Constitution = 2,
        Intelligence = 3,
        Wisdom = 4,
        Charisma = 5,
    }

    #[derive(Debug, Clone)]
    pub enum SkillType {
        Acrobatics,
        AnimalHandling,
        Arcana,
        Athletics,
        Deception,
        History,
        Insight,
        Investigation,
        Intimidation,
        Medicine,
        Nature,
        Perception,
        Performance,
        Persuation,
        Religion,
        SleightOfHand,
        Stealth,
        Survival,
    }

    pub fn get_pb_by_level(level: i64) -> Option<i64> {
        return match level {
            1..=4 => Some(2),
            5..=8 => Some(3),
            9..=12 => Some(4),
            13..=16 => Some(5),
            17..=20 => Some(6),
            _ => None,
        };
    }

    pub fn get_spell_slots_full(level: i64) -> [i64; 9] {
        match level {
            1  => [2, 0, 0, 0, 0, 0, 0, 0, 0],
            2  => [3, 0, 0, 0, 0, 0, 0, 0, 0],
            3  => [4, 2, 0, 0, 0, 0, 0, 0, 0],
            4  => [4, 3, 0, 0, 0, 0, 0, 0, 0],
            5  => [4, 3, 2, 0, 0, 0, 0, 0, 0],
            6  => [4, 3, 3, 0, 0, 0, 0, 0, 0],
            7  => [4, 3, 3, 1, 0, 0, 0, 0, 0],
            8  => [4, 3, 3, 2, 0, 0, 0, 0, 0],
            9  => [4, 3, 3, 3, 1, 0, 0, 0, 0],
            10 => [4, 3, 3, 3, 2, 0, 0, 0, 0],
            11 => [4, 3, 3, 3, 2, 1, 0, 0, 0],
            12 => [4, 3, 3, 3, 2, 1, 0, 0, 0],
            13 => [4, 3, 3, 3, 2, 1, 1, 0, 0],
            14 => [4, 3, 3, 3, 2, 1, 1, 0, 0],
            15 => [4, 3, 3, 3, 2, 1, 1, 1, 0],
            16 => [4, 3, 3, 3, 2, 1, 1, 1, 0],
            17 => [4, 3, 3, 3, 2, 1, 1, 1, 1],
            18 => [4, 3, 3, 3, 2, 1, 1, 1, 1],
            19 => [4, 3, 3, 3, 2, 2, 1, 1, 1],
            20 => [4, 3, 3, 3, 2, 2, 2, 1, 1],
            _ => panic!("Unrecognized value: {level}"),
        }
    }

    pub fn get_spell_slots_half(level: i64) -> [i64; 9] {
        match level {
            1  => [0, 0, 0, 0, 0, 0, 0, 0, 0],
            2  => [2, 0, 0, 0, 0, 0, 0, 0, 0],
            3  => [3, 0, 0, 0, 0, 0, 0, 0, 0],
            4  => [3, 0, 0, 0, 0, 0, 0, 0, 0],
            5  => [4, 2, 0, 0, 0, 0, 0, 0, 0],
            6  => [4, 2, 0, 0, 0, 0, 0, 0, 0],
            7  => [4, 3, 0, 0, 0, 0, 0, 0, 0],
            8  => [4, 3, 0, 0, 0, 0, 0, 0, 0],
            9  => [4, 3, 2, 0, 0, 0, 0, 0, 0],
            10 => [4, 3, 2, 0, 0, 0, 0, 0, 0],
            11 => [4, 3, 3, 0, 0, 0, 0, 0, 0],
            12 => [4, 3, 3, 0, 0, 0, 0, 0, 0],
            13 => [4, 3, 3, 1, 0, 0, 0, 0, 0],
            14 => [4, 3, 3, 1, 0, 0, 0, 0, 0],
            15 => [4, 3, 3, 2, 0, 0, 0, 0, 0],
            16 => [4, 3, 3, 2, 0, 0, 0, 0, 0],
            17 => [4, 3, 3, 3, 1, 0, 0, 0, 0],
            18 => [4, 3, 3, 3, 1, 0, 0, 0, 0],
            19 => [4, 3, 3, 3, 2, 0, 0, 0, 0],
            20 => [4, 3, 3, 3, 2, 0, 0, 0, 0],
            _ => panic!("Unrecognized value: {level}"),
        }
    }


    impl AbilityType {
        pub fn get_mod(score: i64) -> i64 {
            if score >= 0 {
                return (score - 10) / 2;
            } else {
                return (score - 11) / 2;
            }
        }

        pub fn from_u32(value: u32) -> AbilityType {
            match value {
                0 => AbilityType::Strength,
                1 => AbilityType::Dexterity,
                2 => AbilityType::Constitution,
                3 => AbilityType::Intelligence,
                4 => AbilityType::Wisdom,
                5 => AbilityType::Charisma,
                _ => panic!("Unknown value {}", value),
            }
        }
    }

    impl SkillType {
        pub fn get_assoc_ability(&self) -> AbilityType {
            match self {
                SkillType::Athletics      => AbilityType::Strength,
                SkillType::Acrobatics     => AbilityType::Dexterity,
                SkillType::SleightOfHand  => AbilityType::Dexterity,
                SkillType::Stealth        => AbilityType::Dexterity,
                SkillType::Arcana         => AbilityType::Intelligence,
                SkillType::History        => AbilityType::Intelligence,
                SkillType::Investigation  => AbilityType::Intelligence,
                SkillType::Nature         => AbilityType::Intelligence,
                SkillType::Religion       => AbilityType::Intelligence,
                SkillType::AnimalHandling => AbilityType::Wisdom,
                SkillType::Insight        => AbilityType::Wisdom,
                SkillType::Medicine       => AbilityType::Wisdom,
                SkillType::Perception     => AbilityType::Wisdom,
                SkillType::Survival       => AbilityType::Wisdom,
                SkillType::Deception      => AbilityType::Charisma,
                SkillType::Intimidation   => AbilityType::Charisma,
                SkillType::Performance    => AbilityType::Charisma,
                SkillType::Persuation     => AbilityType::Charisma,
            }
        }
    }

    impl fmt::Display for AbilityType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    impl fmt::Display for SkillType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }
}

