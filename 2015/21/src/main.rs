use itertools::Itertools;

fn main() {
    let player = Character {
        hit_points: 100,
        damage: 0,
        armor: 0,
    };

    let boss = Character {
        hit_points: 103,
        damage: 9,
        armor: 2,
    };

    let weapons = vec![
        Item {
            cost: 8,
            damage: 4,
            armor: 0,
        },
        Item {
            cost: 10,
            damage: 5,
            armor: 0,
        },
        Item {
            cost: 25,
            damage: 6,
            armor: 0,
        },
        Item {
            cost: 40,
            damage: 7,
            armor: 0,
        },
        Item {
            cost: 74,
            damage: 8,
            armor: 0,
        },
    ];

    let armor = vec![
        Item {
            cost: 13,
            damage: 0,
            armor: 1,
        },
        Item {
            cost: 31,
            damage: 0,
            armor: 2,
        },
        Item {
            cost: 53,
            damage: 0,
            armor: 3,
        },
        Item {
            cost: 75,
            damage: 0,
            armor: 4,
        },
        Item {
            cost: 102,
            damage: 0,
            armor: 5,
        },
    ];

    let rings = vec![
        Item {
            cost: 25,
            damage: 1,
            armor: 0,
        },
        Item {
            cost: 50,
            damage: 2,
            armor: 0,
        },
        Item {
            cost: 100,
            damage: 3,
            armor: 0,
        },
        Item {
            cost: 20,
            damage: 0,
            armor: 1,
        },
        Item {
            cost: 40,
            damage: 0,
            armor: 2,
        },
        Item {
            cost: 80,
            damage: 0,
            armor: 3,
        },
    ];

    let shopping_lists = generate_shopping_lists(&weapons, &armor, &rings);

    let answer = part_one(&player, &boss, &shopping_lists).expect("no solution for part one");
    println!("part 1: {}", answer);

    let answer = part_two(&player, &boss, &shopping_lists).expect("no solution for part two");
    println!("part 2: {}", answer);
}

#[derive(Debug)]
struct Item {
    cost: u32,
    damage: u32,
    armor: u32,
}

#[derive(Debug, Clone)]
struct Character {
    hit_points: u32,
    damage: u32,
    armor: u32,
}

fn generate_shopping_lists<'a>(
    weapons: &'a [Item],
    armor: &'a [Item],
    rings: &'a [Item],
) -> Vec<Vec<&'a Item>> {
    // weapons: 1
    // (just iterate as usual)

    // armor: 0-1
    let armor_indices = {
        let mut v = vec![usize::MAX];
        v.append(&mut armor.iter().enumerate().map(|(i, _)| i).collect::<Vec<_>>());
        v
    };

    // rings: 0-2
    let rings_indices = {
        let mut v = vec![usize::MAX];
        v.append(&mut rings.iter().enumerate().map(|(i, _)| i).collect::<Vec<_>>());
        let mut v = v
            .into_iter()
            .combinations(2)
            .map(|x| (x[0], x[1]))
            .collect::<Vec<_>>();
        v.push((usize::MAX, usize::MAX));
        v
    };

    let mut v = Vec::new();
    for w in weapons.iter() {
        for ai in armor_indices.iter() {
            for ri in rings_indices.iter() {
                let mut list = vec![w];
                if *ai != usize::MAX {
                    list.push(&armor[*ai]);
                }
                if ri.0 != usize::MAX {
                    list.push(&rings[ri.0]);
                }
                if ri.1 != usize::MAX {
                    list.push(&rings[ri.1]);
                }
                v.push(list);
            }
        }
    }
    v
}

fn fight_to_the_death(first: &mut Character, second: &mut Character) {
    debug_assert!(first.hit_points > 0);
    debug_assert!(second.hit_points > 0);

    loop {
        fight(first, second);
        if second.hit_points == 0 {
            break;
        }
        fight(second, first);
        if first.hit_points == 0 {
            break;
        }
    }
}

