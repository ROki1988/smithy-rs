// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use crate::model::ComplexNestedErrorData;
use crate::model::KitchenSink;
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum EmptyOperationError {
    /// An unexpected error, eg. invalid JSON returned by the service
    Unhandled(Box<dyn ::std::error::Error>),
}
impl ::std::fmt::Display for EmptyOperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmptyOperationError::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl EmptyOperationError {
    pub fn unhandled<E: Into<Box<dyn ::std::error::Error>>>(err: E) -> Self {
        EmptyOperationError::Unhandled(err.into())
    }
    pub fn message(&self) -> Option<&str> {
        match self {
            EmptyOperationError::Unhandled(_inner) => {
                match _inner.downcast_ref::<::smithy_types::Error>() {
                    Some(_inner) => _inner.message(),
                    None => None,
                }
            }
        }
    }
    pub fn code(&self) -> Option<&str> {
        match self {
            EmptyOperationError::Unhandled(_inner) => {
                match _inner.downcast_ref::<::smithy_types::Error>() {
                    Some(_inner) => _inner.code(),
                    None => None,
                }
            }
        }
    }
}
impl ::std::error::Error for EmptyOperationError {
    fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            EmptyOperationError::Unhandled(_inner) => {
                match _inner.downcast_ref::<::smithy_types::Error>() {
                    Some(_inner) => Some(_inner),
                    None => Some(_inner.as_ref()),
                }
            }
        }
    }
}

#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum GreetingWithErrorsError {
    InvalidGreeting(InvalidGreeting),
    ComplexError(ComplexError),
    FooError(FooError),

    /// An unexpected error, eg. invalid JSON returned by the service
    Unhandled(Box<dyn ::std::error::Error>),
}
impl ::std::fmt::Display for GreetingWithErrorsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GreetingWithErrorsError::InvalidGreeting(_inner) => _inner.fmt(f),
            GreetingWithErrorsError::ComplexError(_inner) => _inner.fmt(f),
            GreetingWithErrorsError::FooError(_inner) => _inner.fmt(f),
            GreetingWithErrorsError::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl GreetingWithErrorsError {
    pub fn unhandled<E: Into<Box<dyn ::std::error::Error>>>(err: E) -> Self {
        GreetingWithErrorsError::Unhandled(err.into())
    }
    pub fn message(&self) -> Option<&str> {
        match self {
            GreetingWithErrorsError::InvalidGreeting(_inner) => _inner.message(),
            GreetingWithErrorsError::ComplexError(_inner) => _inner.message(),
            GreetingWithErrorsError::FooError(_inner) => _inner.message(),
            GreetingWithErrorsError::Unhandled(_inner) => {
                match _inner.downcast_ref::<::smithy_types::Error>() {
                    Some(_inner) => _inner.message(),
                    None => None,
                }
            }
        }
    }
    pub fn code(&self) -> Option<&str> {
        match self {
            GreetingWithErrorsError::InvalidGreeting(_inner) => Some(_inner.code()),
            GreetingWithErrorsError::ComplexError(_inner) => Some(_inner.code()),
            GreetingWithErrorsError::FooError(_inner) => Some(_inner.code()),
            GreetingWithErrorsError::Unhandled(_inner) => {
                match _inner.downcast_ref::<::smithy_types::Error>() {
                    Some(_inner) => _inner.code(),
                    None => None,
                }
            }
        }
    }
}
impl ::std::error::Error for GreetingWithErrorsError {
    fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            GreetingWithErrorsError::InvalidGreeting(_inner) => Some(_inner),
            GreetingWithErrorsError::ComplexError(_inner) => Some(_inner),
            GreetingWithErrorsError::FooError(_inner) => Some(_inner),
            GreetingWithErrorsError::Unhandled(_inner) => {
                match _inner.downcast_ref::<::smithy_types::Error>() {
                    Some(_inner) => Some(_inner),
                    None => Some(_inner.as_ref()),
                }
            }
        }
    }
}

