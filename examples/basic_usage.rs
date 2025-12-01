use dice_roller_rs::{roll, roll_simple, roll_advantage, roll_disadvantage};

fn main() {
    println!("=== Dice Roller Examples ===\n");
    
    // Basic roll with modifier
    println!("Rolling 2d6+3 (two six-sided dice plus 3):");
    let result = roll(2, 6, 3);
    println!("{}\n", result);
    
    // Multiple dice types
    println!("Rolling 3d8+5 (three eight-sided dice plus 5):");
    let result = roll(3, 8, 5);
    println!("{}\n", result);
    
    // Classic d20
    println!("Rolling a d20:");
    let d20 = roll_simple(20);
    println!("d20: {}\n", d20);
    
    // Advantage and disadvantage (D&D 5e)
    println!("Rolling d20 with advantage:");
    let adv = roll_advantage(20);
    println!("Result: {}\n", adv);
    
    println!("Rolling d20 with disadvantage:");
    let dis = roll_disadvantage(20);
    println!("Result: {}\n", dis);
    
    // Negative modifier
    println!("Rolling 1d20-2:");
    let result = roll(1, 20, -2);
    println!("{}", result);
}
