use nu_protocol::{CustomValue, ShellError, Span, Value};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Color {
    red: u16,
    green: u16,
    blue: u16,
}

impl CustomValue for Color {
    fn clone_value(&self, span: Span) -> Value {
        let cloned = Color {
            red: self.red,
            green: self.green,
            blue: self.blue,
        };

        Value::CustomValue {
            val: Box::new(cloned),
            span,
        }
    }

    fn value_string(&self) -> String {
        String::from(format!("({}%, {}%, {}%)",
                                self.red    / u16::MAX * 100,
                                self.green  / u16::MAX * 100,
                                self.blue   / u16::MAX * 100
                            ))
    }
    
    fn to_base_value(&self, span: Span) -> Result<Value, ShellError> {
        let val: Vec<u8> = Vec::from([ self.red as u8, self.green as u8, self.blue as u8 ]);

        Ok(Value::Binary { val, span})
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn follow_path_string(&self, _column_name: String, span: Span) -> Result<Value, ShellError> {
        match _column_name.as_str() {
            "red" => Ok(Value::Int {    val: self.red   as i64, span }),
            "green" => Ok(Value::Int {  val: self.green as i64, span }),
            "blue" => Ok(Value::Int {   val: self.blue  as i64, span }),
            _ => Err(ShellError::GenericError(
                "Only possible options: red, green, blue".into(),
                "".into(),
                Some(span),
                None,
                Vec::new(),
            ))
        }
    }

    fn typetag_name(&self) -> &'static str {
        "Color"
    }

    fn typetag_deserialize(&self) {
        unimplemented!("typetag_deserialize")
    }

}