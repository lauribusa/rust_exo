pub fn create_player(){
    
}

pub struct PlayerCharacter {
    name: String,
}

impl PlayerCharacter{
    pub fn new(name: &str) -> PlayerCharacter{
        PlayerCharacter{
            name: String::from(name)
        }
    }
}