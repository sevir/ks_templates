use rhai::{packages::Package, Engine, EvalAltResult, ImmutableString};


#[test]
fn test_{{module}}() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    //engine.register_global_module(EnvPackage::new().as_shared_module());

    //let variable_result = engine.eval::<rhai::ImmutableString>("{{module}}(\"TEST\")")?;

    //let compare: ImmutableString = "hello".into();
    //assert!(compare == variable_result);

    Ok(())
}
