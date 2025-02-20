use crate::{Error, Result};

use std::borrow::Borrow;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::iter::Peekable;
use std::ops::Deref;

/// A (possibly nested) column name.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(transparent, no_pg_array))]
pub struct ResourceName(Vec<String>);

impl ResourceName {
    /// Creates a new column name from input satisfying `FromIterator for ColumnName`. The provided
    /// field names are concatenated into a single path.
    pub fn new<A>(iter: impl IntoIterator<Item = A>) -> Self
    where
        Self: FromIterator<A>,
    {
        iter.into_iter().collect()
    }

    /// Naively splits a string at dots to create a column name.
    ///
    /// This method is _NOT_ recommended for production use, as it does not attempt to interpret
    /// special characters in field names. For example, many systems would interpret the field name
    /// `"a.b" . c ` as equivalent to `ColumnName::new(["\"a.b\"", "c"])` (two fields, whitespace
    /// padding ignored), but this method would return three fields, including whitespace:
    ///
    /// ```
    /// # use delta_kernel::resources::ResourceName;
    /// assert_eq!(
    ///     ColumnName::from_naive_str_split(" \"a.b\" . c "),
    ///     ColumnName::new([" \"a", "b\" ", " c "])
    /// );
    /// ```
    pub fn from_naive_str_split(name: impl AsRef<str>) -> Self {
        Self::new(name.as_ref().split(FIELD_SEPARATOR))
    }

    /// Parses a comma-separated list of column names, properly accounting for escapes and special
    /// characters, e.g.:
    ///
    /// ```
    /// # use delta_kernel::resources::ResourceName;
    /// assert_eq!(
    ///     &ColumnName::parse_column_name_list("a.b , c.`d , e` . f").unwrap(),
    ///     &[ColumnName::new(["a", "b"]), ColumnName::new(["c", "d , e", "f"])]
    /// );
    /// ```
    pub fn parse_column_name_list(names: impl AsRef<str>) -> Result<Vec<ResourceName>> {
        let names = names.as_ref();
        let chars = &mut names.chars().peekable();

        // Ambiguous case: The empty string `""` could reasonably parse as `[ColumnName::new([])]`
        // or `[]`. Prefer the latter as more intuitive and compatible with e.g. `str::join(',')`.
        drop_leading_whitespace(chars);
        let mut ending = match chars.peek() {
            Some(_) => FieldEnding::NextColumn,
            None => FieldEnding::InputExhausted,
        };

        let mut cols = vec![];
        while ending == FieldEnding::NextColumn {
            let (col, new_ending) = parse_resource_name(chars)?;
            cols.push(col);
            ending = new_ending;
        }
        Ok(cols)
    }

    /// Joins this column with another, concatenating their fields into a single nested column path.
    ///
    /// NOTE: This is a convenience method that copies two arguments without consuming them. If more
    /// arguments are needed, or if performance is a concern, it is recommended to use
    /// [`FromIterator for ColumnName`](#impl-FromIterator<ColumnName>-for-ColumnName) instead:
    ///
    /// ```
    /// # use delta_kernel::resources::ResourceName;
    /// let x = ColumnName::new(["a", "b"]);
    /// let y = ColumnName::new(["c", "d"]);
    /// let joined: ColumnName = [x, y].into_iter().collect();
    /// assert_eq!(joined, ColumnName::new(["a", "b", "c", "d"]));
    /// ```
    pub fn join(&self, right: &ResourceName) -> ResourceName {
        [self.clone(), right.clone()].into_iter().collect()
    }

    /// The path of field names for this column name
    pub fn path(&self) -> &[String] {
        &self.0
    }

    /// Consumes this column name and returns the path of field names.
    pub fn into_inner(self) -> Vec<String> {
        self.0
    }
}

/// Creates a new column name from a path of field names. Each field name is taken as-is, and may
/// contain arbitrary characters (including periods, spaces, etc.).
impl<A: Into<String>> FromIterator<A> for ResourceName {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let path = iter.into_iter().map(|s| s.into()).collect();
        Self(path)
    }
}

