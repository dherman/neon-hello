#[macro_use]
extern crate neon;

use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("Hello from Neon!"))
}

register_module!(mut cx, {
    cx.export_function("hello", hello)
});
