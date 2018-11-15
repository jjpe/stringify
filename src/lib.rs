/// Indentation-aware printing.

use std::collections::HashMap;
use std::hash::Hash;

pub trait Stringify {
    const INDENTATION: &'static str = "    ";

    fn stringify(&self, indent_level: usize, buffer: &mut String) {
        self.indent(indent_level, buffer);
        self.stringify_skip_initial(indent_level, buffer);
    }

    /// Stringify but skip the initial indentation
    fn stringify_skip_initial(&self, indent_level: usize, buffer: &mut String);

    fn indent(&self, times: usize, buffer: &mut String) {
        for _ in 0..times {
            buffer.push_str(Self::INDENTATION);
        }
    }
}

impl<K, V> Stringify for HashMap<K, V>
where K: Stringify + Eq + Hash,
      V: Stringify {
    fn stringify_skip_initial(&self, indent_level: usize, buffer: &mut String) {
        buffer.push_str(&format!("HashMap {{\n"));
        for (key, value) in self.iter() {
            key.stringify(indent_level + 1, buffer);
            buffer.push_str(" : ");
            value.stringify_skip_initial(indent_level + 1, buffer);
            buffer.push_str(",\n");
        }
        self.indent(indent_level, buffer);
        buffer.push_str("}}");
    }
}

impl<T> Stringify for Vec<T>
where T: Stringify {
    fn stringify_skip_initial(&self, indent_level: usize, buffer: &mut String) {
        buffer.push_str(&format!("Vec [\n"));
        for item in self.iter() {
            item.stringify(indent_level + 1, buffer);
            buffer.push_str(",\n");
        }
        self.indent(indent_level, buffer);
        buffer.push_str("]");
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
