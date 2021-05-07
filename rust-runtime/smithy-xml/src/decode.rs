/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

use std::borrow::Cow;
use std::convert::TryFrom;
use thiserror::Error;
use xmlparser::{ElementEnd, Token, Tokenizer};

pub type Depth = usize;

#[derive(Eq, PartialEq, Debug, Error)]
pub enum XmlError {
    #[error("XML Parse Error")]
    InvalidXml(#[from] xmlparser::Error),
    #[error("Other: {msg}")]
    Other { msg: &'static str },
    #[error("Expect a data element. Found: {found}")]
    ExpectedData { found: String },
    #[error("Custom: {0}")]
    Custom(String),
}

#[derive(PartialEq, Debug)]
pub struct Name<'a> {
    pub prefix: Cow<'a, str>,
    pub local: Cow<'a, str>,
}

impl Name<'_> {
    pub fn matches(&self, tag_name: &str) -> bool {
        let split = tag_name.find(':');
        match split {
            None => tag_name == self.local.as_ref(),
            Some(idx) => {
                let (prefix, local) = tag_name.split_at(idx);
                let local = &local[1..];
                self.local.as_ref() == local && self.prefix.as_ref() == prefix
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Attr<'a> {
    name: Name<'a>,
    value: Cow<'a, str>,
}

#[derive(Debug, PartialEq)]
pub struct StartEl<'a> {
    name: Name<'a>,
    attributes: Vec<Attr<'a>>,
    closed: bool,
    depth: Depth,
}

/// Xml Start Element
///
/// <a:b   c="d">
///  ^^^   ^^^^^
///  name  attributes
impl<'a> StartEl<'a> {
    pub fn closed(local: &'a str, prefix: &'a str, depth: Depth) -> Self {
        let mut s = Self::new(local, prefix, depth);
        s.closed = true;
        s
    }

    pub fn depth(&self) -> Depth {
        self.depth
    }

    pub fn new(local: &'a str, prefix: &'a str, depth: Depth) -> Self {
        Self {
            name: Name {
                local: local.into(),
                prefix: prefix.into(),
            },
            attributes: vec![],
            closed: false,
            depth,
        }
    }

    /// Retrieve an attribute with a given key
    ///
    /// key `prefix:local` combined as a str, joined by a `:`
    pub fn attr<'b>(&'b self, key: &'b str) -> Option<&'b str> {
        self.attributes
            .iter()
            .find(|attr| attr.name.matches(key))
            .map(|attr| attr.value.as_ref())
    }

    /// Returns whether this `StartEl` matches a given name
    /// in `prefix:local` form.
    pub fn matches(&self, pat: &str) -> bool {
        self.name.matches(pat)
    }

    /// Local component of this element's name
    ///
    /// ```xml
    /// <foo:bar>
    ///      ^^^
    /// ```
    pub fn local(&self) -> &str {
        self.name.local.as_ref()
    }

    /// Prefix component of this elements name (or empty string)
    /// ```xml
    /// <foo:bar>
    ///  ^^^
    /// ```
    pub fn prefix(&self) -> &str {
        self.name.local.as_ref()
    }
}

impl StartEl<'_> {
    /// Returns if a given element closes this tag
    fn end_el(&self, el: ElementEnd, depth: Depth) -> bool {
        if depth != self.depth {
            return false;
        }
        match el {
            ElementEnd::Open => false,
            ElementEnd::Close(prefix, local) => {
                prefix.as_str() == self.name.prefix && local.as_str() == self.name.local
            }
            ElementEnd::Empty => false,
        }
    }
}

/// Xml Document abstraction
///
/// This document wraps a lazy tokenizer. Constructing a document is essentially free.
pub struct Document<'a> {
    tokenizer: Tokenizer<'a>,
    depth: Depth,
}

impl<'a> TryFrom<&'a [u8]> for Document<'a> {
    type Error = XmlError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(Document::new(std::str::from_utf8(value).map_err(|_| {
            XmlError::Other {
                msg: "invalid utf8",
            }
        })?))
    }
}

