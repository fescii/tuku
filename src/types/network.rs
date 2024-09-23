// src/types/network.rs

// Network types
#[derive(Debug, PartialEq)]
pub enum Network {
  Cidr,
  Inet,
  MacAddr,
  MacAddr8,
}

// impl PartialEq for Network
impl PartialEq for Network {
  fn eq(&self, other: &Self) -> bool {
    match self {
      Network::Cidr => {
        if let Network::Cidr = other {
          true
        } else {
          false
        }
      },
      Network::Inet => {
        if let Network::Inet = other {
          true
        } else {
          false
        }
      },
      Network::MacAddr => {
        if let Network::MacAddr = other {
          true
        } else {
          false
        }
      },
      Network::MacAddr8 => {
        if let Network::MacAddr8 = other {
          true
        } else {
          false
        }
      },
    }
  }
}

// impl Network
impl Network {
  pub fn to_string(&self) -> String {
    self.to_string()
  }
}

// impl Display for Network
impl std::fmt::Display for Network {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Network::Cidr => write!(f, "CIDR"),
      Network::Inet => write!(f, "INET"),
      Network::MacAddr => write!(f, "MACADDR"),
      Network::MacAddr8 => write!(f, "MACADDR8"),
    }
  }
}

// impl Network
impl Network {
  pub fn to_string(&self) -> String {
    self.to_string()
  }
}

// impl Display for Network
impl std::fmt::Display for Network {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Network::Cidr => write!(f, "CIDR"),
      Network::Inet => write!(f, "INET"),
      Network::MacAddr => write!(f, "MACADDR"),
      Network::MacAddr8 => write!(f, "MACADDR8"),
    }
  }
}