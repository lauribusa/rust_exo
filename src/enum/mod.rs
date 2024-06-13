
struct Character{
    name: String,
    position: Vector2
}

enum Terrain {
    CONCRETE,
    GRASS,
    WATER
}

struct Vector2(f64, f64);

impl Character{
    fn new(name: String) -> Character {
        Character { name, position: Vector2(0 as f64,0 as f64) }
    }

    fn get_terrain_speed(&self, terrain: Terrain) -> u8{
        match terrain {
            Terrain::CONCRETE => 2 as u8,
            Terrain::GRASS => 1 as u8,
            Terrain::WATER => 0 as u8,
        }
    }
}

pub fn character_creation(){
    let ch = Character::new(String::from("Barakie"));
    println!("Speed for concrete: {}", ch.get_terrain_speed(Terrain::CONCRETE));
}

// We can assign totally different values to enum
// All the same or dynamic

/* A struct is one thing at once, but an enum can be any number of things at different times. */
enum OrcBuilding {
    WarHall(i32),
    GruntCamp(i32),
    DefenseTower(i32)
}
enum OrcUnit {
    Peon(i32),
    Grunt(i32, i32),
    Troll(i32, i32, i32),
    Warlock(i32, i32, i32, i32),
    BladeMaster(String, i32, i32)
}

// as given example above, you can have different tuples of many different values.

// Enum of c-struct
struct SummonDreadlord {
    name: String,
    hp: u32,
    power: u32,
    duration: u32
}

enum OrcMagic{
    Bloodlust(u32),
    // the struct below **cannot** use impl
    ChainLightning { number_of_bounces: u32, power: u32 },
    // but this one can, as it returns a proper struct
    SummonDreadlord(SummonDreadlord)
}

impl OrcMagic{
    fn new(&self) -> OrcMagic {
        match &self {
            OrcMagic::Bloodlust(_) => Self::Bloodlust(10),
            OrcMagic::ChainLightning { number_of_bounces, power } => Self::ChainLightning { number_of_bounces: 3, power: 10 },
            OrcMagic::SummonDreadlord(_) => Self::SummonDreadlord(SummonDreadlord { name: String::from("Varimathras"), hp: 100, power: 2, duration: 60 }),
        }
    } 

    fn print(&self) {
        match &self {
            OrcMagic::Bloodlust(duration) => println!("Bloodlust duration: {}s", {&duration}),
            OrcMagic::ChainLightning { number_of_bounces, power } => println!("La compétence Chaîne d'Éclairs. bounces: {}, power: {}", {&number_of_bounces}, {&power}),
            OrcMagic::SummonDreadlord(dreadlord) => println!("Dreadlord named {} with {} HP, {} ATK for {}s", {&dreadlord.name}, {&dreadlord.hp}, {&dreadlord.power}, {&dreadlord.duration}),
        }
    }
}

pub fn create_orc_faction(){
    let orc_magic = OrcMagic::SummonDreadlord(SummonDreadlord { name: String::from("Tichondrius"), hp: 100, power: 10, duration: 120 });

    let new_magic = orc_magic.new();

    let bloodlust = OrcMagic::Bloodlust(120);

    let chain_lightning = OrcMagic::ChainLightning { number_of_bounces: 5, power: 2 };

    orc_magic.print();
    new_magic.print();
    bloodlust.print();
    chain_lightning.print();

    let blade_master = OrcUnit::BladeMaster(String::from("Khargath Bladefist"), 50, 10);
    if let OrcUnit::BladeMaster(name, hp, pwr) = blade_master {
        println!("BladeMaster: {}, {} HP, {} ATK", &name, &hp, &pwr);
    }
}