impl From<Vec<String>> for ResourceName {
    fn from(path: Vec<String>) -> Self {
        Self(path)
    }
}

/// Creates a new column name by joining multiple column names together.
impl FromIterator<ResourceName> for ResourceName {
    fn from_iter<T: IntoIterator<Item = ResourceName>>(iter: T) -> Self {
        let path = iter.into_iter().flat_map(|c| c.into_iter()).collect();
        Self(path)
    }
}

impl IntoIterator for ResourceName {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Deref for ResourceName {
    type Target = [String];

    fn deref(&self) -> &[String] {
        &self.0
    }
}

// Allows searching collections of `ColumnName` without an owned key value
impl Borrow<[String]> for ResourceName {
    fn borrow(&self) -> &[String] {
        self
    }
}

// Allows searching collections of `&ColumnName` without an owned key value. Needed because there is
// apparently no blanket `impl<U, T> Borrow<U> for &T where T: Borrow<U>`, even tho `Eq` [1] and
// `Hash` [2] both have blanket impl for treating `&T` like `T`.
//
// [1] https://doc.rust-lang.org/std/cmp/trait.Eq.html#impl-Eq-for-%26A
// [2] https://doc.rust-lang.org/std/hash/trait.Hash.html#impl-Hash-for-%26T
impl Borrow<[String]> for &ResourceName {
    fn borrow(&self) -> &[String] {
        self
    }
}

impl Hash for ResourceName {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        (**self).hash(hasher)
    }
}

/// Formats the column name as a string, with fields delimited by periods. Fields containing special
/// characters are escaped by backtick symbols:
///
/// ```
/// # use delta_kernel::resources::ResourceName;
/// assert_eq!(ColumnName::new(["a", "b.c", "d"]).to_string(), "a.`b.c`.d");
/// ```
///
/// Backticks inside escaped field names are themselves escaped by doubling:
///
/// ```
/// # use delta_kernel::resources::ResourceName;
/// assert_eq!(ColumnName::new(["a", "b.`c`.d", "e"]).to_string(), "a.`b.``c``.d`.e");
/// ```
///
/// The string representation is lossless, and can be parsed back into a `ColumnName` using
/// [`FromStr`]:
///
/// ```
/// # use delta_kernel::resources::ResourceName;
/// let colname = ColumnName::new(["a", "b.c", "d"]);
/// let parsed: ColumnName = colname.to_string().parse().unwrap();
/// assert_eq!(colname, parsed);
/// ```
///
/// [`FromStr`]: std::str::FromStr
impl Display for ResourceName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, s) in self.iter().enumerate() {
            use std::fmt::Write as _;

            // Don't emit a field separator before the first field
            if i > 0 {
                f.write_char(FIELD_SEPARATOR)?;
            }

            let digit_char = |c: char| c.is_ascii_digit();
            if s.is_empty() || s.starts_with(digit_char) || s.contains(|c| !is_simple_char(c)) {
                // Special situation detected. For safety, surround the field name with backticks
                // (with proper escaping if the field name itself contains backticks).
                f.write_char(FIELD_ESCAPE_CHAR)?;
                for c in s.chars() {
                    f.write_char(c)?;
                    if c == FIELD_ESCAPE_CHAR {
                        f.write_char(c)?; // escape the escape by doubling
                    }
                }
                f.write_char(FIELD_ESCAPE_CHAR)?;
            } else {
                // Simple field name -- emit it as-is
                f.write_str(s)?;
            }
        }
        Ok(())
    }
}

// Simple column names contain only simple chars, and do not need to be wrapped in backticks.
fn is_simple_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

fn drop_leading_whitespace(iter: &mut Peekable<impl Iterator<Item = char>>) {
    while iter.next_if(|c| c.is_whitespace()).is_some() {}
}

