use jscore::prelude::*;

#[test]
fn test_context_group() {
    let group = JsContextGroup::new();
    let global = group.create_global_context();
    let _context = global.as_context();

    let global2 = group.create_global_context();

    drop(global2);
    drop(global);
    drop(group);
}
