pub static TEXT: &'static str = "use std::ops::{Deref, DerefMut};
use std::borrow::Cow;
use std::convert::AsRef;
use std::cmp::Ordering;
use std::fmt;
use std::io::{self, Read};

use ammonia::clean;
use serde::{Serialize, Serializer};
use diesel;

use rocket::http::{RawStr, Status};
use rocket::http::uri::UriDisplay;
use rocket::http::uri::FromUriParam;
use rocket::http::uncased::UncasedStr;
use rocket::request::{FromFormValue, FromParam};
use rocket::data::{self, Data, FromData};
use rocket::{Outcome, Request};

/// A convenience to sanitize a string.
///
/// A `SanitizedStr` is a sanitized string from an
/// Into<String> type. It exists to sanitize string inputs, represented by
/// the `String`, `&str`, and `Cow<str>` types.
///
/// # Usage
///
/// A `SanitizedStr` is a new type wrapper for String. It can be used as a replacement
/// of String for Diesel, Serde, and for request parameters, query strings, form data.
/// You can use this to construct a struct which will automatically clean this when
/// it's formatted or  inserted into database. The conversion is not done automatically
///
#[repr(C)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct SanitizedStr(String);

impl SanitizedStr {
    /// Constructs an `SanitizedStr` from an `Into<String>`.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use guards::sanitized_str::SanitizedStr;
    ///
    /// let sanitized_str = SanitizedStr::new(\"Hello, world!\");
    ///
    /// // `into` can also be used; note that the type must be specified
    /// let sanitized_str: &SanitizedStr = \"Hello, world!\".into();
    /// ```
    #[inline(always)]
    pub fn new<S: Into<String>>(string: S) -> SanitizedStr {
        SanitizedStr(string.into())
    }

    /// Returns an HTML santized version of `self`.
    ///
    /// Very fast sanitizing, uses [https://github.com/notriddle/ammonia](ammonia).
    ///
    /// # Example
    ///
    /// Strings with HTML sequences are escaped:
    ///
    /// ```rust,ignore
    /// use guards::sanitized_str::SanitizedStr;
    ///
    /// let sanitized_str: &SanitizedStr = \"<b>Hi!</b>\".into();
    /// let escaped = sanitized_str.sanitize();
    /// assert_eq!(escaped.to_owned(), \"&<b>Hi!</b>\");
    ///
    /// let sanitized_str: &SanitizedStr = \"Hello, <script>alert('abc')</script>\".into();
    /// let escaped = sanitized_str.sanitize();
    /// assert_eq!(escaped.to_owned(), \"Hello, alert('abc')\");
    /// ```
    ///
    /// Strings without HTML sequences remain untouched:
    ///
    /// ```rust,ignore
    /// use guards::sanitized_str::SanitizedStr;
    ///
    /// let sanitized_str: &SanitizedStr = \"Hello!\".into();
    /// let escaped = sanitized_str.sanitize();
    /// assert_eq!(escaped.to_owned(), \"Hello!\");
    ///
    /// let sanitized_str: &SanitizedStr = \"大阪\".into();
    /// let escaped = sanitized_str.sanitize();
    /// assert_eq!(escaped.to_owned(), \"大阪\");
    /// ```
    pub fn sanitize(&self) -> Cow<String> {
        Cow::Owned(clean(self))
    }

    /// Converts `self` into an `&str`.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use guards::sanitized_str::SanitizedStr;
    ///
    /// let sanitized_str = SanitizedStr::new(\"Hello, world!\");
    /// assert_eq!(sanitized_str.as_str(), \"Hello, world!\");
    /// ```
    #[inline(always)]
    pub fn as_str(&self) -> &str {
        self
    }

    /// Converts `self` into an `&UncasedStr`.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use guards::sanitized_str::SanitizedStr;
    ///
    /// let sanitized_str = SanitizedStr::new(\"Content-Type\");
    /// assert!(sanitized_str.as_uncased_str() == \"content-TYPE\");
    /// ```
    #[inline(always)]
    pub fn as_uncased_str(&self) -> &UncasedStr {
        self.as_str().into()
    }
}

impl<'a> From<&'a str> for &'a SanitizedStr {
    #[inline(always)]
    fn from(string: &'a str) -> &'a SanitizedStr {
        unsafe { &*(string as *const str as *const SanitizedStr) }
    }
}

impl PartialEq<str> for SanitizedStr {
    #[inline(always)]
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<String> for SanitizedStr {
    #[inline(always)]
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other.as_str()
    }
}

impl<'a> PartialEq<String> for &'a SanitizedStr {
    #[inline(always)]
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialOrd<str> for SanitizedStr {
    #[inline(always)]
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        (self as &str).partial_cmp(other)
    }
}

impl AsRef<str> for SanitizedStr {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self
    }
}

