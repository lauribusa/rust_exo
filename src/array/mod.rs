pub fn print_array(){

    let mut tech_tree = ["Iron Sword", "Steel Sword", "Khorium Blade", "Iron Armor", "Steel Armor", "Khorium Armor", "Far Sight", "Bloodlust"];
    
    tech_tree[1] = "Glass Blade";
    tech_tree[4] = "Daedra Armor";
    
    for tech in tech_tree{
        println!("{tech}");
    }

}

// Last number is excluded from enumeration
pub fn print_enumeration(){
    for num in 1..100{
        println!("{num}");
    }
}