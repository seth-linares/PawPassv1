use rand::{rngs::OsRng, seq::SliceRandom};
use serde::{Deserialize, Serialize};

const NUMBERS: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const SYMBOLS: &[char] = &['!', '@', '#', '$', '%', '^', '&', '*'];
const LOWERCASE_LETTERS: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

const UPPERCASE_LETTERS: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserSettings {
    pub password_length: u8,
    pub min_password_length: u8,
    pub use_num: bool,
    pub min_num: u8,
    pub use_symbol: bool,
    pub min_symbol: u8,
    pub use_lower: bool,
    pub use_upper: bool,
}

impl UserSettings {
    pub fn generate_password(&self) -> String {
        

        let mut rng = OsRng;

        let mut character_pool: Vec<char> = Vec::new();
        if self.use_num {
            character_pool.extend(NUMBERS);
        }
        if self.use_symbol {
            character_pool.extend(SYMBOLS);
        }
        if self.use_lower {
            character_pool.extend(LOWERCASE_LETTERS);
        }
        if self.use_upper {
            character_pool.extend(UPPERCASE_LETTERS);
        }

        let mut password = Vec::with_capacity(self.password_length as usize);

        // Add at least the minimum required characters of each type
        if self.use_num {
            password.extend(NUMBERS.choose_multiple(&mut rng, self.min_num as usize));
        }
        if self.use_symbol {
            password.extend(SYMBOLS.choose_multiple(&mut rng, self.min_symbol as usize));
        }
        if self.use_lower {
            password.push(LOWERCASE_LETTERS.choose(&mut rng).unwrap_or(&'a'));
        }
        if self.use_upper {
            password.push(UPPERCASE_LETTERS.choose(&mut rng).unwrap_or(&'A'));
        }

        // Fill the rest of the password length with random characters from the pool
        while password.len() < self.password_length as usize {
            password.push(character_pool.choose(&mut rng).unwrap());
        }

        password.shuffle(&mut rng);
        password.into_iter().collect()
    }

}

impl Default for UserSettings {
    fn default() -> Self {
        UserSettings {
            password_length: 14,
            min_password_length: 10,
            use_num: true,
            min_num: 2,
            use_symbol: true,
            min_symbol: 2,
            use_lower: true,
            use_upper: true,
        }
    }
}

impl PartialEq for UserSettings {
    fn eq(&self, other: &Self) -> bool {
        self.password_length == other.password_length
            && self.min_password_length == other.min_password_length
            && self.use_num == other.use_num
            && self.min_num == other.min_num
            && self.use_symbol == other.use_symbol
            && self.min_symbol == other.min_symbol
            && self.use_lower == other.use_lower
            && self.use_upper == other.use_upper
    }
}
impl Eq for UserSettings {}