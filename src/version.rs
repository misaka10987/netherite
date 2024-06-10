use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Clone, Copy)]
pub struct SemVer(pub i16, pub i16, pub i16);

impl PartialEq for SemVer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

impl Eq for SemVer {}

impl PartialOrd for SemVer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.0.partial_cmp(&other.0) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.1.partial_cmp(&other.1) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.2.partial_cmp(&other.2)
    }
}

impl Ord for SemVer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Display for SemVer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let SemVer(x, y, z) = self;
        write!(f, "{x}.{y}.{z}")
    }
}

impl Debug for SemVer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SemVer").field(&format!("{self}")).finish()
    }
}

impl FromStr for SemVer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s.split('.').collect();
        let len = s.len();
        if len != 3 {
            return Err(format!("invalid length of {len}, 3 expected"));
        }
        let mut err = false;
        let r: Vec<i16> = s
            .into_iter()
            .map(|s| match s.parse::<i16>() {
                Ok(x) => x,
                Err(_) => {
                    err = true;
                    0
                }
            })
            .collect();
        if err {
            return Err(format!("invalid syntax, int.int.int expected"));
        }
        Ok(SemVer(r[0], r[1], r[2]))
    }
}

impl Serialize for SemVer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{self}"))
    }
}

struct SemVerVisitor;

impl<'de> Visitor<'de> for SemVerVisitor {
    type Value = SemVer;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a semantic version number (e.g. 11.45.14)")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let s: Vec<&str> = v.split('.').collect();
        let len = s.len();
        if len != 3 {
            return Err(E::invalid_length(len, &self));
        }
        let mut err = false;
        let r: Vec<i16> = s
            .into_iter()
            .map(|s| match s.parse::<i16>() {
                Ok(x) => x,
                Err(_) => {
                    err = true;
                    0
                }
            })
            .collect();
        if err {
            return Err(E::invalid_value(serde::de::Unexpected::Str(v), &self));
        }
        Ok(SemVer(r[0], r[1], r[2]))
    }
}

impl<'de> Deserialize<'de> for SemVer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(SemVerVisitor)
    }
}

impl SemVer {
    pub fn within(&self, rng: ReqVer) -> bool {
        let ge = match rng.lo {
            Some(lo) => self >= &lo,
            None => true,
        };
        let le = match rng.hi {
            Some(hi) => self <= &hi,
            None => true,
        };
        ge && le
    }
}

#[derive(Clone, Copy)]
pub struct ReqVer {
    pub lo: Option<SemVer>,
    pub hi: Option<SemVer>,
}

impl ReqVer {
    pub const NONE: Self = Self { lo: None, hi: None };
    pub const fn ge(v: SemVer) -> Self {
        Self {
            lo: Some(v),
            hi: None,
        }
    }
    pub const fn le(v: SemVer) -> Self {
        Self {
            lo: None,
            hi: Some(v),
        }
    }
}

impl Display for ReqVer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let left = match self.lo {
            Some(v) => format!("[{v},"),
            None => "(,".to_string(),
        };
        let right = match self.hi {
            Some(v) => format!("{v}]"),
            None => ")".to_string(),
        };
        let s = left + &right;
        write!(f, "{s}")
    }
}
