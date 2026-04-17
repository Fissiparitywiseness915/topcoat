use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SegmentType {
    Static,
    Group,
    Param,
    CatchAll,
}

#[derive(Debug, Clone)]
pub struct Segment {
    file: &'static str,
    module_ident: &'static str,
    segment_type: Option<SegmentType>,
    rename: Option<Cow<'static, str>>,
}

impl Segment {
    pub const fn new(
        file: &'static str,
        module_ident: &'static str,
        segment_type: Option<SegmentType>,
        rename: Option<Cow<'static, str>>,
    ) -> Self {
        Self {
            file,
            module_ident,
            segment_type,
            rename,
        }
    }

    pub fn file(&self) -> &'static str {
        self.file
    }

    pub fn module_ident(&self) -> &'static str {
        self.module_ident
    }

    pub fn segment_type(&self) -> Option<&SegmentType> {
        self.segment_type.as_ref()
    }

    pub fn rename(&self) -> Option<&Cow<'static, str>> {
        self.rename.as_ref()
    }
}

#[cfg(feature = "discover")]
inventory::collect!(Segment);
