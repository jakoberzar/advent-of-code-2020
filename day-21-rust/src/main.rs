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
    input.trim().lines().map(Product::new).collect()
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
    fn new(input: &'a str) -> Self {
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

struct IngredientMatcher<'a> {
    products: &'a [Product<'a>],
}

impl<'a> IngredientMatcher<'a> {
    fn new(products: &'a [Product]) -> IngredientMatcher<'a> {
        IngredientMatcher { products }
    }

    fn get_ingredient_list(&self) -> Vec<&str> {
        let mut all_ingredients: Vec<&str> = self
            .products
            .iter()
            .flat_map(|product| product.ingredients.clone())
            .collect();
        all_ingredients.sort();
        all_ingredients.dedup();
        all_ingredients
    }

    fn get_allergen_list(&self) -> Vec<&str> {
        let mut all_allergens: Vec<&str> = self
            .products
            .iter()
            .flat_map(|product| product.allergens.clone())
            .collect();
        all_allergens.sort();
        all_allergens.dedup();
        all_allergens
    }

    fn get_ingredient_product_map(&self) -> HashMap<&str, Vec<&Product>> {
        self.get_ingredient_list()
            .iter()
            .map(|ingredient| {
                let contained_products = self
                    .products
                    .iter()
                    .filter(|product| product.ingredients.contains(ingredient))
                    .collect();
                (*ingredient, contained_products)
            })
            .collect()
    }

    fn get_allergen_product_map(&self) -> HashMap<&str, Vec<&Product>> {
        self.get_allergen_list()
            .iter()
            .map(|allergen| {
                let contained_products = self
                    .products
                    .iter()
                    .filter(|product| product.allergens.contains(allergen))
                    .collect();
                (*allergen, contained_products)
            })
            .collect()
    }

    fn find_empty_ingredients(&self) -> Vec<&str> {
        let ingredient_map = self.get_ingredient_product_map();
        let allergen_map = self.get_allergen_product_map();
        let ingredients_without_allergen = ingredient_map
            .iter()
            .filter(|&(ingredient, ingr_products)| {
                let initial_allergens: HashSet<&str> = ingr_products
                    .iter()
                    .flat_map(|product| product.allergens.clone())
                    .collect();
                // Filter initial allergens to check which are possible
                let possible_allergens = initial_allergens.iter().filter(|allergen| {
                    // This ingredient needs to be in all of these allergens products in order to pass
                    let allergen_products = allergen_map.get(*allergen).unwrap();
                    allergen_products
                        .iter()
                        .all(|product| product.ingredients.contains(ingredient))
                });
                let count = possible_allergens.count();
                count == 0
            })
            .map(|(k, _)| *k)
            .collect();
        ingredients_without_allergen
    }

    fn count_empty_ingredients_appearances(&self) -> usize {
        let empty_ingredients = self.find_empty_ingredients();
        self.products
            .iter()
            .map(|product| {
                product
                    .ingredients
                    .iter()
                    .filter(|ingredient| empty_ingredients.contains(ingredient))
                    .count()
            })
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
                Product {
                    allergens,
                    ingredients,
                }
            })
            .collect()
    }

    fn get_ingredient_allergen_list(&self) -> Vec<(&str, &str)> {
        let mut ingredient_map = self.get_ingredient_product_map();
        let mut allergen_map = self.get_allergen_product_map();
        assert!(ingredient_map.len() == allergen_map.len());

        let mut pairs: Vec<(&str, &str)> = Vec::with_capacity(ingredient_map.len());

        while allergen_map.len() > 0 {
            let mut new_pairs: Vec<(&str, &str)> = ingredient_map
                .iter()
                .map(|(ingredient, ingr_products)| {
                    let initial_allergens: HashSet<&str> = ingr_products
                        .iter()
                        .flat_map(|product| product.allergens.clone())
                        .collect();
                    // Filter initial allergens to check which are possible
                    let possible_allergens: Vec<&str> = initial_allergens
                        .iter()
                        .filter(|allergen| {
                            // This ingredient needs to be in all of these allergens products in order to pass
                            allergen_map
                                .get(*allergen)
                                .map_or(false, |allergen_products| {
                                    allergen_products
                                        .iter()
                                        .all(|product| product.ingredients.contains(ingredient))
                                })
                        })
                        .copied()
                        .collect();
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
