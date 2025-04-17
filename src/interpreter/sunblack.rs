use crate::{
    interpreter::{
        Reply,
        RollStatus::{Crit, Failure, FullSuccess, MixedSuccess},
    },
    Rolls,
};

pub fn r#move(rolls: Rolls, zero_d: bool) -> Reply {
    let sixes = rolls
        .dice
        .iter()
        .filter(|&die_value| *die_value == 6)
        .count();
    let pool = if zero_d { 0 } else { rolls.dice.len() };
    let score = if zero_d { rolls.min } else { rolls.max };

    let status = if sixes > 1 {
        if zero_d {
            FullSuccess
        } else {
            Crit
        }
    } else {
        match score {
            6 => FullSuccess,
            4 | 5 => MixedSuccess,
            1..=3 => Failure,
            _ => unreachable!(),
        }
    };

    let title = match status {
        Crit => "Full success!",
        FullSuccess => "Full success!",
        MixedSuccess => "Mixed success!",
        Failure => "Failure!",
    }
    .to_string();

    let mut description = format!("Got **{score}** on {pool}d");

    if zero_d {
        description.push_str(" (rolled as the lower of 2d.)");
    } else if sixes < 2 {
        description.push('.');
    }

    Reply {
        title,
        description,
        status,
        dice: rolls.join_dice(),
        text: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn full_success() {
        let correct_reply = Reply {
            title: "Full success!".into(),
            description: "Got **6** on 2d.".into(),
            status: FullSuccess,
            dice: "6, 2".into(),
            text: None,
        };

        let rolls = Rolls {
            max: 6,
            min: 2,
            dice: vec![6, 2],
        };

        let sparks_reply = r#move(rolls, false);

        assert_eq!(sparks_reply, correct_reply);
    }
}
