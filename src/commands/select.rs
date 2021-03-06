use crate::errors::ShellError;
use crate::object::base::select_fields;
use crate::object::Value;
use crate::prelude::*;

pub fn select(args: CommandArgs) -> Result<OutputStream, ShellError> {
    if args.args.is_empty() {
        return Err(ShellError::string("select requires a field"));
    }

    let fields: Result<Vec<String>, _> = args.args.iter().map(|a| a.as_string()).collect();
    let fields = fields?;

    let objects = args
        .input
        .map(move |item| Value::Object(select_fields(&item, &fields)))
        .map(|item| ReturnValue::Value(item));

    let stream = Pin::new(Box::new(objects));
    Ok(stream)
}
