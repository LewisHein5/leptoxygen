use std::collections::HashMap;

use leptos::{Attribute, IntoAttribute};

pub trait IntoAttributeName {
    fn to_attribute_name(&self) -> &'static str;
}

impl IntoAttributeName for &'static str {
    fn to_attribute_name(&self) -> &'static str {
        self
    }
}

#[derive(Debug, Clone)]
pub struct Attributes {
    pub map: HashMap<&'static str, Attribute>,
}

impl Attributes {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn insert(
        &mut self,
        k: impl IntoAttributeName,
        v: impl IntoAttribute,
    ) -> Option<Attribute> {
        self.map.insert(k.to_attribute_name(), v.into_attribute())
    }

    pub fn insert_entry<IntoAttrName: IntoAttributeName, IntoAttr: IntoAttribute>(
        &mut self,
        entry: impl Into<(IntoAttrName, IntoAttr)>,
    ) -> Option<Attribute> {
        let (k, v) = entry.into();
        self.map.insert(k.to_attribute_name(), v.into_attribute())
    }

    pub fn merge(
        &mut self,
        iter: impl IntoIterator<Item = (impl IntoAttributeName, impl IntoAttribute)>,
    ) {
        self.map.extend(
            iter.into_iter()
                .map(|(k, v)| (k.to_attribute_name(), v.into_attribute())),
        )
    }
}

impl IntoIterator for Attributes {
    type Item = (&'static str, Attribute);

    type IntoIter = std::collections::hash_map::IntoIter<&'static str, Attribute>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}
