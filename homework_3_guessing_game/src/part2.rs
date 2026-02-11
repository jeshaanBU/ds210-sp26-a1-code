use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, mut min: u32, mut max: u32) -> u32 {
        // YOUR SOLUTION GOES HERE.
        while min <= max {
            let mid = min + (max - min) / 2;

            match player.ask_to_compare(mid) {
                0 => return mid,
                -1 => {
                    if mid == 0 { break; }      
                    max = mid - 1;
                }
                1 => {
                    min = mid + 1;
                }
                _ => panic!("invalid"),
            }
        }
        min
    }
}