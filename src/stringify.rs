use crate::{Newline, Style, Styles};
use crate::error::{StringifyResult};
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use std::io::Write;


pub trait Stringify2 {
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
    /// - `buf` is the buffer to which to write the stringification.
    fn stringify<W>(&self, buf: &mut W, styles: &Styles) -> StringifyResult<()>
    where W: Write;

    fn stringify_new(&self, styles: &Styles) -> StringifyResult<String> {
        let mut buf = String::new();
        self.stringify(unsafe { buf.as_mut_vec() }, styles)?;
        Ok(buf)
    }

    /// Convenience method to help stringify an enum variant / struct field.
    fn stringify_field<V, W>(&self,
                             buf: &mut W,
                             styles: &Styles,
                             name: &str,
                             value: &V) -> StringifyResult<()>
    where V: Stringify2,
          W: Write {
        let name_style: Style = styles.get("name")?;
        self.indent(buf, name_style)?;
        buf.write_all(name.as_bytes())?;
        buf.write_all("=".as_bytes())?;
        value.stringify(buf, styles)?;
        Ok(())
    }

    /// Convenience method to help stringify a primitive.
    fn stringify_primitive<W>(&self, buf: &mut W) -> StringifyResult<()>
    where W: Write {
        self.stringify(buf, &styles! { })
    }

    fn stringify_primitive_new(&self) -> StringifyResult<String> {
        let mut buf = String::new();
        self.stringify_primitive(unsafe { buf.as_mut_vec() })?;
        Ok(buf)
    }

    /// If `style.newline` == `Newline::Add`, write a newline.
    /// Then, regardless of whether or not a newline was written,
    /// apply `style.indent` exactly `style.indent_level` times.
    fn indent<W>(&self, buf: &mut W, style: Style) -> StringifyResult<()>
    where W: Write {
        if style.newline == Newline::Add { buf.write_all("\n".as_bytes())?; }
        for _ in 0 .. style.indent_level {
            buf.write_all(style.indent.as_bytes())?;
        }
        Ok(())
    }
}


impl<K, V> Stringify2 for HashMap<K, V>
where K: Stringify2 + Eq + Hash,
      V: Stringify2 {
    fn stringify<W>(&self, buf: &mut W, styles: &Styles) -> StringifyResult<()>
    where W: Write {
        if self.is_empty() {
            buf.write_all("HashMap {}".as_bytes())?;
            return Ok(());
        }
        let start: Style = styles.get("start")?;
        self.indent(buf, start)?;
        buf.write_all("HashMap {".as_bytes())?;
        for (key, value) in self.iter() {
            key.stringify(buf, &styles! {
                "key" => Style::standard(Newline::Add, start.indent_level + 1)
            })?;
            buf.write_all(" : ".as_bytes())?;
            value.stringify(buf, &styles! {
                "value" => Style::standard(Newline::Add, start.indent_level + 1)
            })?;
            buf.write_all(",".as_bytes())?;
        }
        self.indent(buf, Style::standard(
            Newline::Add,
            styles.get("end")?.indent_level + 1
        ))?;
        buf.write_all("}".as_bytes())?;
        Ok(())
    }
}

impl<K, V> Stringify2 for BTreeMap<K, V>
where K: Stringify2 + Eq + Hash,
      V: Stringify2 {
    fn stringify<W>(&self, buf: &mut W, styles: &Styles) -> StringifyResult<()>
    where W: Write {
        if self.is_empty() {
            buf.write_all("BTreeMap {}".as_bytes())?;
            return Ok(());
        }
        let start: Style = styles.get("start")?;
        self.indent(buf, start)?;
        buf.write_all("BTreeMap {".as_bytes())?;
        for (key, value) in self.iter() {
            key.stringify(buf, &styles! {
                "key" => Style::standard(Newline::Add, start.indent_level + 1)
            })?;
            buf.write_all(" : ".as_bytes())?;
            value.stringify(buf, &styles! {
                "value" => Style::standard(Newline::Add, start.indent_level + 1)
            })?;
            buf.write_all(",".as_bytes())?;
        }
        self.indent(buf, Style::standard(
            Newline::Add,
            styles.get("end")?.indent_level + 1
        ))?;
        buf.write_all("}".as_bytes())?;
        Ok(())
    }
}

impl<T> Stringify2 for Vec<T>
where T: Stringify2 {
    fn stringify<W>(&self, buf: &mut W, styles: &Styles) -> StringifyResult<()>
    where W: Write {
        if self.is_empty() {
            buf.write_all("Vec []".as_bytes())?;
            return Ok(());
        }
        let end: Style = styles.get("end")?;
        self.indent(buf, styles.get("start")?)?;
        buf.write_all("Vec [".as_bytes())?;
        for item in self.iter() {
            self.indent(buf, end + 1)?;
            item.stringify(buf, styles)?;
            buf.write_all(",".as_bytes())?;
        }
        self.indent(buf, end)?;
        buf.write_all("]".as_bytes())?;
        Ok(())
    }
}
