use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
const INPUT: &str = include_str!("./../../inputs/day-21.txt");
#[allow(dead_code)]
const SIMPLE_INPUT: &str = include_str!("./../../inputs/simple/day-21.txt");

fn main() {
    let products = parse_input(INPUT);

    // Star 1
    let empty_ingredients_count = star1(&products);
    println!("There were {} empty ingredients", empty_ingredients_count);

    // Star 2
    let wanted_ingredients = star2(&products);
    println!("The required ingredients are {}", wanted_ingredients);
}

fn parse_input(input: &str) -> Vec<Product> {
    input.trim().lines().map(Product::from).collect()
}

fn star1(products: &[Product]) -> usize {
    let matcher = IngredientMatcher::new(products);
    matcher.count_empty_ingredients_appearances()
}

fn star2(products: &[Product]) -> String {
    let old_matcher = IngredientMatcher::new(products);
    let clean_products = old_matcher.obtain_products_with_only_allergen_ingredients();

    let new_matcher = IngredientMatcher::new(&clean_products);
    let mut ingredients_allergens = new_matcher.get_ingredient_allergen_list();
    ingredients_allergens.sort_by_key(|(_, allergen)| *allergen);
    let ingredients: Vec<&str> = ingredients_allergens
        .iter()
        .map(|(ingr, _)| *ingr)
        .collect();
    ingredients.join(",")
}

struct Product<'a> {
    ingredients: Vec<&'a str>,
    allergens: Vec<&'a str>,
}

impl<'a> Product<'a> {
    fn new(ingredients: Vec<&'a str>, allergens: Vec<&'a str>) -> Self {
        Product {
            ingredients,
            allergens,
        }
    }

    fn from(input: &'a str) -> Self {
        let mut parts = input.trim().split("(contains");
        let ingredients = parts.next().unwrap().trim().split(' ').collect();
        let allergens = parts
            .next()
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .trim()
            .split(", ")
            .collect();

        Product {
            ingredients,
            allergens,
        }
    }
}

type StrToProductMap<'a> = HashMap<&'a str, Vec<&'a Product<'a>>>;
struct IngredientMatcher<'a> {
    products: &'a [Product<'a>],
    ingredient_map: StrToProductMap<'a>,
    allergen_map: StrToProductMap<'a>,
}

impl<'a> IngredientMatcher<'a> {
    fn new(products: &'a [Product]) -> IngredientMatcher<'a> {
        let (ingredient_map, allergen_map): (StrToProductMap, StrToProductMap) =
            Self::get_ingredients_and_allergens_maps(products);
        IngredientMatcher {
            products,
            ingredient_map,
            allergen_map,
        }
    }

    fn get_ingredients_and_allergens_maps(
        products: &'a [Product],
    ) -> (StrToProductMap<'a>, StrToProductMap<'a>) {
        let mut ingredient_map: HashMap<&str, Vec<&Product>> = HashMap::new();
        let mut allergen_map: HashMap<&str, Vec<&Product>> = HashMap::new();
        for product in products.iter() {
            for ingredient in product.ingredients.iter() {
                ingredient_map.entry(ingredient).or_default().push(product);
            }
            for allergen in product.allergens.iter() {
                allergen_map.entry(allergen).or_default().push(product);
            }
        }
        (ingredient_map, allergen_map)
    }

    fn find_empty_ingredients(&self) -> Vec<&str> {
        let ingredients_without_allergen = self
            .ingredient_map
            .iter()
            .filter(|&(ingredient, ingr_products)| {
                let possible_allergens = Self::find_possible_ingredient_allergens(
                    ingredient,
                    ingr_products,
                    &self.allergen_map,
                );
                possible_allergens.len() == 0
            })
            .map(|(k, _)| *k)
            .collect();
        ingredients_without_allergen
    }

    fn count_empty_ingredients_appearances(&self) -> usize {
        let empty_ingredients = self.find_empty_ingredients();
        // Count the amount of products every empty ingredients appeared in
        empty_ingredients
            .iter()
            .map(|ingredient| self.ingredient_map.get(ingredient).unwrap().len())
            .sum()
    }

    fn obtain_products_with_only_allergen_ingredients(&self) -> Vec<Product> {
        let empty_ingredients = self.find_empty_ingredients();
        self.products
            .iter()
            .map(|product| {
                let allergens = product.allergens.clone();
                let ingredients = product
                    .ingredients
                    .iter()
                    .filter(|ingredient| !empty_ingredients.contains(ingredient))
                    .copied()
                    .collect();
                Product::new(ingredients, allergens)
            })
            .collect()
    }

    fn find_possible_ingredient_allergens(
        ingredient: &&str,
        ingredient_products: &Vec<&'a Product>,
        allergen_map: &StrToProductMap,
    ) -> Vec<&'a str> {
        // Get all allergens that appeared in the ingredient's products
        let initial_allergens: HashSet<&str> = ingredient_products
            .iter()
            .flat_map(|product| product.allergens.clone())
            .collect();

        // Filter initial allergens to check which are possible
        initial_allergens
            .iter()
            .filter(|allergen| {
                allergen_map
                    .get(*allergen)
                    // In case allergen is not in the map, it has already found the pairing ingredient
                    .map_or(false, |allergen_products| {
                        // This ingredient needs to be in all of these allergens products in order to pass
                        allergen_products
                            .iter()
                            .all(|product| product.ingredients.contains(&ingredient))
                    })
            })
            .copied()
            .collect()
    }

    fn get_ingredient_allergen_list(&self) -> Vec<(&str, &str)> {
        let mut ingredient_map = self.ingredient_map.clone();
        let mut allergen_map = self.allergen_map.clone();
        assert!(ingredient_map.len() == allergen_map.len());

        let mut pairs: Vec<(&str, &str)> = Vec::with_capacity(ingredient_map.len());

        while allergen_map.len() > 0 {
            let mut new_pairs: Vec<(&str, &str)> = ingredient_map
                .iter()
                .map(|(ingredient, products)| {
                    let possible_allergens = Self::find_possible_ingredient_allergens(
                        ingredient,
                        products,
                        &allergen_map,
                    );
                    (ingredient, possible_allergens)
                })
                .filter(|(_, possible_allergens)| possible_allergens.len() == 1)
                .map(|(ingr, possible_allergens)| (*ingr, possible_allergens[0]))
                .collect();

            for (ingredient, allergen) in new_pairs.iter() {
                ingredient_map.remove(ingredient);
                allergen_map.remove(allergen);
            }

            if new_pairs.len() == 0 {
                panic!("Got stuck, no new pairs found!");
            }

            pairs.append(&mut new_pairs);
        }

        pairs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_star1() {
        let products = parse_input(SIMPLE_INPUT);
        assert_eq!(star1(&products), 5);
    }

    #[test]
    fn full_star1() {
        let products = parse_input(INPUT);
        assert_eq!(star1(&products), 2786);
    }

    #[test]
    fn simple_star2() {
        let products = parse_input(SIMPLE_INPUT);
        assert_eq!(star2(&products), "mxmxvkd,sqjhc,fvjkl");
    }

    #[test]
    fn full_star2() {
        let products = parse_input(INPUT);
        assert_eq!(
            star2(&products),
            "prxmdlz,ncjv,knprxg,lxjtns,vzzz,clg,cxfz,qdfpq"
        );
    }
}
