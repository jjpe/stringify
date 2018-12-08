/// Indentation-aware printing.

mod error;
mod newline;
#[macro_use] mod styles;
mod stringify;

pub use crate::styles::{Style, Styles};
pub use crate::newline::Newline;
use std::collections::{HashMap};
use std::hash::Hash;


pub trait Stringify {
    /// Stringify a datum. To achieve this, there are a number of
    /// knobs that can be twisted to achieve the desired result:
    /// - `parent_init` and `parent_rest` are the Styles used for `self`:
    ///   + `parent_init` is used at the start of stringifying `self`
    ///   + `parent_rest` is used everywhere else during the
    ///     stringification of `self`
    /// - `child_init` and `child_rest` are auxiliary Styles
    ///   + `child_init` is used at the start of stringifying some
    ///     internal component of `self`
    ///   + `child_rest` is used everywhere else during the
    ///     stringification of `self`
    /// - `buffer` is the buffer to stringify to. In order to keep
    ///     stringification as cheap as possible, a `&mut` to the buffer is
    ///     passed in rather than allocating and returning the buffer.
    fn stringify(&self,
                 parent_init: Style,
                 parent_rest: Style,
                 child_init: Style,
                 child_rest: Style,
                 buffer: &mut String);

    /// Convenience method that is an allocating version of `.stringify()`.
    fn stringify_new(&self,
                     parent_init: Style,
                     parent_rest: Style,
                     child_init: Style,
                     child_rest: Style) -> String {
        let mut buffer = String::new();
        self.stringify(parent_init, parent_rest, child_init, child_rest, &mut buffer);
        buffer
    }

    /// Convenience method to help stringify an enum variant / struct field.
    fn stringify_field<V>(&self,
                          name: &str,
                          value: &V,
                          name_style: Style,
                          value_style: Style,
                          buffer: &mut String)
    where V: Stringify {
        self.indent(  name_style, buffer);
        buffer.push_str(name);
        buffer.push_str("=");
        value.stringify(value_style, value_style, value_style, value_style, buffer);
    }

    fn stringify_primitive(&self, buffer: &mut String) {
        self.stringify(
            Style::default(), // unused
            Style::default(), // unused
            Style::default(), // unused
            Style::default(), // unused
            buffer
        )
    }

    fn stringify_primitive_new(&self) -> String {
        let mut buffer = String::new();
        self.stringify_primitive(&mut buffer);
        buffer
    }

    /// If `style.newline` == `Newline::Add`, write a newline.
    /// Then, regardless of whether or not a newline was written,
    /// apply `style.indent` exactly `style.indent_level` times.
    fn indent(&self, style: Style, buffer: &mut String) {
        if style.newline == Newline::Add { buffer.push_str("\n"); }
        for _ in 0 .. style.indent_level {
            buffer.push_str(style.indent);
        }
    }
}

impl<K, V> Stringify for HashMap<K, V>
where K: Stringify + Eq + Hash,
      V: Stringify {
    fn stringify(&self,
                 parent_init: Style,
                 parent_rest: Style,
                 key_style: Style,
                 value_style: Style,
                 buffer: &mut String) {
        if self.is_empty() {
            buffer.push_str("HashMap {}");
            return;
        }
        self.indent(parent_init, buffer);
        buffer.push_str("HashMap {");
        for (key, value) in self.iter() {
            key.stringify(key_style, key_style, key_style, key_style, buffer);
            buffer.push_str(" : ");
            value.stringify(value_style, value_style, value_style, value_style, buffer);
            buffer.push_str(",");
        }
        self.indent(Style::standard(Newline::Add, parent_rest.indent_level + 1), buffer);
        buffer.push_str("}");
    }
}

impl<T> Stringify for Vec<T>
where T: Stringify {
    fn stringify(&self,
                 parent_init: Style,
                 parent_rest: Style,
                 elt_init: Style,
                 elt_rest: Style,
                 buffer: &mut String) {
        self.indent(parent_init, buffer);
        if self.is_empty() {
            buffer.push_str("Vec []");
            return;
        }
        buffer.push_str("Vec [");
        for item in self.iter() {
            self.indent(parent_rest + 1, buffer);
            item.stringify(
                elt_init,
                elt_rest,
                Style::default(), // unused
                Style::default(), // unused
                buffer
            );
            buffer.push_str(",");
        }
        self.indent(parent_rest, buffer);
        buffer.push_str("]");
    }
}