#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum JsonEnumsError {
    /// An unexpected error, eg. invalid JSON returned by the service
    Unhandled(Box<dyn ::std::error::Error>),
}
impl ::std::fmt::Display for JsonEnumsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonEnumsError::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl JsonEnumsError {
    pub fn unhandled<E: Into<Box<dyn ::std::error::Error>>>(err: E) -> Self {
        JsonEnumsError::Unhandled(err.into())
    }
    pub fn message(&self) -> Option<&str> {
        match self {
            JsonEnumsError::Unhandled(_inner) => {
                match _inner.downcast_ref::<::smithy_types::Error>() {
                    Some(_inner) => _inner.message(),
                    None => None,
                }
            }
        }
    }
    pub fn code(&self) -> Option<&str> {
        match self {
            JsonEnumsError::Unhandled(_inner) => {
                match _inner.downcast_ref::<::smithy_types::Error>() {
                    Some(_inner) => _inner.code(),
                    None => None,
                }
            }
        }
    }
}
impl ::std::error::Error for JsonEnumsError {
    fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            JsonEnumsError::Unhandled(_inner) => {
                match _inner.downcast_ref::<::smithy_types::Error>() {
                    Some(_inner) => Some(_inner),
                    None => Some(_inner.as_ref()),
                }
            }
        }
    }
}

#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum JsonUnionsError {
    /// An unexpected error, eg. invalid JSON returned by the service
    Unhandled(Box<dyn ::std::error::Error>),
}
impl ::std::fmt::Display for JsonUnionsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonUnionsError::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl JsonUnionsError {
    pub fn unhandled<E: Into<Box<dyn ::std::error::Error>>>(err: E) -> Self {
        JsonUnionsError::Unhandled(err.into())
    }
    pub fn message(&self) -> Option<&str> {
        match self {
            JsonUnionsError::Unhandled(_inner) => {
                match _inner.downcast_ref::<::smithy_types::Error>() {
                    Some(_inner) => _inner.message(),
                    None => None,
                }
            }
        }
    }
    pub fn code(&self) -> Option<&str> {
        match self {
            JsonUnionsError::Unhandled(_inner) => {
                match _inner.downcast_ref::<::smithy_types::Error>() {
                    Some(_inner) => _inner.code(),
                    None => None,
                }
            }
        }
    }
}
impl ::std::error::Error for JsonUnionsError {
    fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            JsonUnionsError::Unhandled(_inner) => {
                match _inner.downcast_ref::<::smithy_types::Error>() {
                    Some(_inner) => Some(_inner),
                    None => Some(_inner.as_ref()),
                }
            }
        }
    }
}

#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum KitchenSinkOperationError {
    ErrorWithMembers(ErrorWithMembers),
    ErrorWithoutMembers(ErrorWithoutMembers),

    /// An unexpected error, eg. invalid JSON returned by the service
    Unhandled(Box<dyn ::std::error::Error>),
}
impl ::std::fmt::Display for KitchenSinkOperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KitchenSinkOperationError::ErrorWithMembers(_inner) => _inner.fmt(f),
            KitchenSinkOperationError::ErrorWithoutMembers(_inner) => _inner.fmt(f),
            KitchenSinkOperationError::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl KitchenSinkOperationError {
    pub fn unhandled<E: Into<Box<dyn ::std::error::Error>>>(err: E) -> Self {
        KitchenSinkOperationError::Unhandled(err.into())
    }
    pub fn message(&self) -> Option<&str> {
        match self {
            KitchenSinkOperationError::ErrorWithMembers(_inner) => _inner.message(),
            KitchenSinkOperationError::ErrorWithoutMembers(_inner) => _inner.message(),
            KitchenSinkOperationError::Unhandled(_inner) => {
                match _inner.downcast_ref::<::smithy_types::Error>() {
                    Some(_inner) => _inner.message(),
                    None => None,
                }
            }
        }
    }
    pub fn code(&self) -> Option<&str> {
        match self {
            KitchenSinkOperationError::ErrorWithMembers(_inner) => Some(_inner.code()),
            KitchenSinkOperationError::ErrorWithoutMembers(_inner) => Some(_inner.code()),
            KitchenSinkOperationError::Unhandled(_inner) => {
                match _inner.downcast_ref::<::smithy_types::Error>() {
                    Some(_inner) => _inner.code(),
                    None => None,
                }
            }
        }
    }
}
impl ::std::error::Error for KitchenSinkOperationError {
    fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            KitchenSinkOperationError::ErrorWithMembers(_inner) => Some(_inner),
            KitchenSinkOperationError::ErrorWithoutMembers(_inner) => Some(_inner),
            KitchenSinkOperationError::Unhandled(_inner) => {
                match _inner.downcast_ref::<::smithy_types::Error>() {
                    Some(_inner) => Some(_inner),
                    None => Some(_inner.as_ref()),
                }
            }
        }
    }
}

