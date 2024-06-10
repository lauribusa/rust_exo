// i32 => entier 32bits (2e32)
// 0 -> 2 400 000 000
// i8 (255) - 128 : 128
// u8 (255) 0 : 255

const HIGHEST_AU: u8 = 200;
const HIGHER_AU: u8 = 150;
const HIGH_AU: u8 = 100;
const LOW_AU: u8 = 40;

pub fn iq_check(){
    let autism: u8 = 120;
    let result = neuro_psy(autism);
    println!("{result}");
}

fn neuro_psy(val: u8) -> String{
    if val > HIGHEST_AU {
        "Profound retardation".to_string()
    } else if 
    val > HIGHER_AU {
        "Deep autism".to_string()
    } else if
    val > HIGH_AU {
        "High autism".to_string()
    } else if 
    val > LOW_AU{
        "Excessive autism".to_string()
    } else {
        "No autism...yet".to_string()
    }
}