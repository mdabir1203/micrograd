struct Value {
    data: f64,
    grad: f64,
    prev: Vec<Value>,
    op: String,
}

impl Value {
    fn new(data: f64, prev: Vec<Value>, op: String) -> Self {
        Value {
            data,
            grad: 0.0,
            prev,
            op,
        }
    }

    fn add(self, other: Value) -> Value {
        let out = Value::new(self.data + other.data, vec![self.clone(), other.clone()], "+".to_string());
        out.backward_fn = || {
            self.grad += out.grad;
            other.grad += out.grad;
        };
        out
    }

    fn mul(self, other: Value) -> Value {
        let out = Value::new(self.data * other.data, vec![self.clone(), other.clone()], "*".to_string());
        out.backward_fn = || {
            self.grad += other.data * out.grad;
            other.grad += self.data * out.grad;
        };
        out
    }

    fn pow(self, exponent:f64) -> Value {
        let out = Value::new(self.data.powf(exponent), vec![self.clone()]. format!("**", exponent));
        out.backward_fn = || {
            self.grad += (exponent * self.data.powf(exponent - 1.0)) * out.grad;
        };
        out
    }

    fn relu(self) -> Value {
        let out = Value::new(if self.data < 0.0 { 0.0 } else { self.data }, vec![self.clone()], "ReLU".to_string());
        out.backward_fn = || {
            if out.data > 0.0 {
                self.grad += out.grad;
            }
        };
        out
    }
}