impl<T, E> Stringify for Result<T, E>
where T: Stringify,
      E: Stringify {
    fn stringify(&self,
                 parent_init: Style,
                 parent_rest: Style,
                 child_init: Style,
                 child_rest: Style,
                 buffer: &mut String) {
        self.indent(parent_init, buffer);
        match self {
            Ok(ok) => {
                buffer.push_str("Ok(");
                ok.stringify(parent_init, parent_rest, child_init, child_rest, buffer);
                buffer.push_str(")");
            },
            Err(err) => {
                buffer.push_str("Err(");
                err.stringify(parent_init, parent_rest, child_init, child_rest, buffer);
                buffer.push_str(")");
            },
        }
    }
}

impl Stringify for bool {
    fn stringify(&self, _: Style, _: Style, _: Style, _: Style, buffer: &mut String) {
        buffer.push_str(&format!("{}", self));
    }
}

impl Stringify for String {
    fn stringify(&self, _: Style, _: Style, _: Style, _: Style, buffer: &mut String) {
        buffer.push_str(&self);
    }
}

impl<'s> Stringify for &'s str {
    fn stringify(&self, _: Style, _: Style, _: Style, _: Style, buffer: &mut String) {
        buffer.push_str(self);
    }
}

impl Stringify for usize {
    fn stringify(&self, _: Style, _: Style, _: Style, _: Style, buffer: &mut String) {
        buffer.push_str(&format!("{}", self));
    }
}

impl Stringify for u8 {
    fn stringify(&self, _: Style, _: Style, _: Style, _: Style, buffer: &mut String) {
        buffer.push_str(&format!("{}", self));
    }
}

impl Stringify for u16 {
    fn stringify(&self, _: Style, _: Style, _: Style, _: Style, buffer: &mut String) {
        buffer.push_str(&format!("{}", self));
    }
}
impl Stringify for u32 {
    fn stringify(&self, _: Style, _: Style, _: Style, _: Style, buffer: &mut String) {
        buffer.push_str(&format!("{}", self));
    }
}
impl Stringify for u64 {
    fn stringify(&self, _: Style, _: Style, _: Style, _: Style, buffer: &mut String) {
        buffer.push_str(&format!("{}", self));
    }
}
impl Stringify for u128 {
    fn stringify(&self, _: Style, _: Style, _: Style, _: Style, buffer: &mut String) {
        buffer.push_str(&format!("{}", self));
    }
}

impl Stringify for isize {
    fn stringify(&self, _: Style, _: Style, _: Style, _: Style, buffer: &mut String) {
        buffer.push_str(&format!("{}", self));
    }
}

impl Stringify for i8 {
    fn stringify(&self, _: Style, _: Style, _: Style, _: Style, buffer: &mut String) {
        buffer.push_str(&format!("{}", self));
    }
}

impl Stringify for i16 {
    fn stringify(&self, _: Style, _: Style, _: Style, _: Style, buffer: &mut String) {
        buffer.push_str(&format!("{}", self));
    }
}
impl Stringify for i32 {
    fn stringify(&self, _: Style, _: Style, _: Style, _: Style, buffer: &mut String) {
        buffer.push_str(&format!("{}", self));
    }
}
impl Stringify for i64 {
    fn stringify(&self, _: Style, _: Style, _: Style, _: Style, buffer: &mut String) {
        buffer.push_str(&format!("{}", self));
    }
}
impl Stringify for i128 {
    fn stringify(&self, _: Style, _: Style, _: Style, _: Style, buffer: &mut String) {
        buffer.push_str(&format!("{}", self));
    }
}



impl Stringify for Style {
    fn stringify(&self,
                 parent_init: Style,
                 parent_rest: Style,
                 _child_init: Style,
                 _child_rest: Style,
                 buffer: &mut String) {
        self.indent(parent_init, buffer);
        buffer.push_str("Style {");

        self.stringify_field(
            "newline",
            &self.newline,
            Style { newline: Newline::Add,  indent_level: 0, indent: Style::INDENT },
            Style { newline: Newline::Omit, indent_level: 0, indent: Style::INDENT },
            buffer
        );

        self.stringify_field(
            "indent_level",
            &self.indent_level,
            Style { newline: Newline::Add,  indent_level: 0, indent: Style::INDENT },
            Style { newline: Newline::Omit, indent_level: 0, indent: Style::INDENT },
            buffer
        );

        self.indent(parent_rest, buffer);
        buffer.push_str("}");
    }
}

impl Stringify for Newline {
    fn stringify(&self, style: Style, _: Style, _: Style, _: Style, buffer: &mut String) {
        self.indent(style, buffer);
        buffer.push_str(&format!("Newline::{:?}", self));
    }
}






// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
