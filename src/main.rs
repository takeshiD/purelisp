use std::fmt::{self, Display, Formatter};

enum Value {
    INT(i32),
    FLOAT(f32),
    STRING(String),
    NIL,
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Value::INT(v) => write!(f, "{v}"),
            Value::FLOAT(v) => write!(f, "{v}"),
            Value::STRING(v) => write!(f, "\"{v}\""),
            Value::NIL => write!(f, "nil"),
        }
    }
}

struct Node {
    value: Value,
}
impl Node {
    fn new(value: Value) -> Node {
        Node { value }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

struct Cons {
    x: Box<Node>,
    y: Box<Node>,
}
impl Display for Cons {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({} . {})", self.x.value, self.y.value)
    }
}

impl Cons {
    fn new(x: Box<Node>, y: Box<Node>) -> Cons {
        Cons { x, y }
    }
}

fn main() {
    let list = Cons::new(
        Box::new(Node::new(Value::INT(1))),
        Cons::new(
            Box::new(Node::new(Value::INT(2))),
            Box::new(Node::new(Value::NIL)),
        ));
    println!("{}", list);
}