#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum NullOperationError {
    /// An unexpected error, eg. invalid JSON returned by the service
    Unhandled(Box<dyn ::std::error::Error>),
}
impl ::std::fmt::Display for NullOperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NullOperationError::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl NullOperationError {
    pub fn unhandled<E: Into<Box<dyn ::std::error::Error>>>(err: E) -> Self {
        NullOperationError::Unhandled(err.into())
    }
    pub fn message(&self) -> Option<&str> {
        match self {
            NullOperationError::Unhandled(_inner) => {
                match _inner.downcast_ref::<::smithy_types::Error>() {
                    Some(_inner) => _inner.message(),
                    None => None,
                }
            }
        }
    }
    pub fn code(&self) -> Option<&str> {
        match self {
            NullOperationError::Unhandled(_inner) => {
                match _inner.downcast_ref::<::smithy_types::Error>() {
                    Some(_inner) => _inner.code(),
                    None => None,
                }
            }
        }
    }
}
impl ::std::error::Error for NullOperationError {
    fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            NullOperationError::Unhandled(_inner) => {
                match _inner.downcast_ref::<::smithy_types::Error>() {
                    Some(_inner) => Some(_inner),
                    None => Some(_inner.as_ref()),
                }
            }
        }
    }
}

#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum OperationWithOptionalInputOutputError {
    /// An unexpected error, eg. invalid JSON returned by the service
    Unhandled(Box<dyn ::std::error::Error>),
}
impl ::std::fmt::Display for OperationWithOptionalInputOutputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationWithOptionalInputOutputError::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl OperationWithOptionalInputOutputError {
    pub fn unhandled<E: Into<Box<dyn ::std::error::Error>>>(err: E) -> Self {
        OperationWithOptionalInputOutputError::Unhandled(err.into())
    }
    pub fn message(&self) -> Option<&str> {
        match self {
            OperationWithOptionalInputOutputError::Unhandled(_inner) => {
                match _inner.downcast_ref::<::smithy_types::Error>() {
                    Some(_inner) => _inner.message(),
                    None => None,
                }
            }
        }
    }
    pub fn code(&self) -> Option<&str> {
        match self {
            OperationWithOptionalInputOutputError::Unhandled(_inner) => {
                match _inner.downcast_ref::<::smithy_types::Error>() {
                    Some(_inner) => _inner.code(),
                    None => None,
                }
            }
        }
    }
}
impl ::std::error::Error for OperationWithOptionalInputOutputError {
    fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            OperationWithOptionalInputOutputError::Unhandled(_inner) => {
                match _inner.downcast_ref::<::smithy_types::Error>() {
                    Some(_inner) => Some(_inner),
                    None => Some(_inner.as_ref()),
                }
            }
        }
    }
}