impl<'inp> Document<'inp> {
    pub fn new(doc: &'inp str) -> Self {
        Document {
            tokenizer: Tokenizer::from(doc),
            depth: 0,
        }
    }

    /// "Depth first" iterator
    ///
    /// Unlike [`next_tag()`](ScopedDecoder::next_tag), this method returns the next
    /// start element regardless of depth. This is useful to give a pointer into the middle
    /// of a document to start reading.
    ///
    /// ```xml
    /// <Response> <-- first call returns this:
    ///    <A> <-- next call
    ///      <Nested /> <-- next call returns this
    ///      <MoreNested>hello</MoreNested> <-- then this:
    ///    </A>
    ///    <B/> <-- second call to next_tag returns this
    /// </Response>
    /// ```
    pub fn next_start_element<'a>(&'a mut self) -> Option<StartEl<'inp>> {
        next_start_element(self)
    }

    /// A scoped reader for the entire document
    pub fn root_element<'a>(&'a mut self) -> Result<ScopedDecoder<'inp, 'a>, XmlError> {
        let start_el = self.next_start_element().ok_or(XmlError::Other {
            msg: "No root element",
        })?;
        Ok(ScopedDecoder {
            doc: self,
            start_el,
            terminated: false,
        })
    }

    /// A scoped reader for a specific tag
    ///
    /// This method is necessary for when you need to return a ScopedDecoder from a function
    /// since normally the stacked-ownership that `next_tag()` uses would prevent returning a reference
    /// to a field owned by the current function
    pub fn scoped_to<'a>(&'a mut self, start_el: StartEl<'inp>) -> ScopedDecoder<'inp, 'a> {
        ScopedDecoder {
            doc: self,
            start_el,
            terminated: false,
        }
    }
}

/// Depth tracking iterator
///
/// ```xml
/// <a> <- startel depth 0
///   <b> <- startel depth 1
///     <c> <- startel depth 2
///     </c> <- endel depth 2
///   </b> <- endel depth 1
/// </a> <- endel depth 0
/// ```
impl<'inp> Iterator for Document<'inp> {
    type Item = Result<(Token<'inp>, Depth), XmlError>;
    fn next<'a>(&'a mut self) -> Option<Result<(Token<'inp>, Depth), XmlError>> {
        let tok = self.tokenizer.next()?;
        let tok = match tok {
            Err(e) => return Some(Err(e.into())),
            Ok(tok) => tok,
        };
        // depth bookkeeping
        match tok {
            Token::ElementEnd {
                end: ElementEnd::Close(_, _),
                ..
            } => {
                self.depth -= 1;
            }
            Token::ElementEnd {
                end: ElementEnd::Empty,
                ..
            } => self.depth -= 1,
            t @ Token::ElementStart { .. } => {
                self.depth += 1;
                // We want the startel and endel to have the same depth, but after the opener,
                // the parser will be at depth 1. Return the previous depth:
                return Some(Ok((t, self.depth - 1)));
            }
            _ => {}
        }
        Some(Ok((tok, self.depth)))
    }
}

/// XmlTag Abstraction
///
/// ScopedDecoder represents a tag-scoped view into an XML document. Methods
/// on `ScopedDecoder` return `None` when the current tag has been exhausted.
pub struct ScopedDecoder<'inp, 'a> {
    doc: &'a mut Document<'inp>,
    start_el: StartEl<'inp>,
    terminated: bool,
}

/// When a scoped decoder is dropped, its entire scope is consumed so that the
/// next read begins at the next tag at the same depth.
impl Drop for ScopedDecoder<'_, '_> {
    fn drop(&mut self) {
        for _ in self {}
    }
}

