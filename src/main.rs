use std::iter;

use enchantments::{AnvilError, Enchants, ItemType, INCLUSIVE_ENCHANTMENTS, PROTECTIONS};
use items::Item;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

mod enchantments;
mod items;

fn main() {
    for protections in PROTECTIONS {
        dbg!(INCLUSIVE_ENCHANTMENTS
            .into_iter()
            .chain(iter::once(protections))
            .flat_map(|e| {
                iter::repeat_n(
                    Item::from_enchantment(e, ItemType::Other),
                    2_u32.pow(e.max_level) as usize,
                )
                .chain(iter::repeat_n(
                    Item::from_enchantment(e, ItemType::Book),
                    2_u32.pow(e.max_level) as usize,
                ))
            })
            .count());

        let max = INCLUSIVE_ENCHANTMENTS
            .into_iter()
            .chain(iter::once(protections))
            .flat_map(|e| {
                iter::repeat_n(
                    Item::from_enchantment(e, ItemType::Other),
                    e.max_level as usize,
                )
                .chain(iter::repeat_n(
                    Item::from_enchantment(e, ItemType::Book),
                    2_u32.pow(e.max_level) as usize,
                ))
            })
            .permutations(5)
            .par_bridge()
            .filter_map(|permutations| {
                let mut permutations = permutations.into_iter();

                while permutations.len() != 1 {
                    let mut vec = Vec::with_capacity(permutations.len().div_ceil(2));
                    while let (Some(t), Some(e)) = (permutations.next(), permutations.next()) {
                        match t.combine(e) {
                            Ok(e) => vec.push(e),
                            Err(AnvilError::RedundantSequence | AnvilError::TooExpensive) => {
                                return None
                            } // Err(err) => panic!("{err:?}")
                        }
                    }
                    if let Some(e) = permutations.next() {
                        vec.push(e);
                    }

                    permutations = vec.into_iter();
                }

                Some(permutations.next().expect("One item expected"))
            })
            .max_by_key(|e| e.total_cost);

        println!("{:?}: {max:?}", protections.name);
    }
}
