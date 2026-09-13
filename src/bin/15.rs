#![expect(dead_code)]
use std::cmp::max;

advent_of_code::solution!(15);

struct Ingredient {
    name: String,
    cap: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

fn get_ingredients() -> Vec<Ingredient> {
    vec![
        Ingredient {
            name: "Sprinkles".to_owned(),
            cap: 2,
            durability: 0,
            flavor: -2,
            texture: 0,
            calories: 3,
        },
        Ingredient {
            name: "Butterscotch".to_owned(),
            cap: 0,
            durability: 5,
            flavor: -3,
            texture: 0,
            calories: 3,
        },
        Ingredient {
            name: "Chocolate".to_owned(),
            cap: 0,
            durability: 0,
            flavor: 5,
            texture: -1,
            calories: 8,
        },
        Ingredient {
            name: "Candy".to_owned(),
            cap: 0,
            durability: -1,
            flavor: 0,
            texture: 5,
            calories: 8,
        },
    ]
}

pub fn part_one(_input: &str) -> Option<i64> {
    let ingredients = get_ingredients();
    let mut max_score = i64::MIN;
    for sprinkles in 0..=100 {
        for butter in 0..=100 - sprinkles {
            for chocolate in 0..=100 - sprinkles - butter {
                for candy in 0..=100 - sprinkles - butter - chocolate {
                    if candy + sprinkles + butter + chocolate != 100 {
                        continue;
                    }
                    let capacity_score = max(
                        0,
                        ingredients[0].cap * sprinkles
                            + ingredients[1].cap * butter
                            + ingredients[2].cap * chocolate
                            + ingredients[3].cap * candy,
                    );
                    let durability_score = max(
                        0,
                        ingredients[0].durability * sprinkles
                            + ingredients[1].durability * butter
                            + ingredients[2].durability * chocolate
                            + ingredients[3].durability * candy,
                    );
                    let flavor_score = max(
                        0,
                        ingredients[0].flavor * sprinkles
                            + ingredients[1].flavor * butter
                            + ingredients[2].flavor * chocolate
                            + ingredients[3].flavor * candy,
                    );
                    let texture_score = max(
                        0,
                        ingredients[0].texture * sprinkles
                            + ingredients[1].texture * butter
                            + ingredients[2].texture * chocolate
                            + ingredients[3].texture * candy,
                    );
                    let this_score =
                        capacity_score * durability_score * flavor_score * texture_score;

                    max_score = max(max_score, this_score);
                }
            }
        }
    }
    Some(max_score)
}

pub fn part_two(_input: &str) -> Option<i64> {
    let ingredients = get_ingredients();
    let mut max_score = i64::MIN;
    for sprinkles in 0..=100 {
        for butter in 0..=100 - sprinkles {
            for chocolate in 0..=100 - sprinkles - butter {
                for candy in 0..=100 - sprinkles - butter - chocolate {
                    if candy + sprinkles + butter + chocolate != 100 {
                        continue;
                    }
                    let capacity_score = max(
                        0,
                        ingredients[0].cap * sprinkles
                            + ingredients[1].cap * butter
                            + ingredients[2].cap * chocolate
                            + ingredients[3].cap * candy,
                    );
                    let durability_score = max(
                        0,
                        ingredients[0].durability * sprinkles
                            + ingredients[1].durability * butter
                            + ingredients[2].durability * chocolate
                            + ingredients[3].durability * candy,
                    );
                    let flavor_score = max(
                        0,
                        ingredients[0].flavor * sprinkles
                            + ingredients[1].flavor * butter
                            + ingredients[2].flavor * chocolate
                            + ingredients[3].flavor * candy,
                    );
                    let texture_score = max(
                        0,
                        ingredients[0].texture * sprinkles
                            + ingredients[1].texture * butter
                            + ingredients[2].texture * chocolate
                            + ingredients[3].texture * candy,
                    );

                    let calories = (sprinkles + butter) * 3 + (chocolate + candy) * 8;

                    if calories == 500 {
                        max_score = max(
                            max_score,
                            capacity_score * durability_score * flavor_score * texture_score,
                        );
                    }
                }
            }
        }
    }
    // for combination in (0..=100)
    //     .combinations_with_replacement(num_ingredients)
    //     .filter(|c| c.iter().sum::<i64>() == 100)
    // {
    //     println!("{:?}", combination);
    // }
    Some(max_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
