use std::ffi::CString;
use nodejs_sys::{
    napi_callback,
    napi_define_class,
    napi_env,
    napi_property_attributes,
    napi_property_descriptor,
    napi_set_named_property,
    napi_status,
    napi_value,
};

#[macro_use]
mod util;
mod napi;

use napi::NapiNeighborino;

#[derive(Debug)]
pub struct Neighborino {

}

impl Neighborino {
    pub fn new<'a> () -> Neighborino {
        Neighborino {

        }
    }

    pub fn insert (&mut self) {

    }
}

#[no_mangle]
pub unsafe extern "C" fn napi_register_module_v1(
    env: napi_env,
    exports: napi_value
) -> nodejs_sys::napi_value {
    // define the properties
    const PROPS: [(&str, napi_callback); 1] = [
        ("printStr", Some(Neighborino::napi_insert))
    ];

    let properties: [napi_property_descriptor; 1] = NAPI_CLASS_PROPERTIES!(PROPS);

    // define the class
    let class_name = "Neighborino";
    let name_len = class_name.len();
    let c_name = C_STRING!(class_name);
    let mut cons: napi_value = std::mem::zeroed();
    NAPI_CALL!(napi_define_class(
        env,
        c_name.as_ptr(),
        name_len,
        Some(Neighborino::constructor),
        std::ptr::null_mut(),
        properties.len(),
        properties.as_ptr(),
        &mut cons
    ));

    // attach the constructor to the module
    NAPI_CALL!(napi_set_named_property(env, exports, c_name.as_ptr(), cons));

    exports
}
