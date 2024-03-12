#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::{c_char, c_void, CStr, CString};
use std::mem::zeroed;
use crate::jvmti::{JavaVM, jint, jlong, JNI_OK, JNI_VERSION_1_8, JVMTI_VERSION, JVMTI_VERSION_1_2, jvmtiCapabilities, jvmtiEnv};

#[cfg_attr(feature = "cargo-clippy", allow(clippy))]
mod jvmti;
#[warn(unused_macros)]
mod macros;


#[allow(unused_variables)]
#[no_mangle]
pub fn JNI_OnLoad(vm: *mut JavaVM, reserved: *mut c_void) -> jint {
    println!("JNI_OnLoad begin");
    init_agent(vm, reserved);
    0x00010008
}

#[allow(unused_variables)]
fn init_agent(vm: *mut JavaVM, reserved: *mut c_void) {
    let mut penv: *mut c_void = std::ptr::null_mut();
    let jni_rs = unsafe {
        (**vm).GetEnv.expect("GetEnv function not found")(
            vm,
            &mut penv,
            JVMTI_VERSION_1_2 as i32,
        )
    };
    let jvmti_env = penv as *mut jvmtiEnv;
    println!("jvmtiVersion:{}", jni_rs);

    let mut capabilities: jvmtiCapabilities = jvmtiCapabilities{
        _bitfield_align_1: [],
        _bitfield_1: Default::default(),
    };
    capabilities.set_can_generate_exception_events(1u32);
    capabilities.set_can_access_local_variables(1u32);
    unsafe { (**jvmti_env).AddCapabilities.expect("AddCapabilities function not found")(jvmti_env, &capabilities); }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn Agent_OnLoad(
    vm: *mut JavaVM,
    options: *mut c_char,
    reserved: *mut c_void,
) -> jint {
    let option = unsafe { CStr::from_ptr(options).to_str().unwrap() };
    println!("Agent_Onload begin options:{}", option);
    init_agent(vm, reserved);
    0
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn Agent_OnAttach(
    vm: *mut JavaVM,
    options: *mut c_char,
    reserved: *mut c_void,
) -> jint {
    let option = unsafe { CStr::from_ptr(options.clone()).to_str().unwrap() };
    println!("Agent_OnAttach begin options:{}", option);
    // init_agent(vm, reserved);
    0
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn Agent_OnUnload(
    vm: *mut JavaVM,
) {
    println!("Agent_OnUnload begin");
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use jvmti_macros::call_jvmti;
    use super::*;

    #[test]
    fn it_works() {
        CString::new(String::from("111=222")).unwrap();
        call_jvmti!(1,"2","1");
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
