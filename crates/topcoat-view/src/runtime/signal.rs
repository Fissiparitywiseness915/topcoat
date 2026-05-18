use serde::{Deserialize, Serialize, de::DeserializeOwned};
use uuid::Uuid;

use crate::runtime::{IntoViewParts, Unescaped, ViewPart};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SignalId(Uuid);

impl SignalId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for SignalId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Signal<T> {
    id: SignalId,
    value: T,
}

impl<T> Signal<T> {
    pub fn new(value: T) -> Self {
        Self {
            id: SignalId::new(),
            value,
        }
    }
}

pub struct SignalDeclaration<'a, T>(&'a Signal<T>);

impl<'a, T> SignalDeclaration<'a, T> {
    pub fn new(signal: &'a Signal<T>) -> Self {
        Self(signal)
    }
}

impl<T> IntoViewParts for SignalDeclaration<'_, T>
where
    T: Serialize + DeserializeOwned,
{
    fn into_view_parts(self) -> impl Iterator<Item = super::ViewPart> {
        [
            ViewPart::UnescapedStaticStr(Unescaped::new_unchecked("<!-- signal: ")),
            ViewPart::UnescapedString(Unescaped::new_unchecked(
                serde_json::to_string(&self.0).unwrap(),
            )),
            ViewPart::UnescapedStaticStr(Unescaped::new_unchecked(" -->")),
        ]
        .into_iter()
    }
}
