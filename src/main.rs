use std::collections::HashMap;

pub trait JsonWriter {
    fn indent(&mut self);
    fn de_indent(&mut self);
    fn advance(&mut self);
    fn write_one_space(&mut self);
    fn write_new_line(&mut self);
}

enum JSONState {
    ObjectStart,
    AfterValue,
}

pub struct Context<'a> {
    indent: usize,
    compact: bool,
    state: JSONState,
    pub output: &'a String,
}

impl JsonWriter for Context {
    fn indent(&mut self) {
        self.indent += 2
    }

    fn de_indent(&mut self) {
        self.indent -= 2
    }

    fn advance(&mut self) {
        if self.compact == false {
            self.output.push_str(" ".repeat(self.indent).as_str())
        }
    }

    fn write_one_space(&mut self) {
        if self.compact == false {
            self.output.push(' ')
        }
    }

    fn write_new_line(&mut self) {
        if self.compact == false {
            self.output.push('\n')
        }
    }
}

impl Context {
    fn new() -> Context {
        Context {
            indent: 0,
            compact: false,
            state: JSONState::ObjectStart,
            output: &"".to_string(),
        }
    }
    fn write(mut self, string: &str) {
        self.output.push_str(string)
    }
}

struct Object {
    name: String,
    value: HashMap<String, ValueType>,
}

impl Object {
    fn new(name: &str) -> Object {
        Object {
            name: String::from(name),
            value: Default::default(),
        }
    }

    fn add(mut self, key: &str, value: ValueType) -> Self {
        self.value.insert(String::from(key), value);
        self
    }

    fn generate(self, mut context: Context) -> String {
        let mut outout = context.output;
        for (key, value) in self.value.iter() {
            outout.push_str("".repeat(context.indent).as_str());
            context.indent += 2;
            match value {
                ValueType::Array(array) => {}
                ValueType::String(string) => {}
                ValueType::Object(object) => {}
                ValueType::Number(number) => {}
            }
        }

        outout.into()
    }
}

fn generate(json: Object, compact: bool) -> String {
    let mut output = String::from("");
    output.push('{');
    let mut context = Context::new();
    context.compact = compact;
    json.generate(context);
    output.push('}');
    output
}

enum ValueType {
    Array(Vec<ValueType>),
    String(String),
    Number(i32),
    Object(Object),
}

fn main() {
    let mut json = Object::new("global");
    json.add("1", ValueType::String(String::from("123")));
}

#[cfg(test)]
mod tests {
    use crate::{Context, JsonWriter};

    #[test]
    fn test_for_writer() {
        let writer = Context::new();
    }
}
