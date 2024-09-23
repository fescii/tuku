// src/types/geometry.rs

// Geometry types
#[derive(Debug, PartialEq)]
pub enum Geometry {
  Point,
  LineString,
  Polygon,
  MultiPoint,
  MultiLineString,
  MultiPolygon,
  GeometryCollection,
}

// impl PartialEq for Geometry
impl PartialEq for Geometry {
  fn eq(&self, other: &Self) -> bool {
    match self {
      Geometry::Point => {
        if let Geometry::Point = other {
          true
        } else {
          false
        }
      },
      Geometry::LineString => {
        if let Geometry::LineString = other {
          true
        } else {
          false
        }
      },
      Geometry::Polygon => {
        if let Geometry::Polygon = other {
          true
        } else {
          false
        }
      },
      Geometry::MultiPoint => {
        if let Geometry::MultiPoint = other {
          true
        } else {
          false
        }
      },
      Geometry::MultiLineString => {
        if let Geometry::MultiLineString = other {
          true
        } else {
          false
        }
      },
      Geometry::MultiPolygon => {
        if let Geometry::MultiPolygon = other {
          true
        } else {
          false
        }
      },
      Geometry::GeometryCollection => {
        if let Geometry::GeometryCollection = other {
          true
        } else {
          false
        }
      },
    }
  }
}

// impl Geometry
impl Geometry {
  pub fn to_string(&self) -> String {
    self.to_string()
  }
}

// impl Display for Geometry
impl std::fmt::Display for Geometry {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Geometry::Point => write!(f, "POINT"),
      Geometry::LineString => write!(f, "LINESTRING"),
      Geometry::Polygon => write!(f, "POLYGON"),
      Geometry::MultiPoint => write!(f, "MULTIPOINT"),
      Geometry::MultiLineString => write!(f, "MULTILINESTRING"),
      Geometry::MultiPolygon => write!(f, "MULTIPOLYGON"),
      Geometry::GeometryCollection => write!(f, "GEOMETRYCOLLECTION"),
    }
  }
}

// impl Geometry
impl Geometry {
  pub fn to_string(&self) -> String {
    self.to_string()
  }
}

// impl Display for Geometry
impl std::fmt::Display for Geometry {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Geometry::Point => write!(f, "POINT"),
      Geometry::LineString => write!(f, "LINESTRING"),
      Geometry::Polygon => write!(f, "POLYGON"),
      Geometry::MultiPoint => write!(f, "MULTIPOINT"),
      Geometry::MultiLineString => write!(f, "MULTILINESTRING"),
      Geometry::MultiPolygon => write!(f, "MULTIPOLYGON"),
      Geometry::GeometryCollection => write!(f, "GEOMETRYCOLLECTION"),
    }
  }
}