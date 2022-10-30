use rhai::def_package;
use rhai::plugin::*;
use rhai::{EvalAltResult, Position, ImmutableString};

def_package! {
    /// Package for read and write env variables
    pub {{module | capitalize }}Package(lib) {
        combine_with_exported_module!(lib, "{{module}}", {{module}}_functions);
    }
}

#[export_module]
mod {{module}}_functions{
    #[rhai_fn(name = "first_function_name", return_raw)]
    pub fn first_function_name(key: &str) -> Result<ImmutableString, Box<EvalAltResult>> {
        if key.is_empty() {
            Err(EvalAltResult::ErrorVariableNotFound(
                format!("Env variable not found: {:?}", key),
                Position::NONE,
            )
            .into())
        } else {
            Ok("ok".into())
        }
    }
}