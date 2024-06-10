pub fn loop_func(){
    let mut loop_count: u8 = 0;
    loop{
        println!("Nain(s) paré(s): {}", loop_count);
        loop_count += 1;
        if loop_count >= 10 {
            break;
        }
    }
    println!("Nains prêts");
}