#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum PutAndGetInlineDocumentsError {
    /// An unexpected error, eg. invalid JSON returned by the service
    Unhandled(Box<dyn ::std::error::Error>),
}
impl ::std::fmt::Display for PutAndGetInlineDocumentsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PutAndGetInlineDocumentsError::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl PutAndGetInlineDocumentsError {
    pub fn unhandled<E: Into<Box<dyn ::std::error::Error>>>(err: E) -> Self {
        PutAndGetInlineDocumentsError::Unhandled(err.into())
    }
    pub fn message(&self) -> Option<&str> {
        match self {
            PutAndGetInlineDocumentsError::Unhandled(_inner) => {
                match _inner.downcast_ref::<::smithy_types::Error>() {
                    Some(_inner) => _inner.message(),
                    None => None,
                }
            }
        }
    }
    pub fn code(&self) -> Option<&str> {
        match self {
            PutAndGetInlineDocumentsError::Unhandled(_inner) => {
                match _inner.downcast_ref::<::smithy_types::Error>() {
                    Some(_inner) => _inner.code(),
                    None => None,
                }
            }
        }
    }
}
impl ::std::error::Error for PutAndGetInlineDocumentsError {
    fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            PutAndGetInlineDocumentsError::Unhandled(_inner) => {
                match _inner.downcast_ref::<::smithy_types::Error>() {
                    Some(_inner) => Some(_inner),
                    None => Some(_inner.as_ref()),
                }
            }
        }
    }
}

/// This error has test cases that test some of the dark corners of Amazon service
/// framework history. It should only be implemented by clients.
#[non_exhaustive]
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    ::std::clone::Clone,
    ::std::cmp::PartialEq,
    ::std::fmt::Debug,
)]
pub struct FooError {}
impl FooError {
    pub fn retryable(&self) -> bool {
        false
    }
    pub fn throttling(&self) -> bool {
        false
    }
    pub fn code(&self) -> &str {
        "FooError"
    }
    pub fn message(&self) -> Option<&str> {
        None
    }
}
impl ::std::fmt::Display for FooError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FooError")?;
        Ok(())
    }
}
impl ::std::error::Error for FooError {}
/// See [`FooError`](crate::error::FooError)
pub mod foo_error {

    use crate::error::FooError;
    /// A builder for [`FooError`](crate::error::FooError)
    #[non_exhaustive]
    #[derive(Debug, Clone, Default)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`FooError`](crate::error::FooError)
        pub fn build(self) -> FooError {
            FooError {}
        }
    }
}
impl FooError {
    /// Creates a new builder-style object to manufacture [`FooError`](crate::error::FooError)
    pub fn builder() -> crate::error::foo_error::Builder {
        crate::error::foo_error::Builder::default()
    }
}

/// This error is thrown when a request is invalid.
#[non_exhaustive]
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    ::std::clone::Clone,
    ::std::cmp::PartialEq,
    ::std::fmt::Debug,
)]
pub struct ComplexError {
    #[serde(rename = "TopLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub top_level: ::std::option::Option<::std::string::String>,
    #[serde(rename = "Nested")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub nested: ::std::option::Option<ComplexNestedErrorData>,
}
impl ComplexError {
    pub fn retryable(&self) -> bool {
        false
    }
    pub fn throttling(&self) -> bool {
        false
    }
    pub fn code(&self) -> &str {
        "ComplexError"
    }
    pub fn message(&self) -> Option<&str> {
        None
    }
}
impl ::std::fmt::Display for ComplexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ComplexError")?;
        Ok(())
    }
}
impl ::std::error::Error for ComplexError {}
/// See [`ComplexError`](crate::error::ComplexError)
pub mod complex_error {

