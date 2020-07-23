use std::collections::HashMap;

struct Context<'a> {
    indent: u32,
    output: &'a String,
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

    fn generate(self, context: Context) -> String {
        let mut outout = context.output;
        for (key, value) in self.value.iter() {
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

fn generate(json: Object) -> String {
    let mut output = String::from("");
    output.push('{');
    json.generate(Context {
        indent: 0,
        output: &output,
    });
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
