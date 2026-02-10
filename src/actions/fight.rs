use crate::models::{bosses::Boss, player_models::Player};

pub enum FightResult {
    Won,
    Lost,
}

pub fn fight(player_stats: &Player, boss_stats: &Boss) -> FightResult {
    let p_def = player_stats.defense;
    let p_att = player_stats.attack;
    let mut p_hp = player_stats.hp;

    let b_def = boss_stats.defence;
    let b_att = boss_stats.attack;
    let mut b_hp = boss_stats.hp;

    while p_hp > 0 && b_hp > 0 {
        let hit_given = p_att - b_def;
        b_hp = b_hp - hit_given as i32;
        println!("Hit given {} /n", hit_given);
        println!("Boss health {} /n", b_hp);

        // if boss not dead yet, then compute its hit
        if b_hp > 0 {
            let hit_taken = b_att - p_def;
            p_hp = p_hp - hit_taken as i32;
            println!("Hit taken {}", hit_taken);
            println!("Player health {}", p_hp);

            // if player dead, return right here
            if p_hp <= 0 {
                println!("Player is dead");
                return FightResult::Lost;
            }
        } else {
            println!("Boss is dead");
            return FightResult::Won;
        }
    }

    FightResult::Lost
}
