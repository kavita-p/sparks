use rand::Rng;

use crate::{
    has_unique_elements,
    interpreter::{
        Reply,
        RollStatus::{Crit, Failure, FullSuccess, MixedSuccess},
        WildType,
        WildType::{
            Acquisition, Action, Attack, Creation, Defense, Ratings, Recovery, Watch, Weather,
        },
    },
    Rolls,
};

use super::RollStatus;

fn create_description(status: &RollStatus, roll_type: &WildType, doubles: bool) -> String {
    let mut description = match status {
        FullSuccess => {
            match roll_type {
                Action => "Complete success, no drawbacks. Mark/clear a box on a track.",
                Attack => "Powerful blow. Deal damage and potentially inflict an effect.",
                Defense => "Completely avoid the threat (though some powerful opponents may have aspects that make even a triumph dangerous).",
                Acquisition => "Gain a solid untainted resource.",
                Creation => "Recipient gains temporary benefit related to resources used.",
                Recovery => "Heal two marks of damage to an aspect, ship rating, injury track or mire.",
                Ratings => "Bypass the obstacle safely.",
                Watch => "Montage, Meeting, Tall Tale (gain a Whisper), Tree Shanty, Undercrew Issue, Reflection (heal Mire)",
                Weather => "Weather clears.",
            }
        },
        MixedSuccess => {
            match roll_type {
                Action => "Success with a drawback. Usually marks/clears a box.",
                Attack => "Attack deals damage and maybe associated effect, but you might take some damage, suffer an effect, lose a resource or be put in a less favourable position.",
                Defense => "Avoid the worst but take damage, an effect, a negative change in position, or destruction (or temporary denial) of a resource.",
                Acquisition => "Gain a resource with a negative tag.",
                Creation => "Recipient gains temporary 2-track aspect with downsides, or no downside, but it doesn't quite do what was intended.",
                Recovery => "Heal one mark of damage to an aspect, ship rating, injury track or mire.",
                Ratings => "Bypass the obstacle but mark 1 Rating damage.",
                Watch => "Nearby Ship, Outpost, Survivor Needing Rescue, Wreck or Ruin, Cache of Cargo/Supplies, Conspiracy",
                Weather => "Weather continues as it is.",
            }
        }
        Failure => {
            match roll_type {
                Action =>
                "Failure and narrative complication or drawback. Usually doesn't mark/clear a box.",
                Attack => "Attack misses or does no damage. You definitely take some damage or an effect, and might lose a resource or be put in a less favourable position too.",
                Defense =>  "Take damage, and likely associated effect and loss of resource or position as well.",
                Acquisition => "Resource not found or ruined during collection.",
                Creation => "Creation might be a bizarre ornament/culinary curiosity, but gives no benefits.",
                Recovery => "Add an extra mark of damage to an aspect, ship rating, injury track or mire.",
                Ratings => "Fail to bypass the obstacle and mark 1 Rating damage.",
                Watch => "Weather, Natural Feature, Wonder (heal Mire), Horror, Unsettled Landfall, True Wilds",
                Weather =>  "Driving rain/hail (lowers visibility), blazing sunshine (potential heatstroke), living storm or bizarre weather phenomenon.",
            }
        }
        Crit => unreachable!()
    }.to_string();

    if doubles {
        description += match roll_type {
            Action => "\n\n**Twist**\nAdds a small, potentially useful twist, suggested by any player. Firefly has final say.",
            Attack => "\n\n**Twist... or Critical**\nUnexpected narrative effect/critical with increased impact.",
            Defense =>  "\n\n**Twist... or Counter**\nUnexpected narrative effect, or counter with a mark of damage against them (if in range).",
            Acquisition => "\n\n**Twist**\nGain a resource with a unique or positive tag suggested by you or another player.",
            Creation => "\n\n**Twist**\nCreation has small, unexpected benefit in addition to the usual result.",
            Recovery => "\n\n**Twist**\nYou don't consume the resource used to carry out your recovery.",
            Ratings => "\n\n**Twist**\nAn unexpected event in addition to the result.",
            _ => "",
        }
    }

    description
}

