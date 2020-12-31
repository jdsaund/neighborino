use nodejs_sys::{
  napi_callback_info,
  napi_get_undefined,
  napi_env,
  napi_status,
  napi_unwrap,
  napi_value,
  napi_wrap,
};

use crate::Neighborino;

pub trait NapiNeighborino {
    extern "C" fn destructor(
        _env: napi_env,
        finalize_data: *mut std::ffi::c_void,
        _finalize_hint: *mut std::ffi::c_void
    ) -> ();

    unsafe extern "C" fn constructor(
        env: napi_env,
        info: napi_callback_info
    ) -> napi_value;

    unsafe extern "C" fn napi_insert (
        env: napi_env,
        info: napi_callback_info
    ) -> napi_value;
}

impl NapiNeighborino for Neighborino {
    #[no_mangle]
    extern "C" fn destructor(
        _env: napi_env,
        finalize_data: *mut std::ffi::c_void,
        _finalize_hint: *mut std::ffi::c_void
    ) -> () {
        // dont try to double free
        if finalize_data.is_null() {
            return;
        }

        // free the memory
        let this: Box<Neighborino> = unsafe { Box::from_raw(finalize_data as *mut Neighborino) };
        std::mem::drop(this);
    }

    #[no_mangle]
    unsafe extern "C" fn constructor(
        env: napi_env,
        info: napi_callback_info
    ) -> napi_value {
         // get and unwrap the params
         let (this, params) = GET_PARAMS!(env, info, 2);

        // downcast params

        // create the instance and put it on the heap
        let native_object: *mut Neighborino = Box::into_raw(Box::new(Neighborino::new(/* params */)));
        NAPI_CALL!(napi_wrap(
            env,
            this,
            native_object as *mut std::ffi::c_void,
            Some(Neighborino::destructor),
            std::ptr::null_mut(),
            std::ptr::null_mut()
        ));

        // tell rust to not manage the native_object memory
        std::mem::forget(native_object);

        this
    }

    #[no_mangle]
    unsafe extern "C" fn napi_insert(
        env: napi_env,
        info: napi_callback_info
    ) -> napi_value {
        // get and unwrap the params
        let (this, _params) = GET_PARAMS!(env, info, 1);

        // unwrap the native object
        let mut box_ptr: *mut std::ffi::c_void = std::ptr::null_mut();
        NAPI_CALL!(napi_unwrap(env, this, &mut box_ptr));
        let mut native_obj = Box::from_raw(box_ptr as *mut Neighborino);

        let _native_result = native_obj.insert();

        // dont free memory that node is still using
        std::mem::forget(native_obj);

        // TEMP: return undefined
        let mut undefined: napi_value = std::mem::zeroed();
        NAPI_CALL!(napi_get_undefined(env, &mut undefined));
        undefined
    }
}
