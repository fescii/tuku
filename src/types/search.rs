// src/types/search.rs

// Search types
#[derive(Debug, PartialEq)]
pub enum Search {
  TsVector,
  TsQuery,
}

// impl PartialEq for Search
impl PartialEq for Search {
  fn eq(&self, other: &Self) -> bool {
    match self {
      Search::TsVector => {
        if let Search::TsVector = other {
          true
        } else {
          false
        }
      },
      Search::TsQuery => {
        if let Search::TsQuery = other {
          true
        } else {
          false
        }
      },
    }
  }
}

// impl Search
impl Search {
  pub fn to_string(&self) -> String {
    self.to_string()
  }
}

// impl Display for Search
impl std::fmt::Display for Search {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Search::TsVector => write!(f, "TSVECTOR"),
      Search::TsQuery => write!(f, "TSQUERY"),
    }
  }
}