    use crate::error::ComplexError;
    use crate::model::ComplexNestedErrorData;
    /// A builder for [`ComplexError`](crate::error::ComplexError)
    #[non_exhaustive]
    #[derive(Debug, Clone, Default)]
    pub struct Builder {
        top_level: ::std::option::Option<::std::string::String>,
        nested: ::std::option::Option<ComplexNestedErrorData>,
    }
    impl Builder {
        pub fn top_level(mut self, inp: impl Into<::std::string::String>) -> Self {
            self.top_level = Some(inp.into());
            self
        }
        pub fn nested(mut self, inp: ComplexNestedErrorData) -> Self {
            self.nested = Some(inp);
            self
        }
        /// Consumes the builder and constructs a [`ComplexError`](crate::error::ComplexError)
        pub fn build(self) -> ComplexError {
            ComplexError {
                top_level: self.top_level,
                nested: self.nested,
            }
        }
    }
}
impl ComplexError {
    /// Creates a new builder-style object to manufacture [`ComplexError`](crate::error::ComplexError)
    pub fn builder() -> crate::error::complex_error::Builder {
        crate::error::complex_error::Builder::default()
    }
}

/// This error is thrown when an invalid greeting value is provided.
#[non_exhaustive]
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    ::std::clone::Clone,
    ::std::cmp::PartialEq,
    ::std::fmt::Debug,
)]
pub struct InvalidGreeting {
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub message: ::std::option::Option<::std::string::String>,
}
impl InvalidGreeting {
    pub fn retryable(&self) -> bool {
        false
    }
    pub fn throttling(&self) -> bool {
        false
    }
    pub fn code(&self) -> &str {
        "InvalidGreeting"
    }
    pub fn message(&self) -> Option<&str> {
        None
    }
}
impl ::std::fmt::Display for InvalidGreeting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidGreeting")?;
        Ok(())
    }
}
impl ::std::error::Error for InvalidGreeting {}
/// See [`InvalidGreeting`](crate::error::InvalidGreeting)
pub mod invalid_greeting {

    use crate::error::InvalidGreeting;
    /// A builder for [`InvalidGreeting`](crate::error::InvalidGreeting)
    #[non_exhaustive]
    #[derive(Debug, Clone, Default)]
    pub struct Builder {
        message: ::std::option::Option<::std::string::String>,
    }
    impl Builder {
        pub fn message(mut self, inp: impl Into<::std::string::String>) -> Self {
            self.message = Some(inp.into());
            self
        }
        /// Consumes the builder and constructs a [`InvalidGreeting`](crate::error::InvalidGreeting)
        pub fn build(self) -> InvalidGreeting {
            InvalidGreeting {
                message: self.message,
            }
        }
    }
}
impl InvalidGreeting {
    /// Creates a new builder-style object to manufacture [`InvalidGreeting`](crate::error::InvalidGreeting)
    pub fn builder() -> crate::error::invalid_greeting::Builder {
        crate::error::invalid_greeting::Builder::default()
    }
}

#[non_exhaustive]
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    ::std::clone::Clone,
    ::std::cmp::PartialEq,
    ::std::fmt::Debug,
)]
pub struct ErrorWithoutMembers {}
impl ErrorWithoutMembers {
    pub fn retryable(&self) -> bool {
        false
    }
    pub fn throttling(&self) -> bool {
        false
    }
    pub fn code(&self) -> &str {
        "ErrorWithoutMembers"
    }
    pub fn message(&self) -> Option<&str> {
        None
    }
}
impl ::std::fmt::Display for ErrorWithoutMembers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ErrorWithoutMembers")?;
        Ok(())
    }
}
impl ::std::error::Error for ErrorWithoutMembers {}
/// See [`ErrorWithoutMembers`](crate::error::ErrorWithoutMembers)
pub mod error_without_members {

