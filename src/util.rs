#[macro_export]
macro_rules! C_STRING {
    ($msg:expr) => {
        CString::new($msg.to_string()).expect("CString::new failed")
    };
}

#[macro_export]
macro_rules! C_STRING_BUF {
    ($msg:expr) => {
        unsafe { CString::from_vec_unchecked($msg) }
    };
}

#[macro_export]
macro_rules! NAPI_CALL {
    ($napi_call:expr) => {
      assert_eq!($napi_call, napi_status::napi_ok)
    };
}

#[macro_export]
macro_rules! NAPI_CLASS_PROPERTIES {
    ($props:expr) => {{
        let mut class_properties: [_; $props.len()] = std::mem::MaybeUninit::uninit().assume_init();
        for (i, (name_str, method)) in $props.iter().enumerate() {
            let name_cstr = CString::new(*name_str).expect("CString::new failed");
            let mut val = napi_property_descriptor {
                utf8name: name_cstr.as_ptr() as * const std::os::raw::c_char,
                name: std::ptr::null_mut(),
                method: *method,
                getter: None,
                setter: None,
                value: std::ptr::null_mut(),
                attributes: napi_property_attributes::napi_default,
                data: std::ptr::null_mut()
            };

            std::ptr::copy_nonoverlapping(&mut class_properties[i], &mut val, 1);
            std::mem::forget(val);
        }
        class_properties
    }}
}

#[macro_export]
macro_rules! GET_PARAMS {
    ($env:ident, $info:ident, $argc:literal) => {{
        let mut params: [napi_value; $argc] = std::mem::MaybeUninit::zeroed().assume_init();
        let mut this: napi_value = std::mem::zeroed();

        NAPI_CALL!(nodejs_sys::napi_get_cb_info(
            $env,
            $info,
            &mut ($argc),
            params.as_mut_ptr(),
            &mut this,
            std::ptr::null_mut(),
        ));

        (this, params)
    }}
}
