use core::str;

struct MyStructWithCallbacks<T: Fn(i32) -> ()> {
    callbacks: Vec<T>,
}

impl<T: Fn(i32) -> ()> MyStructWithCallbacks<T> {
    fn new() -> Self {
        Self {
            callbacks: Vec::new(),
        }
    }
    
    fn add_listener(&mut self, listener: T) {
        self.callbacks.push(listener);
    }

    fn fire_callbacks(&self, value: i32) {
        for callback in &self.callbacks {
             callback(value);
        }
    }
}


fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct A;

pub fn fire_event() {
    let number_list = [1,2,3,4];
    let char_list = ['1','2','3'];
    let string = [String::from("a"), String::from("b")];
    let options: [Option<i32>; 3] = [Some(1), Some(5), None];
    let s: [A; 3] = [A{}, A{}, A{}];

    let largest_number =  largest(&number_list);
    let largest_char = largest(&char_list);
    let largest_string = largest(&string);

    let list = MyStructWithCallbacks {
        callbacks: vec![
            |n| {
                println!("{n}");
            },
            |n| {
                println!("{n}");
            }
        ]
    };

    list.fire_callbacks(50);

    largest(&options);
}