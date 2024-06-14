struct UniqueArray<T> {
    content: Vec<T>
}

impl<T> UniqueArray<T>{
    fn new() -> UniqueArray<T>{
        UniqueArray{
            content: vec![]
        }
    }

    fn push(&mut self, value: T) -> Result<&Vec<T>, &'static str> where T: PartialEq {
        if self.content.contains(&value) {
            return Err("Element already exists")
        }
        self.content.push(value);
        Ok(&self.content)
    }

    fn get(&self, value: T) -> Result<T, &'static str> where T: PartialEq {
        if !self.content.contains(&value) {
            return Err("No such value exists")
        }
        Ok(value)
    }

    fn len(&self) -> usize {
        self.content.len()
    }

    fn print(&self) where T: std::fmt::Display{
        for v in &self.content {
            println!("{}", &v);
        }
    }
}

pub fn unique_array(){
    let mut array_u8 = UniqueArray::<u8>::new();
    let mut array_string = UniqueArray::<String>::new();

    array_push(&mut array_string, String::from("Boules"));
    array_push(&mut array_string, String::from("Boules"));
    array_push(&mut array_string, String::from("Un gros"));
    array_push(&mut array_u8, 9_u8);
    array_push(&mut array_u8, 10_u8);
    array_push(&mut array_u8, 20_u8);

    array_get(&array_u8, 9_u8);
    array_get(&array_u8, 1_u8);

    println!("length of string: {}", array_string.len());

    array_u8.print();
    array_string.print();
}

fn array_push<T>(unique_array: &mut UniqueArray<T>, val: T) where T: PartialEq{
    let result = unique_array.push(val);
    match result {
        Ok(v) => {
            println!("Push successful");
        },
        Err(e) => {
            eprintln!("Couldn't push for reasons {}", e);
        },
    }
}

fn array_get<T>(unique_array: &UniqueArray<T>, val: T) where T: PartialEq {
    let result = unique_array.get(val);
    match result {
        Ok(v) => {
            println!("Got value");
        },
        Err(e) => {
            eprintln!("Couldn't get for reason: {}", e);
        },
    }
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    &list[0]
}

pub fn compare_stuff(){
    let number_list = [1, 2, 3, 4];
}