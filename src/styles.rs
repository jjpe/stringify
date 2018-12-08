use crate::error::{StringifyError, StringifyResult};
use crate::newline::Newline;
use std::collections::BTreeMap;
use std::ops;

#[macro_export]
macro_rules! styles {
    (
        $($key:expr => $value:expr),*
    ) => {{
        use std::collections::BTreeMap;
        #[allow(unused_mut)] let mut btmap = BTreeMap::new();
        $(
            btmap.insert($key, $value);
        )*
            $crate::styles::Styles::new(btmap)
    }};
}

pub struct Styles(BTreeMap<&'static str, Style>);

impl Styles {
    pub fn new(map: BTreeMap<&'static str, Style>) -> Self {
        Styles(map)
    }

    pub fn get(&self, name: &'static str) -> StringifyResult<Style> {
        match self.0.get(name) {
            Some(style) => Ok(*style),
            None => Err(StringifyError::StyleNotFound { name })?,
        }
    }
}



#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Style {
    /// The policy for printing a newline.
    pub newline: Newline,

    /// The indentation level.
    pub indent_level: usize,

    pub indent: &'static str,
}

impl Style {
    pub const INDENT: &'static str = "    "; // 4 spaces

    pub fn standard(newline: Newline, indent_level: usize) -> Self {
        Self {
            newline: newline,
            indent_level: indent_level,
            indent: Self::INDENT,
        }
    }

    #[inline(always)]
    pub fn unused() -> Self { Self::default() }

    pub fn with_newline(&self, newline: Newline) -> Self {
        Self {
            newline: newline,
            indent_level: self.indent_level,
            indent: self.indent,
        }
    }

    pub fn with_indent_level(&self, indent_level: usize) -> Self {
        Self {
            newline: self.newline,
            indent_level: indent_level,
            indent: self.indent,
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Style {
            newline: Newline::Omit,
            indent_level: 0,
            indent: Self::INDENT,
        }
    }
}

impl ops::Add<usize> for Style {
    type Output = Style;

    fn add(self, rhs: usize) -> Self::Output {
        Style {
            newline: self.newline,
            indent_level: self.indent_level + rhs,
            indent: self.indent,
        }
    }
}

impl ops::Add<Style> for Style {
    type Output = Style;

    fn add(self, rhs: Style) -> Self::Output {
        Style {
            newline: self.newline,
            indent_level: self.indent_level + rhs.indent_level,
            indent: self.indent,
        }
    }
}

impl ops::Sub<usize> for Style {
    type Output = Style;

    fn sub(self, rhs: usize) -> Self::Output {
        Style {
            newline: self.newline,
            indent_level: self.indent_level - rhs,
            indent: self.indent,
        }
    }
}

impl ops::Sub<Style> for Style {
    type Output = Style;

    fn sub(self, rhs: Style) -> Self::Output {
        Style {
            newline: self.newline,
            indent_level: self.indent_level - rhs.indent_level,
            indent: self.indent,
        }
    }
}