impl<'inp> ScopedDecoder<'inp, '_> {
    /// The start element for this scope
    pub fn start_el<'a>(&'a self) -> &'a StartEl<'inp> {
        &self.start_el
    }

    /// Returns the next top-level tag in this scope
    /// The returned reader will fully read the tag during its lifetime. If it is dropped without
    /// the data being read, the reader will be advanced until the matching close tag. If you read
    /// an element with `next_tag()` and you want to ignore it, simply drop the resulting `ScopeDecoder`.
    ///
    /// ```xml
    /// <Response> <-- scoped reader on this tag
    ///    <A> <-- first call to next_tag returns this
    ///      <Nested /> <-- to get inner data, call `next_tag` on the returned decoder for `A`
    ///      <MoreNested>hello</MoreNested>
    ///    </A>
    ///    <B/> <-- second call to next_tag returns this
    /// </Response>
    /// ```
    pub fn next_tag<'a>(&'a mut self) -> Option<ScopedDecoder<'inp, 'a>> {
        let next_tag = next_start_element(self)?;
        Some(self.nested_decoder(next_tag))
    }

    fn nested_decoder<'a>(&'a mut self, start_el: StartEl<'inp>) -> ScopedDecoder<'inp, 'a> {
        ScopedDecoder {
            doc: &mut self.doc,
            start_el,
            terminated: false,
        }
    }
}

impl<'inp, 'a> Iterator for ScopedDecoder<'inp, 'a> {
    type Item = Result<(Token<'inp>, Depth), XmlError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start_el.closed {
            self.terminated = true;
        }
        if self.terminated {
            return None;
        }
        let (tok, depth) = match self.doc.next() {
            Some(Ok((tok, depth))) => (tok, depth),
            other => return other,
        };

        match tok {
            Token::ElementEnd { end, .. } if self.start_el.end_el(end, depth) => {
                self.terminated = true;
                return None;
            }
            _ => {}
        }
        Some(Ok((tok, depth)))
    }
}

fn unescape(s: &str) -> Cow<str> {
    s.into()
}

/// Load the next start element out of a depth-tagged token iterator
fn next_start_element<'a, 'inp>(
    tokens: &'a mut impl Iterator<Item = Result<(Token<'inp>, Depth), XmlError>>,
) -> Option<StartEl<'inp>> {
    let mut out = StartEl::new("", "", 0);
    loop {
        match tokens.next()? {
            Ok((Token::ElementStart { local, prefix, .. }, depth)) => {
                out.name.local = unescape(local.as_str());
                out.name.prefix = unescape(prefix.as_str());
                out.depth = depth;
            }
            Ok((
                Token::Attribute {
                    prefix,
                    local,
                    value,
                    ..
                },
                _,
            )) => out.attributes.push(Attr {
                name: Name {
                    local: unescape(local.as_str()),
                    prefix: unescape(prefix.as_str()),
                },
                value: unescape(value.as_str()),
            }),
            Ok((
                Token::ElementEnd {
                    end: ElementEnd::Open,
                    ..
                },
                _,
            )) => break,
            Ok((
                Token::ElementEnd {
                    end: ElementEnd::Empty,
                    ..
                },
                _,
            )) => {
                out.closed = true;
                break;
            }
            _ => {}
        }
    }
    Some(out)
}

/// Returns the data element at the current position
///
/// If the current position is not a data element (and is instead a <startelement>) an error
/// will be returned
pub fn try_data<'a, 'inp>(
    tokens: &'a mut impl Iterator<Item = Result<(Token<'inp>, Depth), XmlError>>,
) -> Result<&'inp str, XmlError> {
    loop {
        match tokens.next().map(|opt| opt.map(|opt| opt.0)) {
            None => return Ok(""),
            Some(Ok(Token::Text { text })) if !text.as_str().trim().is_empty() => {
                return Ok(text.as_str().trim())
            }
            Some(Ok(e @ Token::ElementStart { .. })) => {
                return Err(XmlError::ExpectedData {
                    found: format!("{:?}", e),
                })
            }
            Some(Err(e)) => return Err(e),
            _ => {}
        }
    }
}

#[cfg(test)]
mod test {
    use crate::decode::{try_data, Attr, Document, Name, StartEl};

    #[test]
    fn scoped_tokens() {
        let xml = r#"<Response><A></A></Response>"#;
        let mut doc = Document::new(xml);
        let mut root = doc.root_element().expect("valid document");
        assert_eq!(root.start_el().local(), "Response");
        assert_eq!(root.next_tag().expect("tag exists").start_el().local(), "A");
        assert!(root.next_tag().is_none());
    }

