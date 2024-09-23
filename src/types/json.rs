// src/types/json.rs

// JSON types
#[derive(Debug, PartialEq)]
pub enum Json {
  Json,
  Jsonb,
}

// Array types
#[derive(Debug, PartialEq)]
pub enum Array {
  Array,
}

// impl PartialEq for Json
impl PartialEq for Json {
  fn eq(&self, other: &Self) -> bool {
    match self {
      Json::Json => {
        if let Json::Json = other {
          true
        } else {
          false
        }
      },
      Json::Jsonb => {
        if let Json::Jsonb = other {
          true
        } else {
          false
        }
      },
    }
  }
}

// impl Json
impl Json {
  pub fn to_string(&self) -> String {
    self.to_string()
  }
}

// impl Display for Json
impl std::fmt::Display for Json {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Json::Json => write!(f, "JSON"),
      Json::Jsonb => write!(f, "JSONB"),
    }
  }
}

// impl PartialEq for Array
impl PartialEq for Array {
  fn eq(&self, other: &Self) -> bool {
    match self {
      Array::Array => {
        if let Array::Array = other {
          true
        } else {
          false
        }
      },
    }
  }
}

// impl Array
impl Array {
  pub fn to_string(&self) -> String {
    self.to_string()
  }
}

// impl Display for Array
impl std::fmt::Display for Array {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Array::Array => write!(f, "ARRAY"),
    }
  }
}

// impl PartialEq for Json
impl PartialEq for Json {
  fn eq(&self, other: &Self) -> bool {
    match self {
      Json::Json => {
        if let Json::Json = other {
          true
        } else {
          false
        }
      },
      Json::Jsonb => {
        if let Json::Jsonb = other {
          true
        } else {
          false
        }
      },
    }
  }
}