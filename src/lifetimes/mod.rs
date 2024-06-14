fn cleanse_text(s: &str) -> &str {
    s.trim()
}

fn cleanse_text_explicit<'a>(s: &'a str) -> &'a str {
    s.trim()
}

// incorrect
// the compiler cannot guess which lifetime is used for the return type. In this example, rust implicity creates different lifetimes for each parameter, and we must precise which one we're using.
/* fn get_largest_str<'a, 'b>(s1: &'a str, s2: &'b str) -> &str {
    if s1 > s2 {
        s1
    } else {
        s2
    }
} */
// ^ commented so rust can compile everything else

// solution: if we explictly say which lifetime is the correct one, then rust compiles properly.
pub fn get_largest_str_lifetime<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1 > s2 {
        s1
    } else {
        s2
    }
}

#[cfg(test)]
mod test {
    use crate::lifetimes::get_largest_str_lifetime;

    #[test]
    fn test_largest(){
        assert_eq!(get_largest_str_lifetime("a", "ba"), "ba")
    }
}


// GESTION DE CONTACTS
// créer deux struct: contact et addressbook
// faire en sorte que addressbook puisse contenir un vecteur de REFERENCE de contacts qui s'assure que leur lifetime soit égal a la struct qui détiens ce vecteur