impl AsRef<[u8]> for SanitizedStr {
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Deref for SanitizedStr {
    type Target = str;

    #[inline(always)]
    fn deref(&self) -> &str {
        &self.0
    }
}

impl DerefMut for SanitizedStr {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut str {
        &mut self.0
    }
}

impl fmt::Display for SanitizedStr {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.sanitize(), f)
    }
}

/// Implement Serialize trait for `SanitizedStr`
/// It's escaped automatically.
/// If you want to get unescaped string you need to deref and put the string to your object directly.
impl Serialize for SanitizedStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.sanitize())
    }
}

impl<'a> FromParam<'a> for SanitizedStr {
    type Error = &'a RawStr;

    #[inline(always)]
    fn from_param(param: &'a RawStr) -> Result<SanitizedStr, Self::Error> {
        param
            .percent_decode()
            .map(|cow| SanitizedStr::new(clean(&cow)))
            .map_err(|_| param)
    }
}

impl<'v> FromFormValue<'v> for SanitizedStr {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<SanitizedStr, Self::Error> {
        form_value
            .percent_decode()
            .map(|cow| SanitizedStr::new(clean(&cow)))
            .map_err(|_| form_value)
    }
}

impl FromData for SanitizedStr {
    type Error = io::Error;

    fn from_data(_req: &Request, data: Data) -> data::Outcome<Self, Self::Error> {
        let mut string = String::new();
        match data.open().read_to_string(&mut string) {
            Ok(_) => Outcome::Success(SanitizedStr::new(clean(&string))),
            Err(e) => Outcome::Failure((Status::BadRequest, e)),
        }
    }
}

impl<'a> UriDisplay for SanitizedStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.sanitize().fmt(f)
    }
}

impl<'a> FromUriParam<(&'a str)> for SanitizedStr {
    type Target = SanitizedStr;
    fn from_uri_param(string: &'a str) -> SanitizedStr {
        SanitizedStr::new(clean(string))
    }
}

impl diesel::sql_types::NotNull for SanitizedStr {}
impl diesel::sql_types::SingleValue for SanitizedStr {}

impl<ST, DB> diesel::types::ToSql<ST, DB> for SanitizedStr
where
    String: diesel::types::ToSql<ST, DB>,
    DB: diesel::backend::Backend,
    DB: diesel::types::HasSqlType<ST>,
{
    #[allow(deprecated)]
    fn to_sql<W: ::std::io::Write>(
        &self,
        out: &mut diesel::types::ToSqlOutput<W, DB>,
    ) -> Result<diesel::types::IsNull, Box<::std::error::Error + Send + Sync>> {
        self.sanitize().to_sql(out)
    }
}
impl<ST> diesel::expression::AsExpression<ST> for SanitizedStr
where
    diesel::expression::bound::Bound<ST, String>: diesel::expression::Expression<SqlType = ST>,
{
    type Expression = diesel::expression::bound::Bound<ST, String>;
    fn as_expression(self) -> Self::Expression {
        diesel::expression::bound::Bound::new(self.0)
    }
}
impl<'expr, ST> diesel::expression::AsExpression<ST> for &'expr SanitizedStr
where
    diesel::expression::bound::Bound<ST, String>: diesel::expression::Expression<SqlType = ST>,
{
    type Expression = diesel::expression::bound::Bound<ST, &'expr String>;
    fn as_expression(self) -> Self::Expression {
        diesel::expression::bound::Bound::new(&self.0)
    }
}
impl<ST, DB> diesel::types::FromSql<ST, DB> for SanitizedStr
where
    String: diesel::types::FromSql<ST, DB>,
    DB: diesel::backend::Backend,
    DB: diesel::types::HasSqlType<ST>,
{
    fn from_sql(
        raw: Option<&<DB as diesel::backend::Backend>::RawValue>,
    ) -> Result<Self, Box<::std::error::Error + Send + Sync>> {
        diesel::types::FromSql::<ST, DB>::from_sql(raw).map(SanitizedStr)
    }
}
impl<ST, DB> diesel::types::FromSqlRow<ST, DB> for SanitizedStr
where
    String: diesel::types::FromSql<ST, DB>,
    DB: diesel::backend::Backend,
    DB: diesel::types::HasSqlType<ST>,
{
    fn build_from_row<R: diesel::row::Row<DB>>(
        row: &mut R,
    ) -> Result<Self, Box<::std::error::Error + Send + Sync>> {
        diesel::types::FromSql::<ST, DB>::from_sql(row.take())
    }
}
impl<ST, DB> diesel::query_source::Queryable<ST, DB> for SanitizedStr
where
    String: diesel::types::FromSqlRow<ST, DB>,
    DB: diesel::backend::Backend,
    DB: diesel::types::HasSqlType<ST>,
{
    type Row = String;
    fn build(row: Self::Row) -> Self {
        SanitizedStr(row)
    }
}
impl diesel::query_builder::QueryId for SanitizedStr {
    type QueryId = Self;
}
";
