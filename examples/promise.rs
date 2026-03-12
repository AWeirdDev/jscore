use jscore::prelude::*;

fn main() {
    let group = ContextGroup::new();
    let global = group.create_global_context();
    let ctx = global.as_context();

    let content = JsString::new(
        r#"
Promise
"#,
    );
    let script = Script::builder().script(&content).build();
    let result = script.evaluate(ctx);
    if let Err(e) = &result {
        panic!(
            "{}",
            e.to_string_copy(ctx).unwrap().to_rust_string().unwrap()
        );
    }
    let result = result.unwrap();

    let result = result
        .to_string_copy(ctx)
        .unwrap()
        .to_rust_string()
        .unwrap();

    println!("{result}");
    // global.retain();
}
