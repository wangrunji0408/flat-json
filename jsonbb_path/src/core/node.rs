//! Types representing nodes within a JSON object
use std::slice::Iter;

use jsonbb::ValueRef;
use serde::Serialize;

/// A list of nodes resulting from a JSONPath query
///
/// Each node within the list is a borrowed reference to the node in the original
/// [`serde_json::Value`] that was queried.
#[derive(Debug, Default, Eq, PartialEq, Serialize, Clone)]
pub struct NodeList<'a>(pub(crate) Vec<ValueRef<'a>>);

impl<'a> NodeList<'a> {
    /// Extract _at most_ one node from a [`NodeList`]
    ///
    /// This is intended for queries that are expected to optionally yield a single node.
    ///
    /// # Usage
    /// ```rust
    /// # use jsonbb::json;
    /// # use jsonbb_path::JsonPath;
    /// # use jsonbb_path::AtMostOneError;
    /// # fn main() -> Result<(), jsonbb_path::ParseError> {
    /// let value = json!({"foo": ["bar", "baz"]});
    /// # {
    /// let path = JsonPath::parse("$.foo[0]")?;
    /// let node = path.query(value.as_ref()).at_most_one().unwrap();
    /// assert_eq!(node, Some(json!("bar").as_ref()));
    /// # }
    /// # {
    /// let path = JsonPath::parse("$.foo.*")?;
    /// let error = path.query(value.as_ref()).at_most_one().unwrap_err();
    /// assert!(matches!(error, AtMostOneError(2)));
    /// # }
    /// # Ok(())
    /// # }
    /// ```
    pub fn at_most_one(&self) -> Result<Option<ValueRef<'a>>, AtMostOneError> {
        if self.0.is_empty() {
            Ok(None)
        } else if self.0.len() > 1 {
            Err(AtMostOneError(self.0.len()))
        } else {
            Ok(self.0.get(0).copied())
        }
    }

    /// Extract _exactly_ one node from a [`NodeList`]
    ///
    /// This is intended for queries that are expected to yield exactly one node.
    ///
    /// # Usage
    /// ```rust
    /// # use jsonbb::json;
    /// # use jsonbb_path::JsonPath;
    /// # use jsonbb_path::ExactlyOneError;
    /// # fn main() -> Result<(), jsonbb_path::ParseError> {
    /// let value = json!({"foo": ["bar", "baz"]});
    /// # {
    /// let path = JsonPath::parse("$.foo[0]")?;
    /// let node = path.query(value.as_ref()).exactly_one().unwrap();
    /// assert_eq!(node, "bar");
    /// # }
    /// # {
    /// let path = JsonPath::parse("$.foo.*")?;
    /// let error = path.query(value.as_ref()).exactly_one().unwrap_err();
    /// assert!(matches!(error, ExactlyOneError::MoreThanOne(2)));
    /// # }
    /// # Ok(())
    /// # }
    /// ```
    pub fn exactly_one(&self) -> Result<ValueRef<'a>, ExactlyOneError> {
        if self.0.is_empty() {
            Err(ExactlyOneError::Empty)
        } else if self.0.len() > 1 {
            Err(ExactlyOneError::MoreThanOne(self.0.len()))
        } else {
            Ok(*self.0.get(0).unwrap())
        }
    }

    /// Extract all nodes yielded by the query.
    ///
    /// This is intended for queries that are expected to yield zero or more nodes.
    ///
    /// # Usage
    /// ```rust
    /// # use jsonbb::json;
    /// # use jsonbb_path::JsonPath;
    /// # fn main() -> Result<(), jsonbb_path::ParseError> {
    /// let value = json!({"foo": ["bar", "baz"]});
    /// let path = JsonPath::parse("$.foo.*")?;
    /// let nodes = path.query(value.as_ref()).all();
    /// assert_eq!(nodes, vec!["bar", "baz"]);
    /// # Ok(())
    /// # }
    /// ```
    pub fn all(self) -> Vec<ValueRef<'a>> {
        self.0
    }

    /// Get the length of a [`NodeList`]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if a [NodeList] is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Get an iterator over a [`NodeList`]
    ///
    /// Note that [`NodeList`] also implements [`IntoIterator`].
    pub fn iter(&self) -> Iter<'_, ValueRef<'_>> {
        self.0.iter()
    }

    /// Returns the first node in the [`NodeList`], or `None` if it is empty
    ///
    /// # Usage
    /// ```rust
    /// # use jsonbb::json;
    /// # use jsonbb_path::JsonPath;
    /// # fn main() -> Result<(), jsonbb_path::ParseError> {
    /// let value = json!({"foo": ["bar", "baz"]});
    /// let path = JsonPath::parse("$.foo.*")?;
    /// let node = path.query(value.as_ref()).first();
    /// assert_eq!(node, Some(json!("bar").as_ref()));
    /// # Ok(())
    /// # }
    /// ```
    pub fn first(&self) -> Option<ValueRef<'a>> {
        self.0.first().copied()
    }

    /// Returns the last node in the [`NodeList`], or `None` if it is empty
    ///
    /// # Usage
    /// ```rust
    /// # use jsonbb::json;
    /// # use jsonbb_path::JsonPath;
    /// # fn main() -> Result<(), jsonbb_path::ParseError> {
    /// let value = json!({"foo": ["bar", "baz"]});
    /// let path = JsonPath::parse("$.foo.*")?;
    /// let node = path.query(value.as_ref()).last();
    /// assert_eq!(node, Some(json!("baz").as_ref()));
    /// # Ok(())
    /// # }
    /// ```
    pub fn last(&self) -> Option<ValueRef<'a>> {
        self.0.last().copied()
    }

    /// Returns the node at the given index in the [`NodeList`], or `None` if the given index is
    /// out of bounds.
    ///
    /// # Usage
    /// ```rust
    /// # use jsonbb::json;
    /// # use jsonbb_path::JsonPath;
    /// # fn main() -> Result<(), jsonbb_path::ParseError> {
    /// let value = json!({"foo": ["bar", "biz", "bop"]});
    /// let path = JsonPath::parse("$.foo.*")?;
    /// let nodes = path.query(value.as_ref());
    /// assert_eq!(nodes.get(1), Some(json!("biz").as_ref()));
    /// assert!(nodes.get(4).is_none());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get(&self, index: usize) -> Option<ValueRef<'a>> {
        self.0.get(index).copied()
    }

    /// Extract _at most_ one node from a [`NodeList`]
    ///
    /// This is intended for queries that are expected to optionally yield a single node.
    ///
    /// # Usage
    /// ```rust
    /// # use jsonbb::json;
    /// # use jsonbb_path::JsonPath;
    /// # fn main() -> Result<(), jsonbb_path::ParseError> {
    /// let value = json!({"foo": ["bar", "baz"]});
    /// # {
    /// let path = JsonPath::parse("$.foo[0]")?;
    /// let node = path.query(value.as_ref()).one();
    /// assert_eq!(node, Some(json!("bar").as_ref()));
    /// # }
    /// # {
    /// let path = JsonPath::parse("$.foo.*")?;
    /// let node = path.query(value.as_ref()).one();
    /// assert!(node.is_none());
    /// # }
    /// # Ok(())
    /// # }
    /// ```
    #[deprecated(
        since = "0.5.1",
        note = "it is recommended to use `at_most_one`, `exactly_one`, `first`, `last`, or `get` instead"
    )]
    pub fn one(self) -> Option<ValueRef<'a>> {
        if self.0.is_empty() || self.0.len() > 1 {
            None
        } else {
            self.0.get(0).copied()
        }
    }
}

