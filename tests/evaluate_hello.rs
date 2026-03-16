use jscore::prelude::*;

#[test]
fn evaluate_hello() {
    let group = JsContextGroup::new();
    let global = group.create_global_context();
    let ctx = global.as_context();

    let script = JsString::new("'hello, world!");
    let err = Script::builder()
        .script(script)
        .build()
        .evaluate(ctx)
        .err()
        .unwrap()
        .as_object(ctx)
        .unwrap();

    assert!(
        err.get_property(ctx, "name".into())
            .unwrap()
            .to_string_copy(ctx)
            .unwrap()
            .to_rust_string()
            .unwrap()
            .eq("SyntaxError")
    );
}
