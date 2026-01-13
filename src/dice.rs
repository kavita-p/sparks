use rand::distr::{Distribution, Uniform};
use rand::rng;

pub struct Rolls {
    pub max: i64,
    pub min: i64,
    pub dice: Vec<i64>,
}
impl Rolls {
    pub fn new(count: i64, sides: i64) -> Self {
        let count = count.try_into().unwrap_or(1);
        let sides = Uniform::new_inclusive(1, sides).unwrap();

        let dice: Vec<i64> = sides.sample_iter(rng()).take(count).collect();

        let max = *dice.iter().max().unwrap_or(&0);
        let min = *dice.iter().min().unwrap_or(&0);

        Self { max, min, dice }
    }

    pub fn join_dice(self) -> String {
        self.dice
            .into_iter()
            .map(|d| d.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }

    pub fn join_dice_confidently(self, original: i64, replacement: i64) -> String {
        self.dice
            .into_iter()
            .map(|d| {
                if d == original {
                    format!("~~{original}~~ (treated as **{replacement}**)")
                } else {
                    d.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join(", ")
    }

    pub fn join_cut_dice(self, drop_count: usize) -> String {
        let mut largest_dice = self
            .dice
            .into_iter()
            .enumerate()
            .collect::<Vec<(usize, i64)>>();
        largest_dice.sort_by(|a, b| b.1.cmp(&a.1));

        let mut marked_dice = largest_dice
            .into_iter()
            .enumerate()
            .map(|(pos, (idx, val))| {
                if pos < drop_count {
                    (idx, val, true)
                } else {
                    (idx, val, false)
                }
            })
            .collect::<Vec<(usize, i64, bool)>>();
        marked_dice.sort_by(|a, b| a.0.cmp(&b.0));

        marked_dice
            .into_iter()
            .map(|(_idx, val, strike)| {
                if strike {
                    format!("~~{val}~~")
                } else {
                    val.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join(", ")
    }
}
