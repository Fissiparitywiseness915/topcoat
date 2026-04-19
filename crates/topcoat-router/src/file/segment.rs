use std::{borrow::Cow, collections::HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SegmentKind {
    Static,
    Group,
    Param,
    CatchAll,
}

#[derive(Debug, Clone)]
pub struct Segment {
    file: &'static str,
    kind: Option<SegmentKind>,
    rename: Option<Cow<'static, str>>,
}

impl Segment {
    pub const fn new(
        file: &'static str,
        kind: Option<SegmentKind>,
        rename: Option<Cow<'static, str>>,
    ) -> Self {
        Self { file, kind, rename }
    }

    pub fn file(&self) -> &'static str {
        self.file
    }

    pub fn kind(&self) -> Option<&SegmentKind> {
        self.kind.as_ref()
    }

    pub fn rename(&self) -> Option<&str> {
        self.rename.as_deref()
    }
}

#[cfg(feature = "discover")]
inventory::collect!(Segment);

#[derive(Debug, Default, Clone)]
pub struct Segments {
    segments: HashMap<&'static str, Segment>,
}

impl Segments {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn register(&mut self, path: &'static str, segment: Segment) {
        if let Some(existing) = self.segments.insert(path, segment) {
            panic!("duplicate segment description in `{}`", existing.file())
        }
    }

    pub fn get(&self, path: &str) -> Option<&Segment> {
        self.segments.get(path)
    }

    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }
}
