use sparksrs::Rolls;
use crate::interpreter::{Reply, RollStatus::*};
use std::cmp::Ordering;
use std::fmt::Write as _;

pub fn pbta_move(rolls: Rolls, stat: i32) -> Reply {
    let score: i32 = rolls.dice.iter().sum::<u32>() as i32 + stat;

    let (title_literal, status) = match score {
        12..=i32::MAX => ("Full success!", Crit),
        10 | 11 => ("Full success!", FullSuccess),
        7..=9 => ("Mixed success!", MixedSuccess),
        i32::MIN..=6 => ("Failure!", Failure),
    };

    let mut description = format!("Got **{}** on 2d6", score);

    match score.cmp(&0) {
        Ordering::Greater => { write!(description, " + {}.", stat).unwrap(); },
        Ordering::Equal => { description += "."; }
        Ordering::Less => { write!(description, " - {}.", stat.saturating_abs()).unwrap(); },
    };

    if score >= 12 {
        write!(description, "\n\nYou also gain any bonuses that trigger on a **12+** for this move, if applicable.").unwrap();
    }

    Reply {
        title: String::from(title_literal),
        description,
        status,
        dice: rolls.dice,
    }
}

#[cfg(test)]
mod pbta_interpreter_tests {
    use super::*;


    #[test]
    fn renders_positive_stat () {
        let correct_reply = Reply {
            title: String::from("Full success!"),
            description: String::from("Got **12** on 2d6 + 0.\n\nYou also gain any bonuses that trigger on a **12+** for this move, if applicable."),
            status: Crit,
            dice: vec![6, 6]
        };

        let rolls = Rolls {
            max: 6,
            min: 6,
            dice: vec![6, 6]
        };

        let sparks_reply = pbta_move(rolls, 0);

        assert_eq!(sparks_reply, correct_reply);
    }
}

