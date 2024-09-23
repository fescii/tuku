// src/types/enumerated.rs

// Enumerated types
#[derive(Debug, PartialEq)]
pub enum Enumerated {
  Enum,
}

// impl PartialEq for Enumerated
impl PartialEq for Enumerated {
  fn eq(&self, other: &Self) -> bool {
    match self {
      Enumerated::Enum => {
        if let Enumerated::Enum = other {
          true
        } else {
          false
        }
      },
    }
  }
}

// impl Enumerated
impl Enumerated {
  pub fn to_string(&self) -> String {
    self.to_string()
  }
}

// impl Display for Enumerated
impl std::fmt::Display for Enumerated {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Enumerated::Enum => write!(f, "ENUM"),
    }
  }
}