fn fight(attacker: &Character, defender: &mut Character) {
    debug_assert!(attacker.hit_points > 0);
    debug_assert!(defender.hit_points > 0);

    let damage = match attacker.damage.saturating_sub(defender.armor) {
        0 => 1,
        x => x,
    };
    defender.hit_points = defender.hit_points.saturating_sub(damage);
}

fn part_one(player: &Character, boss: &Character, shopping_lists: &[Vec<&Item>]) -> Option<u32> {
    let mut shopping_lists = shopping_lists.to_owned();
    shopping_lists.sort_unstable_by_key(|list| list.iter().map(|item| item.cost).sum::<u32>());
    for list in shopping_lists {
        let mut p = player.clone();
        p.damage += list.iter().map(|item| item.damage).sum::<u32>();
        p.armor += list.iter().map(|item| item.armor).sum::<u32>();

        let mut b = boss.clone();

        fight_to_the_death(&mut p, &mut b);
        if b.hit_points == 0 {
            let cost = list.iter().map(|item| item.cost).sum::<u32>();
            return Some(cost);
        }
    }
    None
}

fn part_two(player: &Character, boss: &Character, shopping_lists: &[Vec<&Item>]) -> Option<u32> {
    let mut shopping_lists = shopping_lists.to_owned();
    shopping_lists.sort_unstable_by_key(|list| list.iter().map(|item| item.cost).sum::<u32>());
    for list in shopping_lists.into_iter().rev() {
        let mut p = player.clone();
        p.damage += list.iter().map(|item| item.damage).sum::<u32>();
        p.armor += list.iter().map(|item| item.armor).sum::<u32>();

        let mut b = boss.clone();

        fight_to_the_death(&mut p, &mut b);
        if p.hit_points == 0 {
            let cost = list.iter().map(|item| item.cost).sum::<u32>();
            return Some(cost);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fight() {
        let mut player = Character {
            hit_points: 8,
            damage: 5,
            armor: 5,
        };
        let mut boss = Character {
            hit_points: 12,
            damage: 7,
            armor: 2,
        };

        fight(&player, &mut boss);
        assert_eq!(boss.hit_points, 9);

        fight(&boss, &mut player);
        assert_eq!(player.hit_points, 6);

        fight(&player, &mut boss);
        assert_eq!(boss.hit_points, 6);

        fight(&boss, &mut player);
        assert_eq!(player.hit_points, 4);

        fight(&player, &mut boss);
        assert_eq!(boss.hit_points, 3);

        fight(&boss, &mut player);
        assert_eq!(player.hit_points, 2);

        fight(&player, &mut boss);
        assert_eq!(boss.hit_points, 0);
    }

    #[test]
    fn test_fight_to_the_death() {
        let mut player = Character {
            hit_points: 8,
            damage: 5,
            armor: 5,
        };
        let mut boss = Character {
            hit_points: 12,
            damage: 7,
            armor: 2,
        };
        fight_to_the_death(&mut player, &mut boss);
        assert_ne!(player.hit_points, 0);
        assert_eq!(boss.hit_points, 0);
    }

    #[test]
    fn test_generate_shopping_lists() {
        let weapons = vec![Item {
            cost: 1,
            damage: 0,
            armor: 0,
        }];
        let armor = vec![
            Item {
                cost: 2,
                damage: 0,
                armor: 0,
            },
            Item {
                cost: 3,
                damage: 0,
                armor: 0,
            },
        ];
        let rings = vec![];
        let lists = generate_shopping_lists(&weapons, &armor, &rings);
        assert_eq!(lists.len(), 3);

        let rings = vec![
            Item {
                cost: 5,
                damage: 0,
                armor: 0,
            },
            Item {
                cost: 7,
                damage: 0,
                armor: 0,
            },
            Item {
                cost: 11,
                damage: 0,
                armor: 0,
            },
        ];
        let lists = generate_shopping_lists(&weapons, &armor, &rings);
        assert_eq!(lists.len(), 3 * 7);
    }
}