/// Parses a column name from a string. Field names are separated by dots. Whitespace between fields
/// is ignored. Field names enclosed in backticks may contain arbitrary characters, including
/// periods and spaces. To include a literal backtick in a field name, escape it by doubling, e.g.:
///
/// ```
/// # use delta_kernel::resources::ResourceName;
/// assert_eq!(ColumnName::new(["a", "b.`c`.d", "e"]).to_string(), "a.`b.``c``.d`.e");
/// ```
///
/// NOTE: Unlike the conversion from `ColumnName` to `String` and back, a conversion from `String`
/// to `ColumnName` and back may not exactly match the original string, if the latter included
/// whitespace or unnecessary field escapes, e.g.:
///
/// ```
/// # use delta_kernel::resources::ResourceName;
/// let parsed: ColumnName = " `a` . `b.``c``.d` . `e` ".parse().unwrap();
/// assert_eq!(parsed.to_string(), "a.`b.``c``.d`.e");
/// ```
impl std::str::FromStr for ResourceName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match parse_resource_name(&mut s.chars().peekable())? {
            (_, FieldEnding::NextColumn) => Err(Error::generic("Trailing comma in column name")),
            (col, _) => Ok(col),
        }
    }
}

type Chars<'a> = Peekable<std::str::Chars<'a>>;

// What comes after the end of the field we just parsed?
#[derive(PartialEq)]
enum FieldEnding {
    InputExhausted,
    NextField,
    NextColumn,
}

// These characters are remarkably hard to read. Names are a lot less bug-prone.
const FIELD_ESCAPE_CHAR: char = '`';
const FIELD_SEPARATOR: char = '.';
const COLUMN_SEPARATOR: char = ',';

fn parse_resource_name(chars: &mut Chars<'_>) -> Result<(ResourceName, FieldEnding)> {
    // Ambiguous case: The empty string `""`could reasonably parse as either `ColumnName::new([""])`
    // or `ColumnName::new([])`. However, `ColumnName::new([""]).to_string()` is `"[]"` and
    // `ColumnName::new([]).to_string()` is `""`, so we choose the latter because it produces a
    // lossless round trip from `ColumnName` to `String` and back. We also swallow a leading comma
    // to produce an empty column, so that the string "," parses as two empty columns.
    drop_leading_whitespace(chars);
    let mut ending = if chars.peek().is_none() {
        FieldEnding::InputExhausted
    } else if chars.next_if_eq(&COLUMN_SEPARATOR).is_some() {
        FieldEnding::NextColumn
    } else {
        FieldEnding::NextField
    };

    let mut path = vec![];
    while ending == FieldEnding::NextField {
        drop_leading_whitespace(chars);
        let field_name = match chars.next_if_eq(&FIELD_ESCAPE_CHAR) {
            Some(_) => parse_escaped_field_name(chars)?,
            None => parse_simple_field_name(chars)?,
        };

        // Figure out what's next (ignoring leading whitespace)
        ending = match chars.find(|c| !c.is_whitespace()) {
            None => FieldEnding::InputExhausted,
            Some(FIELD_SEPARATOR) => FieldEnding::NextField,
            Some(COLUMN_SEPARATOR) => FieldEnding::NextColumn,
            Some(other) => {
                return Err(Error::generic(format!(
                    "Invalid character {other:?} after field {field_name:?}",
                )))
            }
        };
        path.push(field_name);
    }
    Ok((ResourceName::new(path), ending))
}

/// Parses a simple field name, e.g. 'a.b.c'.
fn parse_simple_field_name(chars: &mut Chars<'_>) -> Result<String> {
    let mut name = String::new();
    let mut first = true;
    while let Some(c) = chars.next_if(|c| is_simple_char(*c)) {
        if first && c.is_ascii_digit() {
            return Err(Error::generic(format!(
                "Unescaped field name cannot start with a digit {c:?}"
            )));
        }
        name.push(c);
        first = false;
    }
    Ok(name)
}

/// Parses a field name escaped with backticks, e.g. "`ab``c``d`".
fn parse_escaped_field_name(chars: &mut Chars<'_>) -> Result<String> {
    let mut name = String::new();
    loop {
        match chars.next() {
            Some(FIELD_ESCAPE_CHAR) if chars.next_if_eq(&FIELD_ESCAPE_CHAR).is_none() => break,
            Some(c) => name.push(c),
            None => {
                return Err(Error::generic(format!(
                    "No closing {FIELD_ESCAPE_CHAR:?} after field {name:?}"
                )));
            }
        }
    }
    Ok(name)
}

