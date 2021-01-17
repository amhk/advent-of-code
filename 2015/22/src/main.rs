use std::collections::HashMap;

fn main() {
    let player = Character {
        hit_points: 50,
        armor: 0,
        damage: 0,
        mana: 500,
        active_spells: HashMap::new(),
    };
    let boss = Character {
        hit_points: 71,
        armor: 0,
        damage: 10,
        mana: 0,
        active_spells: HashMap::new(),
    };

    let min_mana_cost =
        fight(&player, &boss, Difficulty::Normal, 11).expect("no solution for part one");
    println!("part 1: {}", min_mana_cost);

    let min_mana_cost =
        fight(&player, &boss, Difficulty::Hard, 12).expect("no solution for part two");
    println!("part 2: {}", min_mana_cost);
}

#[derive(PartialEq, Clone, Copy)]
enum Difficulty {
    Normal,
    Hard,
}

#[derive(Debug, Clone)]
struct Character {
    hit_points: u32,
    armor: u32,
    damage: u32,
    mana: u32,
    active_spells: HashMap<Action, usize>,
}

impl Character {
    fn is_dead(&self) -> bool {
        self.hit_points == 0
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Action {
    Physical(u32),
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

#[derive(Debug)]
enum SpellCast {
    Direct(u32),
    Effect(u32),
}

fn try_cast_spell(player: &mut Character, action: &Action) -> Option<SpellCast> {
    if player.active_spells.contains_key(action) {
        return None;
    }
    let (cost, duration) = match action {
        Action::Physical(_) => unreachable!(),
        Action::MagicMissile => (53, 0),
        Action::Drain => (73, 0),
        Action::Shield => (113, 6),
        Action::Poison => (173, 6),
        Action::Recharge => (229, 5),
    };
    if player.mana < cost {
        return None;
    }
    player.mana -= cost;
    if duration > 0 {
        player.active_spells.insert(*action, duration);
        Some(SpellCast::Effect(cost))
    } else {
        Some(SpellCast::Direct(cost))
    }
}

fn apply_action(attacker: &mut Character, defender: &mut Character, action: &Action) {
    match action {
        Action::Physical(dmg) => {
            let dmg = match dmg.saturating_sub(defender.armor) {
                0 => 1,
                x => x,
            };
            defender.hit_points = defender.hit_points.saturating_sub(dmg);
        }
        Action::MagicMissile => {
            defender.hit_points = defender.hit_points.saturating_sub(4);
        }
        Action::Drain => {
            defender.hit_points = defender.hit_points.saturating_sub(2);
            attacker.hit_points += 2;
        }
        Action::Shield => {
            attacker.armor = 7; // not cumulative over turns
        }
        Action::Poison => {
            defender.hit_points = defender.hit_points.saturating_sub(3);
        }
        Action::Recharge => {
            attacker.mana += 101;
        }
    }
}

fn apply_effects(mut player: &mut Character, mut boss: &mut Character) {
    player.armor = 0; // in case Shield just expired
    let mut spells = player.active_spells.clone();
    for (action, duration) in spells.iter_mut() {
        apply_action(&mut player, &mut boss, action);
        *duration -= 1;
    }
    player.active_spells = spells
        .iter()
        .filter(|(_, &d)| d > 0)
        .map(|(k, v)| (*k, *v))
        .collect();
}

fn fight(
    player: &Character,
    boss: &Character,
    difficulty: Difficulty,
    depth: usize,
) -> Option<u32> {
    debug_assert!(!player.is_dead());
    debug_assert!(!boss.is_dead());

    let costs = vec![
        fight_action(
            player.clone(),
            boss.clone(),
            Action::MagicMissile,
            difficulty,
            depth,
        ),
        fight_action(
            player.clone(),
            boss.clone(),
            Action::Drain,
            difficulty,
            depth,
        ),
        fight_action(
            player.clone(),
            boss.clone(),
            Action::Shield,
            difficulty,
            depth,
        ),
        fight_action(
            player.clone(),
            boss.clone(),
            Action::Poison,
            difficulty,
            depth,
        ),
        fight_action(
            player.clone(),
            boss.clone(),
            Action::Recharge,
            difficulty,
            depth,
        ),
    ];
    costs.iter().filter_map(|x| *x).min()
}

fn fight_action(
    mut player: Character,
    mut boss: Character,
    action: Action,
    difficulty: Difficulty,
    depth: usize,
) -> Option<u32> {
    if difficulty == Difficulty::Hard {
        player.hit_points -= 1;
        if player.is_dead() {
            return None;
        }
    }

    // player's turn
    apply_effects(&mut player, &mut boss);
    if boss.is_dead() {
        return Some(0);
    }

    let spell_cast = try_cast_spell(&mut player, &action)?;

    if let SpellCast::Direct(cost) = spell_cast {
        apply_action(&mut player, &mut boss, &action);
        if player.is_dead() {
            return None;
        }
        if boss.is_dead() {
            return Some(cost);
        }
    }

    // boss' turn
    apply_effects(&mut player, &mut boss);
    if boss.is_dead() {
        let cost = match spell_cast {
            SpellCast::Direct(cost) => cost,
            SpellCast::Effect(cost) => cost,
        };
        return Some(cost);
    }

    let boss_action = Action::Physical(boss.damage);
    apply_action(&mut boss, &mut player, &boss_action);
    if player.is_dead() {
        return None;
    }

    if depth == 0 {
        return None;
    }

    match fight(&player, &boss, difficulty, depth - 1) {
        None => None,
        Some(subcost) => match spell_cast {
            SpellCast::Direct(cost) => Some(cost + subcost),
            SpellCast::Effect(cost) => Some(cost + subcost),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fight_1() {
        let mut player = Character {
            hit_points: 10,
            armor: 0,
            damage: 0,
            mana: 250,
            active_spells: HashMap::new(),
        };
        let mut boss = Character {
            hit_points: 13,
            armor: 0,
            damage: 8,
            mana: 0,
            active_spells: HashMap::new(),
        };

        let boss_action = Action::Physical(boss.damage);

        // player's turn
        assert_eq!(player.hit_points, 10);
        assert_eq!(player.mana, 250);
        assert_eq!(boss.hit_points, 13);
        assert!(try_cast_spell(&mut player, &Action::Poison).is_some());

        // boss' turn
        assert_eq!(player.hit_points, 10);
        assert_eq!(player.mana, 77);
        assert_eq!(boss.hit_points, 13);
        apply_action(&mut player, &mut boss, &Action::Poison);
        apply_action(&mut boss, &mut player, &boss_action);

        // player's turn
        assert_eq!(player.hit_points, 2);
        assert_eq!(player.mana, 77);
        assert_eq!(boss.hit_points, 10);
        apply_action(&mut player, &mut boss, &Action::Poison);
        assert!(try_cast_spell(&mut player, &Action::MagicMissile).is_some());
        apply_action(&mut player, &mut boss, &Action::MagicMissile);

        // boss's turn
        assert_eq!(player.hit_points, 2);
        assert_eq!(player.mana, 24);
        assert_eq!(boss.hit_points, 3);
        apply_action(&mut player, &mut boss, &Action::Poison);
        assert_eq!(boss.hit_points, 0);
    }

    #[test]
    fn test_fight_2() {
        let mut player = Character {
            hit_points: 10,
            armor: 0,
            damage: 0,
            mana: 250,
            active_spells: HashMap::new(),
        };
        let mut boss = Character {
            hit_points: 14,
            armor: 0,
            damage: 8,
            mana: 0,
            active_spells: HashMap::new(),
        };

        let boss_action = Action::Physical(boss.damage);

        // -- Player turn --
        // - Player has 10 hit points, 0 armor, 250 mana
        // - Boss has 14 hit points
        // Player casts Recharge.
        apply_effects(&mut player, &mut boss);
        assert_eq!(player.hit_points, 10);
        assert_eq!(player.armor, 0);
        assert_eq!(player.mana, 250);
        assert_eq!(boss.hit_points, 14);
        assert!(try_cast_spell(&mut player, &Action::Recharge).is_some());

        // -- Boss turn --
        // - Player has 10 hit points, 0 armor, 21 mana
        // - Boss has 14 hit points
        // Recharge provides 101 mana; its timer is now 4.
        // Boss attacks for 8 damage!
        apply_effects(&mut player, &mut boss);
        assert_eq!(player.hit_points, 10);
        assert_eq!(player.armor, 0);
        assert_eq!(player.mana, 122);
        assert_eq!(boss.hit_points, 14);
        assert_eq!(player.active_spells.get(&Action::Recharge), Some(&4));
        apply_action(&mut boss, &mut player, &boss_action);

        // -- Player turn --
        // - Player has 2 hit points, 0 armor, 122 mana
        // - Boss has 14 hit points
        // Recharge provides 101 mana; its timer is now 3.
        // Player casts Shield, increasing armor by 7.
        apply_effects(&mut player, &mut boss);
        assert_eq!(player.hit_points, 2);
        assert_eq!(player.armor, 0);
        assert_eq!(player.mana, 223);
        assert_eq!(boss.hit_points, 14);
        assert_eq!(player.active_spells.get(&Action::Recharge), Some(&3));
        assert!(try_cast_spell(&mut player, &Action::Shield).is_some());

        // -- Boss turn --
        // - Player has 2 hit points, 7 armor, 110 mana
        // - Boss has 14 hit points
        // Shield's timer is now 5.
        // Recharge provides 101 mana; its timer is now 2.
        // Boss attacks for 8 - 7 = 1 damage!
        apply_effects(&mut player, &mut boss);
        assert_eq!(player.hit_points, 2);
        assert_eq!(player.armor, 7);
        assert_eq!(player.mana, 211);
        assert_eq!(boss.hit_points, 14);
        assert_eq!(player.active_spells.get(&Action::Recharge), Some(&2));
        apply_action(&mut boss, &mut player, &boss_action);

        // -- Player turn --
        // - Player has 1 hit point, 7 armor, 211 mana
        // - Boss has 14 hit points
        // Shield's timer is now 4.
        // Recharge provides 101 mana; its timer is now 1.
        // Player casts Drain, dealing 2 damage, and healing 2 hit points.
        apply_effects(&mut player, &mut boss);
        assert_eq!(player.hit_points, 1);
        assert_eq!(player.armor, 7);
        assert_eq!(player.mana, 312);
        assert_eq!(boss.hit_points, 14);
        assert_eq!(player.active_spells.get(&Action::Recharge), Some(&1));
        assert!(try_cast_spell(&mut player, &Action::Drain).is_some());
        apply_action(&mut player, &mut boss, &Action::Drain);

        // -- Boss turn --
        // - Player has 3 hit points, 7 armor, 239 mana
        // - Boss has 12 hit points
        // Shield's timer is now 3.
        // Recharge provides 101 mana; its timer is now 0.
        // Recharge wears off.
        // Boss attacks for 8 - 7 = 1 damage!
        apply_effects(&mut player, &mut boss);
        assert_eq!(player.hit_points, 3);
        assert_eq!(player.armor, 7);
        assert_eq!(player.mana, 340);
        assert_eq!(boss.hit_points, 12);
        assert_eq!(player.active_spells.get(&Action::Recharge), None);
        apply_action(&mut boss, &mut player, &boss_action);

        // -- Player turn --
        // - Player has 2 hit points, 7 armor, 340 mana
        // - Boss has 12 hit points
        // Shield's timer is now 2.
        // Player casts Poison.
        apply_effects(&mut player, &mut boss);
        assert_eq!(player.hit_points, 2);
        assert_eq!(player.armor, 7);
        assert_eq!(player.mana, 340);
        assert_eq!(boss.hit_points, 12);
        assert_eq!(player.active_spells.get(&Action::Shield), Some(&2));
        assert!(try_cast_spell(&mut player, &Action::Poison).is_some());

        // -- Boss turn --
        // - Player has 2 hit points, 7 armor, 167 mana
        // - Boss has 12 hit points
        // Shield's timer is now 1.
        // Poison deals 3 damage; its timer is now 5.
        // Boss attacks for 8 - 7 = 1 damage!
        apply_effects(&mut player, &mut boss);
        assert_eq!(player.hit_points, 2);
        assert_eq!(player.armor, 7);
        assert_eq!(player.mana, 167);
        assert_eq!(boss.hit_points, 12 - 3);
        assert_eq!(player.active_spells.get(&Action::Shield), Some(&1));
        assert_eq!(player.active_spells.get(&Action::Poison), Some(&5));
        apply_action(&mut boss, &mut player, &boss_action);

        // -- Player turn --
        // - Player has 1 hit point, 7 armor, 167 mana
        // - Boss has 9 hit points
        // Shield's timer is now 0.
        // Shield wears off, decreasing armor by 7.
        // Poison deals 3 damage; its timer is now 4.
        // Player casts Magic Missile, dealing 4 damage.
        apply_effects(&mut player, &mut boss);
        assert_eq!(player.hit_points, 1);
        assert_eq!(player.armor, 7);
        assert_eq!(player.mana, 167);
        assert_eq!(boss.hit_points, 9 - 3);
        assert_eq!(player.active_spells.get(&Action::Shield), None);
        assert_eq!(player.active_spells.get(&Action::Poison), Some(&4));
        assert!(try_cast_spell(&mut player, &Action::MagicMissile).is_some());
        apply_action(&mut player, &mut boss, &Action::MagicMissile);

        // -- Boss turn --
        // - Player has 1 hit point, 0 armor, 114 mana
        // - Boss has 2 hit points
        // Poison deals 3 damage. This kills the boss, and the player wins.
        assert_eq!(player.hit_points, 1);
        assert_eq!(player.mana, 114);
        assert_eq!(boss.hit_points, 2);
        apply_effects(&mut player, &mut boss);
        assert_eq!(player.armor, 0);
        assert!(boss.is_dead());
    }
}
