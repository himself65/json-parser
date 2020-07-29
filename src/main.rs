use std::collections::HashMap;
use std::fmt::Display;

pub trait JsonWriter {
    fn indent(&mut self) -> &mut Self;
    fn deindent(&mut self) -> &mut Self;
    fn advance(&mut self) -> &mut Self;
    fn write_one_space(&mut self) -> &mut Self;
    fn write_new_line(&mut self) -> &mut Self;
    fn json_start(&mut self) -> &mut Self;
    fn json_end(&mut self) -> &mut Self;
    fn json_objectstart(&mut self, value: &str) -> &mut Self;
    fn json_arraystart(&mut self, value: &str) -> &mut Self;
    fn json_objectend(&mut self) -> &mut Self;
    fn json_arrayend(&mut self) -> &mut Self;
    fn json_keyvalue(&mut self, key: &str, value: &str) -> &mut Self;
}

enum JSONState {
    ObjectStart,
    AfterValue,
}

pub struct Context {
    indent: usize,
    compact: bool,
    state: JSONState,
    pub output: String,
}

impl JsonWriter for Context {
    fn indent(&mut self) -> &mut Self {
        self.indent += 2;
        self
    }

    fn deindent(&mut self) -> &mut Self {
        self.indent -= 2;
        self
    }

    fn advance(&mut self) -> &mut Self {
        if self.compact == false {
            self.write(" ".repeat(self.indent).as_str());
        }
        self
    }

    fn write_one_space(&mut self) -> &mut Self {
        if self.compact == false {
            self.write(" ");
        }
        self
    }

    fn write_new_line(&mut self) -> &mut Self {
        if self.compact == false {
            self.write("\n");
        }
        self
    }

    fn json_start(&mut self) -> &mut Self {
        match self.state {
            JSONState::AfterValue => {
                self.write(",");
            }
            JSONState::ObjectStart => {}
        };
        self.write_new_line().advance().write("{").indent();
        self.state = JSONState::ObjectStart;
        self
    }

    fn json_end(&mut self) -> &mut Self {
        self.write_new_line().deindent().advance();
        self.write("}");
        self
    }

    fn json_objectstart(&mut self, value: &str) -> &mut Self {
        match self.state {
            JSONState::AfterValue => {
                self.write(",");
            }
            JSONState::ObjectStart => {}
        };
        self.write_new_line()
            .advance()
            .write(value)
            .write(":")
            .write_one_space()
            .write("{")
            .indent();
        self.state = JSONState::ObjectStart;
        self
    }

    fn json_arraystart(&mut self, value: &str) -> &mut Self {
        match self.state {
            JSONState::AfterValue => {
                self.write(",");
            }
            JSONState::ObjectStart => {}
        };
        self.write_new_line()
            .advance()
            .write(value)
            .write(":")
            .write_one_space()
            .write("[")
            .indent();
        self
    }

    fn json_objectend(&mut self) -> &mut Self {
        self.write_new_line().deindent().advance().write("}");
        if self.indent == 0 {
            self.write("\n");
        }
        self.state = JSONState::AfterValue;
        self
    }

    fn json_arrayend(&mut self) -> &mut Self {
        self.write_new_line().deindent().indent().write("]");
        self.state = JSONState::AfterValue;
        self
    }

    fn json_keyvalue(&mut self, key: &str, value: &str) -> &mut Self {
        match self.state {
            JSONState::AfterValue => { self.write(","); },
            JSONState::ObjectStart => {}
        };
        self.write_new_line()
            .advance()
            .write(key)
            .write(":")
            .write_new_line()
            .write(value);
        self.state = JSONState::AfterValue;
        self
    }
}

impl Context {
    fn new() -> Context {
        Context {
            indent: 0,
            compact: false,
            state: JSONState::ObjectStart,
            output: String::from(""),
        }
    }
    fn write(&mut self, string: &str) -> &mut Self {
        self.output.push_str(string);
        self
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

        outout
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
