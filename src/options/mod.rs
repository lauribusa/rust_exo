pub fn create_options_table() {
    let v = vec![1, 2, 3, 4, 5];
    let value = get_safe(&v, 2);
    let no_value = get_safe(&v, 10);
}

// this is like a nullable in c#
fn get_safe(v: &Vec<i32>, i: i32) -> Option<i32> {
    if i < 0 || i >= v.len() as i32 {
        println!("Out of bounds [{}]", i);
        None
    } else {
        println!("Found value [{}] {}", i,v[i as usize] );
        Some(v[i as usize])
    }
}

fn create_range() {
    let x = std::ops::Range { start: 0, end: 10 };
    // équivalent à
    let y = 0..10;
    println!("{}", x == y);

    for n in y {
        println!("{}", n);
    }
}

struct User {
    name: String,
    email: Option<String>,
}
pub fn create_user_table() {
    let user_with_mail = User {
        name: "Alice".to_string(),
        email: Some(String::from("truc@gmail.com")),
    };
    let user_without_mail = User {
        name: "Bob".to_string(),
        email: None,
    };
}
