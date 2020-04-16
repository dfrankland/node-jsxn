use ::jsxn::jsxn;
use cfg_if::cfg_if;
use nom::error::ErrorKind;
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        use wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

cfg_if! {
    if #[cfg(feature = "debug_assertions")] {
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_namespace = console)]
            fn log(s: &str);
        }
    }
}

fn err_into_js_value<E: Debug>(err: E) -> JsValue {
    JsValue::from_str(&format!("{:?}", err))
}

#[wasm_bindgen(js_name = parseString)]
pub fn parse_string(jsxn_string: String) -> Result<JsValue, JsValue> {
    cfg_if! {
        if #[cfg(feature = "debug_assertions")] {
            log(&format!("`parse_string` received {:?}", jsxn_string));
        }
    }

    let jsx_value = jsxn::root::<(&str, ErrorKind)>(&jsxn_string)
        .map_err(err_into_js_value)?
        .1;
    JsValue::from_serde(&jsx_value).map_err(err_into_js_value)
}

cfg_if! {
    if #[cfg(feature = "parse_file")] {
        use serde_json::json;

        #[wasm_bindgen(module = "fs")]
        extern "C" {
            #[wasm_bindgen(js_name = readFileSync)]
            fn read_file_sync(path: String, options: &JsValue) -> String;
        }

        #[wasm_bindgen(module = "path")]
        extern "C" {
            #[wasm_bindgen(js_name = resolve)]
            fn resolve_path(path: String) -> String;
        }

        #[wasm_bindgen(js_name = parseFile)]
        pub fn parse_file(jsxn_file: String) -> Result<JsValue, JsValue> {
            cfg_if! {
                if #[cfg(feature = "debug_assertions")] {
                    log(&format!("`parse_file` received {:?}", jsxn_file));
                }
            }

            let path = resolve_path(jsxn_file);
            let read_file_sync_options = JsValue::from_serde(&json!({ "encoding": "utf8" })).map_err(err_into_js_value)?;
            let jsxn_string = read_file_sync(path, &read_file_sync_options);

            parse_string(jsxn_string)
        }
    } else {
        #[wasm_bindgen(js_name = parseFile)]
        pub fn parse_file(_: String) -> Result<JsValue, JsValue> {
            Err(err_into_js_value("Not supported in the browser."))
        }
    }
}
