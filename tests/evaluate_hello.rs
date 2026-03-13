use jscore::prelude::*;

#[test]
fn evaluate_hello() {
    let group = JsContextGroup::new();
    let global = group.create_global_context();
    let ctx = global.as_context();

    let script = JsString::new("'hello, world!'");
    Script::builder()
        .script(&script)
        .build()
        .evaluate(ctx)
        .expect("failed to evaluate");
}
