use std::cmp::Ordering;

#[derive(Clone)]
pub enum Choice {
    Rock,
    Paper,
    Scisors,
}

impl Choice {
    pub fn new(c: char) -> Option<Self> {
        match c {
            'A' | 'X' => Some(Choice::Rock),
            'B' | 'Y' => Some(Choice::Paper),
            'C' | 'Z' => Some(Choice::Scisors),
            _ => None,
        }
    }

    pub fn to_get_result(adversary: &Choice, result: char) -> Result<Choice, &str> {
        match result {
            'X' => match adversary { // I lose
                Choice::Rock => Ok(Choice::Scisors),
                Choice::Paper => Ok(Choice::Rock),
                Choice::Scisors => Ok(Choice::Paper),
            },
            'Y' => Ok(adversary.clone()), // Draw
            'Z' => match adversary { // I win
                Choice::Rock => Ok(Choice::Paper),
                Choice::Paper => Ok(Choice::Scisors),
                Choice::Scisors => Ok(Choice::Rock),
            },
            _ => Err("Invalid outcome"),
        }
    }

    fn points(&self) -> usize {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scisors => 3,
        }
    }

    pub fn play_against(&self, adversary: &Choice) -> usize {
        self.points()
            + match self.partial_cmp(adversary) {
                Some(Ordering::Less) => 0,
                Some(Ordering::Equal) => 3,
                Some(Ordering::Greater) => 6,
                None => 0, // Should never happen
            }
    }
}

impl PartialEq for Choice {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Choice::Rock => match other {
                Choice::Rock => true,
                _ => false,
            },
            Choice::Paper => match other {
                Choice::Paper => true,
                _ => false,
            },
            Choice::Scisors => match other {
                Choice::Scisors => true,
                _ => false,
            },
        }
    }
}

impl PartialOrd for Choice {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Choice::Rock => match other {
                Choice::Rock => Some(Ordering::Equal),
                Choice::Paper => Some(Ordering::Less),
                Choice::Scisors => Some(Ordering::Greater),
            },
            Choice::Paper => match other {
                Choice::Rock => Some(Ordering::Greater),
                Choice::Paper => Some(Ordering::Equal),
                Choice::Scisors => Some(Ordering::Less),
            },
            Choice::Scisors => match other {
                Choice::Rock => Some(Ordering::Less),
                Choice::Paper => Some(Ordering::Greater),
                Choice::Scisors => Some(Ordering::Equal),
            },
        }
    }
}