/// Creates a nested column name whose field names are all simple column names (containing only
/// alphanumeric characters and underscores), delimited by dots. This macro is provided as a
/// convenience for the common case where the caller knows the column name contains only simple
/// field names and that splitting by periods is safe:
///
/// ```
/// # use delta_kernel::expressions::{column_name, ColumnName};
/// assert_eq!(column_name!("a.b.c"), ColumnName::new(["a", "b", "c"]));
/// ```
///
/// To avoid accidental misuse, the argument must be a string literal, so the compiler can validate
/// the safety conditions. Thus, the following uses would fail to compile:
///
/// ```fail_compile
/// # use delta_kernel::expressions::column_name;
/// let s = "a.b";
/// let name = column_name!(s); // not a string literal
/// ```
///
/// ```fail_compile
/// # use delta_kernel::expressions::simple_column_name;
/// let name = simple_column_name!("a b"); // non-alphanumeric character
/// ```
// NOTE: Macros are only public if exported, which defines them at the root of the crate. But we
// don't want it there. So, we export a hidden macro and pub use it here where we actually want it.
#[macro_export]
#[doc(hidden)]
macro_rules! __resource_name {
    ( $($name:tt)* ) => {
        $crate::resources::ResourceName::new($crate::delta_sharing_derive::parse_column_name!($($name)*))
    };
}
#[doc(inline)]
pub use __resource_name as resource_name;

/// Joins two column names together, when one or both inputs might be literal strings representing
/// simple (non-nested) column names. For example:
///
/// ```
/// # use delta_kernel::expressions::{column_name, joined_resource_name};
/// assert_eq!(joined_resource_name!("a.b", "c"), column_name!("a.b").join(&column_name!("c")))
/// ```
///
/// To avoid accidental misuse, at least one argument must be a string literal. Thus, the following
/// invocation would fail to compile:
///
/// ```fail_compile
/// # use delta_kernel::expressions::joined_resource_name;
/// let s = "s";
/// let name = joined_resource_name!(s, s);
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! __joined_resource_name {
    ( $left:literal, $right:literal ) => {
        $crate::__resource_name!($left).join(&$crate::__resource_name!($right))
    };
    ( $left:literal, $right:expr ) => {
        $crate::__resource_name!($left).join(&$right)
    };
    ( $left:expr, $right:literal) => {
        $left.join(&$crate::__resource_name!($right))
    };
    ( $($other:tt)* ) => {
        compile_error!("joined_resource_name!() requires at least one string literal input")
    };
}
#[doc(inline)]
pub use __joined_resource_name as joined_resource_name;

#[macro_export]
#[doc(hidden)]
macro_rules! __column_expr {
    ( $($name:tt)* ) => {
        $crate::expressions::Expression::from($crate::__resource_name!($($name)*))
    };
}
#[doc(inline)]
pub use __column_expr as column_expr;

