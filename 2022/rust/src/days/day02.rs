#[derive(Debug, Clone, Copy)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl From<char> for Play {
    fn from(value: char) -> Self {
        match value {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Play::Scissors,
            _ => panic!("Invalid input: {}", value),
        }
    }
}

impl From<char> for Outcome {
    fn from(value: char) -> Self {
        match value {
            'X' => Self::Loss,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => panic!("Invalid input: {}", value),
        }
    }
}

impl Play {
    pub fn outcome(self, opp: Self) -> Outcome {
        match (self, opp) {
            // Wins
            (Self::Rock, Self::Scissors) => Outcome::Win,
            (Self::Paper, Self::Rock) => Outcome::Win,
            (Self::Scissors, Self::Paper) => Outcome::Win,

            // Draws
            (Self::Rock, Self::Rock) => Outcome::Draw,
            (Self::Paper, Self::Paper) => Outcome::Draw,
            (Self::Scissors, Self::Scissors) => Outcome::Draw,

            _ => Outcome::Loss,
        }
    }
    
    pub fn points(self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3
        }
    }
}

impl Outcome {
    pub fn play_against(self, opp: Play) -> Play {
        match (self, opp) {
            // Wins
            (Self::Win, Play::Scissors) => Play::Rock,
            (Self::Win, Play::Rock) => Play::Paper,
            (Self::Win, Play::Paper) => Play::Scissors,

            // Draws
            (Self::Draw, Play::Rock) => Play::Rock,
            (Self::Draw, Play::Paper) => Play::Paper,
            (Self::Draw, Play::Scissors) => Play::Scissors,

            // Loss
            (Self::Loss, Play::Paper) => Play::Rock,
            (Self::Loss, Play::Scissors) => Play::Paper,
            (Self::Loss, Play::Rock) => Play::Scissors,
        }
    }

    pub fn points(self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loss => 0,
        }
    }
}

pub fn part_1(input: &str) -> u32 {
    let mut total = 0;
    for line in input.lines() {
        let vec: Vec<_> = line.chars().collect();
        let [a, ' ', b] = vec[..] else { panic!() };

        let opp = Play::from(a);
        let you = Play::from(b);

        total += you.points() + you.outcome(opp).points();
    }
    total
}

pub fn part_2(input: &str) -> u32 {
    let mut total = 0;
    for line in input.lines() {
        let vec: Vec<_> = line.chars().collect();
        let [a, ' ', b] = vec[..] else { panic!() };

        let opp = Play::from(a);
        let outcome = Outcome::from(b);
        let you = outcome.play_against(opp);

        total += you.points() + outcome.points();
    }
    total
}