    use crate::error::ErrorWithoutMembers;
    /// A builder for [`ErrorWithoutMembers`](crate::error::ErrorWithoutMembers)
    #[non_exhaustive]
    #[derive(Debug, Clone, Default)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`ErrorWithoutMembers`](crate::error::ErrorWithoutMembers)
        pub fn build(self) -> ErrorWithoutMembers {
            ErrorWithoutMembers {}
        }
    }
}
impl ErrorWithoutMembers {
    /// Creates a new builder-style object to manufacture [`ErrorWithoutMembers`](crate::error::ErrorWithoutMembers)
    pub fn builder() -> crate::error::error_without_members::Builder {
        crate::error::error_without_members::Builder::default()
    }
}

#[non_exhaustive]
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    ::std::clone::Clone,
    ::std::cmp::PartialEq,
    ::std::fmt::Debug,
)]
pub struct ErrorWithMembers {
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ComplexData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub complex_data: ::std::option::Option<KitchenSink>,
    #[serde(rename = "IntegerField")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub integer_field: ::std::option::Option<i32>,
    #[serde(rename = "ListField")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub list_field: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "MapField")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub map_field: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub message: ::std::option::Option<::std::string::String>,
    /// abc
    #[serde(rename = "StringField")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub string_field: ::std::option::Option<::std::string::String>,
}
impl ErrorWithMembers {
    pub fn retryable(&self) -> bool {
        false
    }
    pub fn throttling(&self) -> bool {
        false
    }
    pub fn code(&self) -> &str {
        "ErrorWithMembers"
    }
    pub fn message(&self) -> Option<&str> {
        None
    }
}
impl ::std::fmt::Display for ErrorWithMembers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ErrorWithMembers")?;
        Ok(())
    }
}
impl ::std::error::Error for ErrorWithMembers {}
/// See [`ErrorWithMembers`](crate::error::ErrorWithMembers)
pub mod error_with_members {

    use crate::error::ErrorWithMembers;
    use crate::model::KitchenSink;
    /// A builder for [`ErrorWithMembers`](crate::error::ErrorWithMembers)
    #[non_exhaustive]
    #[derive(Debug, Clone, Default)]
    pub struct Builder {
        code: ::std::option::Option<::std::string::String>,
        complex_data: ::std::option::Option<KitchenSink>,
        integer_field: ::std::option::Option<i32>,
        list_field: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        map_field: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
        message: ::std::option::Option<::std::string::String>,
        string_field: ::std::option::Option<::std::string::String>,
    }
    impl Builder {
        pub fn code(mut self, inp: impl Into<::std::string::String>) -> Self {
            self.code = Some(inp.into());
            self
        }
        pub fn complex_data(mut self, inp: KitchenSink) -> Self {
            self.complex_data = Some(inp);
            self
        }
        pub fn integer_field(mut self, inp: i32) -> Self {
            self.integer_field = Some(inp);
            self
        }
        pub fn list_field(mut self, inp: ::std::vec::Vec<::std::string::String>) -> Self {
            self.list_field = Some(inp);
            self
        }
        pub fn map_field(
            mut self,
            inp: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ) -> Self {
            self.map_field = Some(inp);
            self
        }
        pub fn message(mut self, inp: impl Into<::std::string::String>) -> Self {
            self.message = Some(inp.into());
            self
        }
        /// abc
        pub fn string_field(mut self, inp: impl Into<::std::string::String>) -> Self {
            self.string_field = Some(inp.into());
            self
        }
        /// Consumes the builder and constructs a [`ErrorWithMembers`](crate::error::ErrorWithMembers)
        pub fn build(self) -> ErrorWithMembers {
            ErrorWithMembers {
                code: self.code,
                complex_data: self.complex_data,
                integer_field: self.integer_field,
                list_field: self.list_field,
                map_field: self.map_field,
                message: self.message,
                string_field: self.string_field,
            }
        }
    }
}
impl ErrorWithMembers {
    /// Creates a new builder-style object to manufacture [`ErrorWithMembers`](crate::error::ErrorWithMembers)
    pub fn builder() -> crate::error::error_with_members::Builder {
        crate::error::error_with_members::Builder::default()
    }
}
