use jscore::prelude::*;

#[test]
fn test_context_group() {
    let group = ContextGroup::new();
    let global = group.create_global_context();
    let _context = global.as_context();

    drop(group);
    // context.get_global_context();
}