pub fn wild_roll(rolls: Rolls, roll_type: &WildType, zero_d: bool, cut: Option<i64>) -> Reply {
    let special_roll = roll_type == &Watch || roll_type == &Weather;

    let drop_count = cut.unwrap_or(0).try_into().unwrap_or(0);

    let overcut = drop_count >= rolls.dice.len();

    let (score, doubles) = if zero_d || overcut {
        (rand::thread_rng().gen_range(1..=6), false)
    } else if drop_count > 0 {
        let mut sorted_dice = rolls.dice.clone();
        sorted_dice.sort_by(|a, b| b.cmp(a));
        (
            sorted_dice[drop_count],
            !has_unique_elements(&sorted_dice[drop_count..]),
        )
    } else {
        (rolls.max, !has_unique_elements(&rolls.dice))
    };

    let status = if zero_d || overcut && !special_roll {
        if score > 3 {
            MixedSuccess
        } else {
            Failure
        }
    } else {
        match score {
            6 => FullSuccess,
            4 | 5 => MixedSuccess,
            1..=3 => Failure,
            _ => unreachable!(),
        }
    };

    let title_text = match status {
        FullSuccess => match roll_type {
            Watch => "Peace",
            Weather => "Clear Skies",
            _ => "Triumph",
        },
        MixedSuccess => match roll_type {
            Watch => "Order",
            Weather => "Continuation",
            _ => "Conflict",
        },
        Failure => match roll_type {
            Watch => "Nature",
            Weather => "A Change for the Worse",
            _ => "Disaster",
        },
        Crit => unreachable!(),
    };

    let mut title = format!("__{roll_type}__ [{score}] {title_text}");
    if doubles && !special_roll {
        title += " with a twist";
    }

    let mut description = create_description(&status, roll_type, doubles);

    if overcut {
        description += "\n\n**Overcut**\nYou cut your entire dice pool! Rolled a single d6, treating triumphs as conflicts.";
    }

    if zero_d && drop_count == 0 {
        description += "\n\n**Zero Dice**\nYou had nothing in your dice pool! Rolled a single d6, treating triumphs as conflicts.";
    }

    if overcut && drop_count > 0 && zero_d {
        description +=
            "\n\n*Also, hey: what are you even doing that you're cutting from a roll of 0 dice?*";
    }

    let dice = if zero_d || overcut {
        score.to_string()
    } else {
        rolls.join_cut_dice(drop_count)
    };

    Reply {
        title,
        description,
        status,
        dice,
        text: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn action_cut_0() {
        let correct_reply = Reply {
            title: "__Action__ [6] Triumph".into(),
            description: "Complete success, no drawbacks. Mark/clear a box on a track.".into(),
            status: FullSuccess,
            dice: "2, 6, 4".into(),
            text: None,
        };

        let test_rolls = Rolls {
            min: 2,
            max: 6,
            dice: vec![2, 6, 4],
        };

        let sparks_reply = wild_roll(test_rolls, &Action, false, None);

        assert_eq!(sparks_reply, correct_reply);
    }

    #[test]
    fn action_cut_1() {
        let correct_reply = Reply {
            title: "__Action__ [4] Conflict".into(),
            description: "Success with a drawback. Usually marks/clears a box.".into(),
            status: MixedSuccess,
            dice: "2, ~~5~~, 4".into(),
            text: None,
        };

        let test_rolls = Rolls {
            min: 2,
            max: 5,
            dice: vec![2, 5, 4],
        };

        let sparks_reply = wild_roll(test_rolls, &Action, false, Some(1));

        assert_eq!(sparks_reply, correct_reply);
    }

    #[test]
    fn doubles_but_cut() {
        let correct_reply = Reply {
            title: "__Action__ [4] Conflict".into(),
            description: "Success with a drawback. Usually marks/clears a box.".into(),
            status: MixedSuccess,
            dice: "~~4~~, 2, 4".into(),
            text: None,
        };

        let test_rolls = Rolls {
            min: 2,
            max: 4,
            dice: vec![4, 2, 4],
        };

        let sparks_reply = wild_roll(test_rolls, &Action, false, Some(1));

        assert_eq!(sparks_reply, correct_reply);
    }
}
