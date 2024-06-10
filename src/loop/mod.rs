use std::time::{SystemTime, UNIX_EPOCH};

pub fn loop_func(){
    for i in 0..10 {
        println!("Nain(s) paré(s) {}", i + 1);
    }
}

pub fn crit_check(){
    let mut loop_count: u8 = 0;
    let success: bool = loop{
        loop_count += 1;
        let result: u32 = rand();
        println!("Rolling... {result}");
        if result >= 18 {
            println!("Crit! {result}");
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