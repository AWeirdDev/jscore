use jscore::prelude::*;

#[test]
fn evaluate_hello() {
    let group = ContextGroup::new();
    let global = group.create_global_context();
    let ctx = &global.as_context();

    let script = JsString::new("'hello, world!'");
    let result = Script::new(ctx, &script, None, None, None)
        .evaluate(ctx)
        .unwrap();

    assert_eq!(
        result.to_string_copy_lossy(ctx).to_string().as_str(),
        "hello, world!"
    );
}