#[macro_export]
#[doc(hidden)]
macro_rules! __joined_column_expr {
    ( $($name:tt)* ) => {
        $crate::expressions::Expression::from($crate::__joined_resource_name!($($name)*))
    };
}
#[doc(inline)]
pub use __joined_column_expr as joined_column_expr;
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod test {
    use super::*;
    use delta_sharing_derive::parse_column_name;

    impl ResourceName {
        fn empty() -> Self {
            Self::new(&[] as &[String])
        }
    }

    #[test]
    fn test_parse_column_name_macros() {
        assert_eq!(parse_column_name!("a"), ["a"]);

        assert_eq!(parse_column_name!("a"), ["a"]);
        assert_eq!(parse_column_name!("a.b"), ["a", "b"]);
        assert_eq!(parse_column_name!("a.b.c"), ["a", "b", "c"]);
    }

    #[test]
    fn test_column_name_macros() {
        let simple = resource_name!("x");
        let nested = resource_name!("x.y");

        assert_eq!(resource_name!("a"), ResourceName::new(["a"]));
        assert_eq!(resource_name!("a.b"), ResourceName::new(["a", "b"]));
        assert_eq!(resource_name!("a.b.c"), ResourceName::new(["a", "b", "c"]));

        assert_eq!(
            joined_resource_name!("a", "b"),
            ResourceName::new(["a", "b"])
        );
        assert_eq!(
            joined_resource_name!("a", "b"),
            ResourceName::new(["a", "b"])
        );

        assert_eq!(
            joined_resource_name!(simple, "b"),
            ResourceName::new(["x", "b"])
        );
        assert_eq!(
            joined_resource_name!(nested, "b"),
            ResourceName::new(["x", "y", "b"])
        );

        assert_eq!(
            joined_resource_name!("a", &simple),
            ResourceName::new(["a", "x"])
        );
        assert_eq!(
            joined_resource_name!("a", &nested),
            ResourceName::new(["a", "x", "y"])
        );
    }

    #[test]
    fn test_column_name_methods() {
        let simple = resource_name!("x");
        let nested = resource_name!("x.y");

        // path()
        assert_eq!(simple.path(), ["x"]);
        assert_eq!(nested.path(), ["x", "y"]);

        // into_inner()
        assert_eq!(simple.clone().into_inner(), ["x"]);
        assert_eq!(nested.clone().into_inner(), ["x", "y"]);

        // impl Deref
        let name: &[String] = &nested;
        assert_eq!(name, &["x", "y"]);

        // impl<A: Into<String>> FromIterator<A>
        let name: ResourceName = ["x", "y"].into_iter().collect();
        assert_eq!(name, nested);

        // impl FromIterator<ColumnName>
        let name: ResourceName = [&nested, &simple].into_iter().cloned().collect();
        assert_eq!(name, resource_name!("x.y.x"));

        // ColumnName::new
        let name = ResourceName::new([nested, simple]);
        assert_eq!(name, resource_name!("x.y.x"));

        let name = ResourceName::new(["x", "y"]);
        assert_eq!(name, resource_name!("x.y"));

        // ColumnName::into_iter()
        let name = resource_name!("x.y.z");
        let name = ResourceName::new(name);
        assert_eq!(name, resource_name!("x.y.z"));
    }

    #[test]
    fn test_column_name_from_str() {
        let cases = [
            ("", Some(ResourceName::empty())), // the ambiguous case!
            (".", Some(ResourceName::new(["", ""]))),
            ("  .  ", Some(ResourceName::new(["", ""]))),
            (" ", Some(ResourceName::empty())),
            ("0", None),
            (".a", Some(ResourceName::new(["", "a"]))),
            ("a.", Some(ResourceName::new(["a", ""]))),
            ("  a  .  ", Some(ResourceName::new(["a", ""]))),
            ("a..b", Some(ResourceName::new(["a", "", "b"]))),
            ("`a", None),
            ("a`", None),
            ("a`b`", None),
            ("`a`b", None),
            ("`a``b`", Some(ResourceName::new(["a`b"]))),
            ("  `a``b`  ", Some(ResourceName::new(["a`b"]))),
            ("`a`` b`", Some(ResourceName::new(["a` b"]))),
            ("a", Some(ResourceName::new(["a"]))),
            ("a0", Some(ResourceName::new(["a0"]))),
            ("`a`", Some(ResourceName::new(["a"]))),
            ("  `a`  ", Some(ResourceName::new(["a"]))),
            ("` `", Some(ResourceName::new([" "]))),
            ("  ` `  ", Some(ResourceName::new([" "]))),
            ("`0`", Some(ResourceName::new(["0"]))),
            ("`.`", Some(ResourceName::new(["."]))),
            ("`.`.`.`", Some(ResourceName::new([".", "."]))),
            ("` `.` `", Some(ResourceName::new([" ", " "]))),
            ("a.b", Some(ResourceName::new(["a", "b"]))),
            ("a b", None),
            ("a.`b`", Some(ResourceName::new(["a", "b"]))),
            ("`a`.b", Some(ResourceName::new(["a", "b"]))),
            ("`a`.`b`", Some(ResourceName::new(["a", "b"]))),
            ("`a`.`b`.`c`", Some(ResourceName::new(["a", "b", "c"]))),
            ("`a``.`b```", None),
            ("`a```.`b``", None),
            ("`a```.`b```", Some(ResourceName::new(["a`", "b`"]))),
            ("`a.`b``.c`", None),
            ("`a.``b`.c`", None),
            ("`a.``b``.c`", Some(ResourceName::new(["a.`b`.c"]))),
            ("a`.b``", None),
        ];
        for (input, expected_output) in cases {
            let output: Result<ResourceName> = input.parse();
            match (&output, &expected_output) {
                (Ok(output), Some(expected_output)) => {
                    assert_eq!(output, expected_output, "from {input}")
                }
                (Err(_), None) => {}
                _ => panic!("Expected {input} to parse as {expected_output:?}, got {output:?}"),
            }
        }
    }

    #[test]
    fn test_column_name_to_string() {
        let cases = [
            ("", ResourceName::empty()), // the ambiguous case!
            ("``.``", ResourceName::new(["", ""])),
            ("``.a", ResourceName::new(["", "a"])),
            ("a.``", ResourceName::new(["a", ""])),
            ("a.``.b", ResourceName::new(["a", "", "b"])),
            ("a", ResourceName::new(["a"])),
            ("a0", ResourceName::new(["a0"])),
            ("`a `", ResourceName::new(["a "])),
            ("` `", ResourceName::new([" "])),
            ("`0`", ResourceName::new(["0"])),
            ("`.`", ResourceName::new(["."])),
            ("`.`.`.`", ResourceName::new([".", "."])),
            ("` `.` `", ResourceName::new([" ", " "])),
            ("a.b", ResourceName::new(["a", "b"])),
            ("a.b.c", ResourceName::new(["a", "b", "c"])),
            ("a.`b.c`.d", ResourceName::new(["a", "b.c", "d"])),
            ("`a```.`b```", ResourceName::new(["a`", "b`"])),
        ];
        for (expected_output, input) in cases {
            let output = input.to_string();
            assert_eq!(output, expected_output);

            let parsed: ResourceName = output.parse().expect(&output);
            assert_eq!(parsed, input);
        }

        // Ensure unnecessary escaping and whitespace is tolerated
        let cases = [
            ("  `a`  ", "a", ResourceName::new(["a"])),
            ("  `a0`  ", "a0", ResourceName::new(["a0"])),
            ("  `a`  .  `b`  ", "a.b", ResourceName::new(["a", "b"])),
        ];
        for (input, expected_output, expected_parsed) in cases {
            let parsed: ResourceName = input.parse().unwrap();
            assert_eq!(parsed, expected_parsed);
            assert_eq!(parsed.to_string(), expected_output);
        }
    }

    #[test]
    fn test_parse_column_name_list() {
        let cases = [
            ("", Some(vec![])),
            (
                "  ,  ",
                Some(vec![ResourceName::empty(), ResourceName::empty()]),
            ),
            ("  a  ", Some(vec![resource_name!("a")])),
            (
                "  ,  a  ",
                Some(vec![ResourceName::empty(), resource_name!("a")]),
            ),
            (
                "  a  ,  ",
                Some(vec![resource_name!("a"), ResourceName::empty()]),
            ),
            (
                "a  ,  b",
                Some(vec![resource_name!("a"), resource_name!("b")]),
            ),
            ("`a, b`", Some(vec![ResourceName::new(["a, b"])])),
            (
                "a.b, c",
                Some(vec![resource_name!("a.b"), resource_name!("c")]),
            ),
            (
                "`a.b`, c",
                Some(vec![ResourceName::new(["a.b"]), resource_name!("c")]),
            ),
            (
                "`a.b`, c",
                Some(vec![ResourceName::new(["a.b"]), resource_name!("c")]),
            ),
        ];
        for (input, expected_output) in cases {
            let output = ResourceName::parse_column_name_list(input);
            match (&output, &expected_output) {
                (Ok(output), Some(expected_output)) => {
                    assert_eq!(output, expected_output, "from \"{input}\"")
                }
                (Err(_), None) => {}
                _ => panic!("Expected {input} to parse as {expected_output:?}, got {output:?}"),
            }
        }
    }
}
