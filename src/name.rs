use macroquad::rand::ChooseRandom;

/// Generate name
pub struct NameGenerator;

impl NameGenerator {
    pub fn generate() -> String {
        let name_vec = NAMES_LIST.to_vec();
        let first_name = name_vec.choose().unwrap().to_owned();
        let last_name = name_vec.choose().unwrap().to_owned();
        let string = format!("{} {}", first_name, last_name);
        string
    }
}

const NAMES_LIST: [&str; 100] = [
    "Max",
    "Charlie",
    "Cooper",
    "Milo",
    "Bear",
    "Tucker",
    "Zeus",
    "Duke",
    "Leo",
    "Louie",
    "Jax",
    "Winston",
    "Oliver",
    "Jack",
    "McWoof",
    "Abrahen",
    "Eggatha",
    "Chick",
    "Yolko",
    "Jopeep",
    "Tootsie",
    "Hen",
    "Henburn",
    "Henobi",
    "Hensworth",
    "Beak",
    "Beekham",
    "Flappy",
    "Layin",
    "Fiesta",
    "Clucker",
    "Monella",
    "Aurora",
    "Cinderella",
    "Grumpy",
    "Chewbarka",
    "Kanye",
    "Lick",
    "Jude",
    "Winnie",
    "Andy",
    "Dumbledog",
    "Growling",
    "Pawter",
    "Smith",
    "Bones",
    "Bark",
    "Paw",
    "Ryan",
    "Woofie",
    "Sarah",
    "Karl",
    "Vera",
    "Nine",
    "Olivia",
    "Ozzy",
    "Isaboo",
    "Daisy",
    "Foxy",
    "Boo",
    "Poppy",
    "Furry",
    "Lola",
    "Sherriff",
    "Winston",
    "Flossie",
    "Mighty",
    "Sunny",
    "Noodles",
    "Sushi",
    "Cappuccino",
    "Meatloaf",
    "Frankie",
    "Freddie",
    "Jim",
    "Norman",
    "Carry",
    "Esmeralda",
    "Nancy",
    "Aster",
    "Calla",
    "Caspia",
    "Billy",
    "Buttons",
    "Chrysanthemum",
    "Myrtle",
    "Cosmos",
    "Daffodil",
    "Dahlia",
    "Freesia",
    "Ginger",
    "Iris",
    "Gardenia",
    "Sweet",
    "Pea",
    "Zinnia",
    "Apollo",
    "Hypnos",
    "Nike",
    "Olympia",
];
