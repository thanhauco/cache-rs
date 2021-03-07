use rlua::Lua;
pub fn eval(script: &str) {
    let lua = Lua::new();
    lua.context(|ctx| {
        ctx.load(script).exec().unwrap();
    });
}