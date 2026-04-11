use std::{borrow::Cow, str::FromStr};

use crate::Path;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pattern {
    string: Cow<'static, str>,
}

impl Pattern {
    pub fn new(s: impl Into<Cow<'static, str>>) -> Result<Self, ParsePatternError> {
        let s = s.into();
        if s.is_empty() {
            return Ok(Self { string: s });
        }
        if !s.starts_with("/") {
            return Err(ParsePatternError::LeadingSlash);
        }

        Ok(Self { string: s })
    }

    pub fn segments(&self) -> impl Iterator<Item = Segment<'_>> {
        self.string.split("/").skip(1).map(|segment| {
            if segment.starts_with(":") {
                return Segment::Dynamic(&segment[1..]);
            }
            return Segment::Static(segment);
        })
    }

    pub fn static_segments(&self) -> impl Iterator<Item = &str> {
        self.segments().filter_map(|segment| match segment {
            Segment::Static(inner) => Some(inner),
            _ => None,
        })
    }

    pub fn dynamic_segments(&self) -> impl Iterator<Item = &str> {
        self.segments().filter_map(|segment| match segment {
            Segment::Dynamic(inner) => Some(inner),
            _ => None,
        })
    }

    pub fn is_static(&self) -> bool {
        !self.is_dynamic()
    }

    pub fn is_dynamic(&self) -> bool {
        self.dynamic_segments().next().is_some()
    }

    pub fn as_path(&self) -> Option<Path> {
        self.is_static().then(|| Path::new(self.string.clone()))
    }
}

pub enum Segment<'a> {
    Static(&'a str),
    Dynamic(&'a str),
    CatchAll(&'a str),
}

pub enum ParsePatternError {
    /// Path pattern must be either empty or start with a leading slash.
    LeadingSlash,
}

impl FromStr for Pattern {
    type Err = ParsePatternError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Pattern::new(s.to_owned())
    }
}

impl TryFrom<&'static str> for Pattern {
    type Error = ParsePatternError;

    fn try_from(value: &'static str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for Pattern {
    type Error = ParsePatternError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
