use jscore::prelude::*;

fn main() {
    let group = ContextGroup::new();

    // Create the global context for JS interactions
    let global = group.create_global_context();
    let ctx = global.as_context();

    // Create a script
    {
        let content = JsString::new("(() => 'hello from js!')()");
        let script = Script::builder().script(&content).build();
        let result = script.evaluate(ctx).expect("failed to run script");
        let result_str = result
            .to_string_copy(ctx)
            .unwrap()
            .to_rust_string()
            .unwrap();
        println!("result: {result_str}");
    }
}