/// Error produced when expecting no more than one node from a query
#[derive(Debug, thiserror::Error)]
#[error("nodelist expected to contain at most one entry, but instead contains {0} entries")]
pub struct AtMostOneError(pub usize);

/// Error produced when expecting exactly one node from a query
#[derive(Debug, thiserror::Error)]
pub enum ExactlyOneError {
    /// The query resulted in an empty [`NodeList`]
    #[error("nodelist expected to contain one entry, but is empty")]
    Empty,
    /// The query resulted in a [`NodeList`] containing more than one node
    #[error("nodelist expected to contain one entry, but instead contains {0} entries")]
    MoreThanOne(usize),
}

impl ExactlyOneError {
    /// Check that it is the `Empty` variant
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    /// Check that it is the `MoreThanOne` variant
    pub fn is_more_than_one(&self) -> bool {
        self.as_more_than_one().is_some()
    }

    /// Extract the number of nodes, if it was more than one, or `None` otherwise
    pub fn as_more_than_one(&self) -> Option<usize> {
        match self {
            ExactlyOneError::Empty => None,
            ExactlyOneError::MoreThanOne(u) => Some(*u),
        }
    }
}

impl<'a> From<Vec<ValueRef<'a>>> for NodeList<'a> {
    fn from(nodes: Vec<ValueRef<'a>>) -> Self {
        Self(nodes)
    }
}

impl<'a> IntoIterator for NodeList<'a> {
    type Item = ValueRef<'a>;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::NodeList;
    use crate::JsonPath;
    use jsonbb::{json, to_value};

    #[test]
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<NodeList>();
    }

    #[test]
    fn test_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<NodeList>();
    }

    #[test]
    fn test_serialize() {
        let v = json!([1, 2, 3, 4]);
        let q = JsonPath::parse("$.*")
            .expect("valid query")
            .query(v.as_ref());
        assert_eq!(to_value(q).expect("serialize"), v);
    }
}