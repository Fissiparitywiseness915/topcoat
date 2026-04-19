use std::borrow::Cow;

use heck::ToKebabCase;

use crate::{FileLayout, FilePage, PathBuf, PathSegment, Router, Segment, SegmentKind, Segments};

#[doc(hidden)]
pub struct FileRouter {
    inner: Router,
    file_root: &'static str,
    segments: Segments,
}

impl FileRouter {
    pub fn new(file_root: &'static str) -> Self {
        Self {
            file_root: Self::strip_module_suffix(file_root),
            inner: Router::new(),
            segments: Segments::new(),
        }
    }

    fn strip_module_suffix(file: &'static str) -> &'static str {
        let path = file.strip_suffix(".rs").unwrap_or(file);
        let path = path.strip_suffix("/mod").unwrap_or(path);
        path.strip_suffix("\\mod").unwrap_or(path)
    }

    fn canonical_module_path(&self, file: &'static str) -> &'static str {
        let path = file
            .strip_prefix(self.file_root)
            .expect("file must be under file router's file root");
        Self::strip_module_suffix(path)
    }

    pub fn segment(mut self, segment: Segment) -> Self {
        assert!(
            self.inner.is_empty(),
            "`segment` must be called before registering any resource"
        );
        self.segments
            .register(self.canonical_module_path(segment.file()), segment);
        self
    }

    fn file_to_path(&self, file: &'static str) -> PathBuf {
        let module_path = self.canonical_module_path(file);
        let mut path_buf = PathBuf::new();
        let mut current_index = 0;

        // Iterate over the folder structure. At each module level, check if there is a matching
        // [`Segment`] for that path specified by the user that overrides the default behavior.
        for component in module_path.split(&['/', '\\']).skip(1) {
            current_index += component.len() + 1;
            let segment = self.segments.get(&module_path[..current_index]);

            // A module is a group segment if it starts with "_" or a static segment otherwise,
            // unless this is overridden by the user.
            let kind = match segment.and_then(|segment| segment.kind()) {
                Some(kind) => *kind,
                None => match component.starts_with("_") {
                    true => SegmentKind::Group,
                    false => SegmentKind::Static,
                },
            };
            // Static segments are converted to kebab-case, other modules names are left as is.
            // This can also be overridden by the user.
            let name = match segment.and_then(|segment| segment.rename()) {
                Some(rename) => Cow::Borrowed(rename),
                None => match kind {
                    SegmentKind::Static => Cow::Owned(component.to_kebab_case()),
                    _ => Cow::Borrowed(component),
                },
            };

            let path_segment = match kind {
                SegmentKind::Static => PathSegment::Static(&name),
                SegmentKind::Group => PathSegment::Group(&name),
                SegmentKind::Param => PathSegment::Param(&name),
                SegmentKind::CatchAll => PathSegment::CatchAll(&name),
            };

            path_buf += path_segment;
        }
        path_buf
    }

    pub fn page(mut self, page: FilePage) -> Self {
        let file = page.file();
        let page = page.into_page(Cow::Owned(self.file_to_path(file)));
        self.inner = self.inner.page(page);
        self
    }

    pub fn layout(mut self, layout: FileLayout) -> Self {
        let file = layout.file();
        let layout = layout.into_layout(Cow::Owned(self.file_to_path(file)));
        self.inner = self.inner.layout(layout);
        self
    }

    pub fn discover(mut self) -> Self {
        for segment in inventory::iter::<Segment>().cloned() {
            self = self.segment(segment);
        }
        for page in inventory::iter::<FilePage>().cloned() {
            self = self.page(page);
        }
        for layout in inventory::iter::<FileLayout>().cloned() {
            self = self.layout(layout);
        }
        self.inner = self.inner.discover();
        self
    }
}

impl From<FileRouter> for Router {
    fn from(value: FileRouter) -> Self {
        value.inner
    }
}
