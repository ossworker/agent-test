macro_rules! jvmti_call {
    ($jvmtienv:expr, $f:ident, $($args:tt)*) => {{
        let rc;
        unsafe {
            let func  = (**$jvmtienv).$f.expect("jvmti {} function not found",stringify!(($f)));
            rc = func($jvmtienv, $($args)*);
        }
        if rc!=jvmti::JNI_OK {
          let message = format!("JVMTI {} failed", stringify!($f));
          bail!(::errors::ErrorKind::JvmTi(message, rc as i32))
        }else{
            Ok(())
        }
    }};
}