    #[test]
    fn handle_depth_properly() {
        let xml = r#"<Response><Response></Response><A/></Response>"#;
        let mut doc = Document::new(xml);
        let mut scoped = doc.root_element().expect("valid document");
        assert_eq!(
            scoped.next_tag().unwrap().start_el(),
            &StartEl::new("Response", "", 1)
        );
        let closed_a = StartEl::closed("A", "", 1);
        assert_eq!(scoped.next_tag().unwrap().start_el(), &closed_a);
        assert!(scoped.next_tag().is_none())
    }

    #[test]
    fn self_closing() {
        let xml = r#"<Response/>"#;
        let mut doc = Document::new(xml);
        let mut scoped = doc.root_element().expect("valid doc");
        assert_eq!(scoped.start_el.closed, true);
        assert!(scoped.next_tag().is_none())
    }

    #[test]
    fn terminate_scope() {
        let xml = r#"<Response><Struct><A></A><Also/></Struct><More/></Response>"#;
        let mut doc = Document::new(xml);
        let mut response_iter = doc.root_element().expect("valid doc");
        let mut struct_iter = response_iter.next_tag().unwrap();
        assert_eq!(
            struct_iter.next_tag().as_ref().map(|t| t.start_el()),
            Some(&StartEl::new("A", "", 2))
        );
        // When the inner iter is dropped, it will read to the end of its scope
        // prevent accidental behavior where we didn't read a full node
        drop(struct_iter);
        assert_eq!(
            response_iter.next_tag().unwrap().start_el(),
            &StartEl::closed("More", "", 1)
        );
    }

    #[test]
    fn read_data_invalid() {
        let xml = r#"<Response><A></A></Response>"#;
        let mut doc = Document::new(xml);
        let mut resp = doc.root_element().unwrap();
        try_data(&mut resp).expect_err("no data");
    }

    #[test]
    fn read_data() {
        let xml = r#"<Response>hello</Response>"#;
        let mut doc = Document::new(xml);
        let mut scoped = doc.root_element().unwrap();
        assert_eq!(try_data(&mut scoped), Ok("hello"));
    }

    #[test]
    fn read_attributes() {
        let xml = r#"<Response xsi:type="CanonicalUser">hello</Response>"#;
        let mut tokenizer = Document::new(xml);
        let root = tokenizer.root_element().unwrap();

        assert_eq!(
            root.start_el().attributes,
            vec![Attr {
                name: Name {
                    prefix: "xsi".into(),
                    local: "type".into()
                },
                value: "CanonicalUser".into()
            }]
        )
    }

    #[test]
    #[ignore]
    fn escape_data() {
        let xml = r#"<Response>&gt;</Response>"#;
        let mut doc = Document::new(xml);
        let mut root = doc.root_element().unwrap();
        assert_eq!(try_data(&mut root), Ok(">"));
    }

    #[test]
    fn nested_self_closer() {
        let xml = r#"<XmlListsInputOutput>
                <stringList/>
                <stringSet></stringSet>
        </XmlListsInputOutput>"#;
        let mut doc = Document::new(xml);
        let mut root = doc.root_element().unwrap();
        let mut string_list = root.next_tag().unwrap();
        assert_eq!(
            string_list.start_el(),
            &StartEl::closed("stringList", "", 1)
        );
        assert!(string_list.next_tag().is_none());
        drop(string_list);
        assert_eq!(
            root.next_tag().unwrap().start_el(),
            &StartEl::new("stringSet", "", 1)
        );
    }

    #[test]
    fn confusing_nested_same_name_tag() {
        // an inner b which could be confused as closing the outer b if depth
        // is not properly tracked:
        let root_tags = &["a", "b", "c", "d"];
        let xml = r#"<XmlListsInputOutput>
                <a/>
                <b><c/><b></b><here/></b>
                <c></c>
                <d>more</d>
        </XmlListsInputOutput>"#;
        let mut doc = Document::new(xml);
        let mut root = doc.root_element().unwrap();
        let mut cmp = vec![];
        while let Some(tag) = root.next_tag() {
            cmp.push(tag.start_el().local().to_owned());
        }
        assert_eq!(root_tags, cmp.as_slice());
    }
}
