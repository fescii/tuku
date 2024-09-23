// src/types/character.rs

// Character types
#[derive(Debug, PartialEq)]
pub enum Character {
  Char,
  Varchar,
  Text,
  BpChar,
}

// impl PartialEq for Character
impl PartialEq for Character {
  fn eq(&self, other: &Self) -> bool {
    match self {
      Character::Char => {
        if let Character::Char = other {
          true
        } else {
          false
        }
      },
      Character::Varchar => {
        if let Character::Varchar = other {
          true
        } else {
          false
        }
      },
      Character::Text => {
        if let Character::Text = other {
          true
        } else {
          false
        }
      },
      Character::BpChar => {
        if let Character::BpChar = other {
          true
        } else {
          false
        }
      },
    }
  }
}

// impl Character
impl Character {
  pub fn to_string(&self) -> String {
    self.to_string()
  }
}

// impl Display for Character
impl std::fmt::Display for Character {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Character::Char => write!(f, "CHARACTER"),
      Character::Varchar => write!(f, "VARCHAR"),
      Character::Text => write!(f, "TEXT"),
      Character::BpChar => write!(f, "BPCHAR"),
    }
  }
}