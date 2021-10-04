
pub mod read {
    use serde_json as json;
    use crate::error::IPyNbError;


    pub fn normalize_output(output: &json::Value) -> Result<json::Value, IPyNbError> {
        let new_output: json::Value;

        if output.is_object() {
            let output_type: &str = &output["output_type"].as_str().unwrap(); // unrap temporary (probably)

            match output_type {
                "stream" => {new_output = output_stream(output)?;},
                "execute_result" => {new_output = output_result(output)?;},
                "error" => {new_output = output_error(output)?;},
                _ => {return Err(IPyNbError::BadJSONValue);}
            };
        }
        
        else {
            return Err(IPyNbError::BadJSONValue);
        }

         Ok(new_output)
    }

    fn output_stream(output: &json::Value) -> Result<json::Value, IPyNbError> {
        let new_output: json::Value = json::json!({
            "name": output["name"],
            "output_type": output["output_type"],
            "text": output["text"],
            "error": false,
            "error_value": json::Value::Null,
        });

        Ok(new_output)
    }

    fn output_result(output: &json::Value) -> Result<json::Value, IPyNbError> {
        let new_output: json::Value = json::json!({
            "name": "result",
            "output_type": output["output_type"],
            "text": output["data"]["text/plain"],
            "error": false,
            "error_value": json::Value::Null,
        });

        Ok(new_output)
    }

    fn output_error(output: &json::Value) -> Result<json::Value, IPyNbError> {
        let new_output: json::Value = json::json!({
            "name": output["ename"],
            "output_type": output["output_type"],
            "text": output["traceback"],
            "error": true,
            "error_value": output["evalue"],
        });

        Ok(new_output)
    }
}


pub fn vec_to_string(vec: &Vec<String>) -> String {
    let mut string = String::new();

    for line in vec {
        string.push_str(&line);
    }
    string
}