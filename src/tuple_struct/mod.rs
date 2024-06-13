struct Vector3(i32, i32, i32);

impl Vector3 {
    fn zero() -> Vector3{
        Vector3(
            255,
            255,
            255
        )
    }

    fn one() -> Vector3 {
        Vector3(
            1,
            1,
            1
        )
    }

    fn up() -> Vector3 {
        Vector3(0, 1, 0)
    }

    fn x(&self) -> i32{
        self.0
    }

    fn y(&self) -> i32{
        self.1
    }

    fn z(&self) -> i32{
        self.2
    }
}

pub fn tuple_struct_constructor(){
    let white = Vector3::up();
    let black = Vector3::one();

    println!("{}", white.y());
    println!("{}", black.x());
}

struct Password(String);

impl Password {
    fn secret(&self) -> String{
        let mut str = "".to_string();
        for _ in self.0.chars() {
            str.push('*');
        }
        str
    }

    fn compare(&self, pswd: &String) -> bool {
        if self.0.contains(pswd) {
            true
        } else {
            false
        }
    }
}

pub fn create_password_and_compare(){
    let password = Password("grosfroc".to_string());
    let confirm_password = "ravacholle".to_string();
    let confirm_correct_password = "grosfroc".to_string();

    println!("{}", password.secret());

    println!("trying to compare ... {}", password.compare(&confirm_password));

    println!("Trying to compare... {}", password.compare(&confirm_correct_password))
}