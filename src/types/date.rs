// src/types/date.rs

// DateTime types
#[derive(Debug, PartialEq)]
pub enum DateTime {
  Date,
  Time,
  TimeWithTimeZone,
  Timestamp,
  TimestampWithTimeZone,
  Interval,
}

// impl PartialEq for DateTime
impl PartialEq for DateTime {
  fn eq(&self, other: &Self) -> bool {
    match self {
      DateTime::Date => {
        if let DateTime::Date = other {
          true
        } else {
          false
        }
      },
      DateTime::Time => {
        if let DateTime::Time = other {
          true
        } else {
          false
        }
      },
      DateTime::TimeWithTimeZone => {
        if let DateTime::TimeWithTimeZone = other {
          true
        } else {
          false
        }
      },
      DateTime::Timestamp => {
        if let DateTime::Timestamp = other {
          true
        } else {
          false
        }
      },
      DateTime::TimestampWithTimeZone => {
        if let DateTime::TimestampWithTimeZone = other {
          true
        } else {
          false
        }
      },
      DateTime::Interval => {
        if let DateTime::Interval = other {
          true
        } else {
          false
        }
      },
    }
  }
}

// impl DateTime
impl DateTime {
  pub fn to_string(&self) -> String {
    self.to_string()
  }
}

// impl Display for DateTime
impl std::fmt::Display for DateTime {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      DateTime::Date => write!(f, "DATE"),
      DateTime::Time => write!(f, "TIME"),
      DateTime::TimeWithTimeZone => write!(f, "TIME WITH TIME ZONE"),
      DateTime::Timestamp => write!(f, "TIMESTAMP"),
      DateTime::TimestampWithTimeZone => write!(f, "TIMESTAMP WITH TIME ZONE"),
      DateTime::Interval => write!(f, "INTERVAL"),
    }
  }
}

// impl date_to_sql for our DateTime enum
impl date_to_sql for DateTime {
  fn date_to_sql(&self) -> &str {
    match self {
      DateTime::Date => "DATE",
      DateTime::Time => "TIME",
      DateTime::TimeWithTimeZone => "TIME WITH TIME ZONE",
      DateTime::Timestamp => "TIMESTAMP",
      DateTime::TimestampWithTimeZone => "TIMESTAMP WITH TIME ZONE",
      DateTime::Interval => "INTERVAL",
    }
  }
}

// impl FromStr for DateTime
impl std::str::FromStr for DateTime {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "DATE" => Ok(DateTime::Date),
      "TIME" => Ok(DateTime::Time),
      "TIME WITH TIME ZONE" => Ok(DateTime::TimeWithTimeZone),
      "TIMESTAMP" => Ok(DateTime::Timestamp),
      "TIMESTAMP WITH TIME ZONE" => Ok(DateTime::TimestampWithTimeZone),
      "INTERVAL" => Ok(DateTime::Interval),
      _ => Err("Unknown DateTime type".to_string()),
    }
  }
}

// fn to generate psql function for updating date time when a row is updated or inserted using triggers
pub fn generate_trigger_function(table_name: &str, columns: &Vec<Column>) -> String {
  let mut sql = String::new();
  sql.push_str(&format!("CREATE OR REPLACE FUNCTION update_modified_column()\n"));
  sql.push_str(&format!("RETURNS TRIGGER AS $$\n"));
  sql.push_str(&format!("BEGIN\n"));
  sql.push_str(&format!("  NEW.modified = now();\n"));
  sql.push_str(&format!("  RETURN NEW;\n"));
  sql.push_str(&format!("END;\n"));
  sql.push_str(&format!("$$ language 'plpgsql';\n"));
  sql.push_str(&format!("\n"));
  sql.push_str(&format!("CREATE TRIGGER update_{} BEFORE UPDATE ON {} FOR EACH ROW EXECUTE FUNCTION update_modified_column();\n", table_name, table_name));
  sql
}