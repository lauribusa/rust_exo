	use std::time::{SystemTime, UNIX_EPOCH};

pub fn roll_dice(){
    let roll = rand();
    let result = random_roll(roll);
    println!("Battle results: {result} ({})", roll);
}

fn random_roll(num: u32) -> String{
    if num > 17{
        "Decisive Victory".to_string()
    } else if num > 10{
        "Close Victory".to_string()
    } else if num > 2 {
        "Pyrrhic Victory".to_string()
    } else {
        "Crushing Defeat".to_string()
    }
}
	
pub fn rand() -> u32 {
	
    let since_epoch = SystemTime::now().duration_since(UNIX_EPOCH)
	
        .expect("Le temps a reculé");
	
    let nanos = since_epoch.as_nanos();
	
    (nanos % 21) as u32
	
}