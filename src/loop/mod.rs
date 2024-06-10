use std::time::{SystemTime, UNIX_EPOCH};

pub fn loop_func(){
    let mut loop_count: u8 = 0;
    loop{
        loop_count += 1;
        println!("Nain(s) paré(s): {}", loop_count);
        if loop_count >= 10 {
            break;
        }
    }
    println!("Nains prêts");
}

pub fn fail_check(){
    let mut loop_count: u8 = 0;
    let success: bool = loop{
        loop_count += 1;
        let result: u32 = rand();
        if result >= 18 {
            break true;
        }
        if loop_count >= 10 {
            break false;
        }
    };
    println!("Crit success? {}", success);
}
	
fn rand() -> u32 {
    let since_epoch = SystemTime::now().duration_since(UNIX_EPOCH)
	
        .expect("Time went backwards??");
	
    let nanos = since_epoch.as_nanos();
	
    (nanos % 21) as u32
	
}