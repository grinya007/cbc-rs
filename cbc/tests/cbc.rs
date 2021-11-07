extern crate coin_cbc_sys;

#[test]
fn test_cbc_version_major() {
    unsafe {
        let c_buf = coin_cbc_sys::Cbc_getVersion();
        let c_str = std::ffi::CStr::from_ptr(c_buf);
        let version = c_str.to_str().unwrap();
        println!("{}", version);
    }
}

#[test]
fn test_model() {
    unsafe {
        let model = coin_cbc_sys::Cbc_newModel();
        coin_cbc_sys::Cbc_deleteModel(model);
    }
}
