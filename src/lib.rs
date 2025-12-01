use rand::Rng;
use std::fmt;

/// Represents a dice roll result
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiceRoll {
    /// The number of dice rolled
    pub num_dice: u32,
    /// The number of sides on each die
    pub sides: u32,
    /// The modifier added to the total
    pub modifier: i32,
    /// Individual roll results
    pub rolls: Vec<u32>,
    /// Total result including modifier
    pub total: i32,
}

impl fmt::Display for DiceRoll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}d{}{:+}: {:?} = {}",
            self.num_dice, self.sides, self.modifier, self.rolls, self.total
        )
    }
}

/// Roll dice with the format NdS+M (e.g., 2d6+3)
/// 
/// # Arguments
/// * `num_dice` - Number of dice to roll
/// * `sides` - Number of sides on each die
/// * `modifier` - Modifier to add to the total (can be negative)
/// 
/// # Examples
/// ```
/// use dice_roller_rs::roll;
/// 
/// // Roll 2d6+3
/// let result = roll(2, 6, 3);
/// assert_eq!(result.num_dice, 2);
/// assert_eq!(result.sides, 6);
/// assert!(result.total >= 5 && result.total <= 15);
/// ```
pub fn roll(num_dice: u32, sides: u32, modifier: i32) -> DiceRoll {
    let mut rng = rand::thread_rng();
    let rolls: Vec<u32> = (0..num_dice)
        .map(|_| rng.gen_range(1..=sides))
        .collect();
    
    let sum: u32 = rolls.iter().sum();
    let total = sum as i32 + modifier;
    
    DiceRoll {
        num_dice,
        sides,
        modifier,
        rolls,
        total,
    }
}

/// Roll a simple die (e.g., d20, d6)
/// 
/// # Examples
/// ```
/// use dice_roller_rs::roll_simple;
/// 
/// let result = roll_simple(20);
/// assert!(result >= 1 && result <= 20);
/// ```
pub fn roll_simple(sides: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=sides)
}

/// Roll with advantage (roll twice, take higher)
/// 
/// # Examples
/// ```
/// use dice_roller_rs::roll_advantage;
/// 
/// let result = roll_advantage(20);
/// assert!(result >= 1 && result <= 20);
/// ```
pub fn roll_advantage(sides: u32) -> u32 {
    let roll1 = roll_simple(sides);
    let roll2 = roll_simple(sides);
    roll1.max(roll2)
}

/// Roll with disadvantage (roll twice, take lower)
/// 
/// # Examples
/// ```
/// use dice_roller_rs::roll_disadvantage;
/// 
/// let result = roll_disadvantage(20);
/// assert!(result >= 1 && result <= 20);
/// ```
pub fn roll_disadvantage(sides: u32) -> u32 {
    let roll1 = roll_simple(sides);
    let roll2 = roll_simple(sides);
    roll1.min(roll2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll_basic() {
        let result = roll(2, 6, 0);
        assert_eq!(result.num_dice, 2);
        assert_eq!(result.sides, 6);
        assert_eq!(result.modifier, 0);
        assert_eq!(result.rolls.len(), 2);
        assert!(result.total >= 2 && result.total <= 12);
    }

    #[test]
    fn test_roll_with_modifier() {
        let result = roll(1, 20, 5);
        assert_eq!(result.num_dice, 1);
        assert_eq!(result.sides, 20);
        assert_eq!(result.modifier, 5);
        assert!(result.total >= 6 && result.total <= 25);
    }

    #[test]
    fn test_roll_with_negative_modifier() {
        let result = roll(1, 6, -2);
        assert!(result.total >= -1 && result.total <= 4);
    }

    #[test]
    fn test_roll_simple() {
        for _ in 0..100 {
            let result = roll_simple(20);
            assert!(result >= 1 && result <= 20);
        }
    }

    #[test]
    fn test_roll_advantage() {
        for _ in 0..100 {
            let result = roll_advantage(20);
            assert!(result >= 1 && result <= 20);
        }
    }

    #[test]
    fn test_roll_disadvantage() {
        for _ in 0..100 {
            let result = roll_disadvantage(20);
            assert!(result >= 1 && result <= 20);
        }
    }
}
