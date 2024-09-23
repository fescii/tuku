// src/types/numeric.rs

// Numeric types
/*
 Struct to represent all numeric types in postgresql
 */
#[derive(Debug, Clone)]
pub enum Numeric {
  SmallInt,
  Integer,
  BigInt,
  Decimal,
  Numeric,
  Real,
  DoublePrecision,
  SmallSerial,
  Serial,
  BigSerial,
}

// impl Display for Numeric 
impl std::fmt::Display for Numeric {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Numeric::SmallInt => write!(f, "SMALLINT"),
      Numeric::Integer => write!(f, "INTEGER"),
      Numeric::BigInt => write!(f, "BIGINT"),
      Numeric::Decimal => write!(f, "DECIMAL"),
      Numeric::Numeric => write!(f, "NUMERIC"),
      Numeric::Real => write!(f, "REAL"),
      Numeric::DoublePrecision => write!(f, "DOUBLE PRECISION"),
      Numeric::SmallSerial => write!(f, "SMALLSERIAL"),
      Numeric::Serial => write!(f, "SERIAL"),
      Numeric::BigSerial => write!(f, "BIGSERIAL"),
    }
  }
}

// impl FromStr for Numeric
impl std::str::FromStr for Numeric {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "SMALLINT" => Ok(Numeric::SmallInt),
      "INTEGER" => Ok(Numeric::Integer),
      "BIGINT" => Ok(Numeric::BigInt),
      "DECIMAL" => Ok(Numeric::Decimal),
      "NUMERIC" => Ok(Numeric::Numeric),
      "REAL" => Ok(Numeric::Real),
      "DOUBLE PRECISION" => Ok(Numeric::DoublePrecision),
      "SMALLSERIAL" => Ok(Numeric::SmallSerial),
      "SERIAL" => Ok(Numeric::Serial),
      "BIGSERIAL" => Ok(Numeric::BigSerial),
      _ => Err("Unknown numeric type".to_string()),
    }
  }
}

// impl PartialEq for Numeric
impl PartialEq for Numeric {
  fn eq(&self, other: &Self) -> bool {
    match self {
      Numeric::SmallInt => {
        if let Numeric::SmallInt = other {
          true
        } else {
          false
        }
      },
      Numeric::Integer => {
        if let Numeric::Integer = other {
          true
        } else {
          false
        }
      },
      Numeric::BigInt => {
        if let Numeric::BigInt = other {
          true
        } else {
          false
        }
      },
      Numeric::Decimal => {
        if let Numeric::Decimal = other {
          true
        } else {
          false
        }
      },
      Numeric::Numeric => {
        if let Numeric::Numeric = other {
          true
        } else {
          false
        }
      },
      Numeric::Real => {
        if let Numeric::Real = other {
          true
        } else {
          false
        }
      },
      Numeric::DoublePrecision => {
        if let Numeric::DoublePrecision = other {
          true
        } else {
          false
        }
      },
      Numeric::SmallSerial => {
        if let Numeric::SmallSerial = other {
          true
        } else {
          false
        }
      },
      Numeric::Serial => {
        if let Numeric::Serial = other {
          true
        } else {
          false
        }
      },
      Numeric::BigSerial => {
        if let Numeric::BigSerial = other {
          true
        } else {
          false
        }
      },
    }
  }
}

// impl Numeric
impl Numeric {
  pub fn to_string(&self) -> String {
    self.to_string()
  }
}


// define monetary types
/*
 Struct to represent all monetary types in postgresql
 */
#[derive(Debug, Clone)]
pub enum Monetary {
  Money,
}

// impl Display for Monetary
impl std::fmt::Display for Monetary {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Monetary::Money => write!(f, "MONEY"),
    }
  }
}

// impl FromStr for Monetary
impl std::str::FromStr for Monetary {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "MONEY" => Ok(Monetary::Money),
      _ => Err("Unknown monetary type".to_string()),
    }
  }
}

// impl PartialEq for Monetary
impl PartialEq for Monetary {
  fn eq(&self, other: &Self) -> bool {
    match self {
      Monetary::Money => {
        if let Monetary::Money = other {
          true
        } else {
          false
        }
      },
    }
  }
}

// impl Monetary
impl Monetary {
  pub fn to_string(&self) -> String {
    self.to_string()
  }
}