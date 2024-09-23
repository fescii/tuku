// src/types/uuid.rs

// UUID types
#[derive(Debug, PartialEq)]
pub enum Uuid {
  Uuid,
}

// impl PartialEq for Uuid
impl PartialEq for Uuid {
  fn eq(&self, other: &Self) -> bool {
    match self {
      Uuid::Uuid => {
        if let Uuid::Uuid = other {
          true
        } else {
          false
        }
      },
    }
  }
}

// impl Uuid
impl Uuid {
  pub fn to_string(&self) -> String {
    self.to_string()
  }
}

// impl Display for Uuid
impl std::fmt::Display for Uuid {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Uuid::Uuid => write!(f, "UUID"),
    }
  }
}