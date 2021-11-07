extern crate coin_cbc_sys;

fn main() {
    unsafe {
        let model = coin_cbc_sys::Cbc_newModel();
        coin_cbc_sys::Cbc_deleteModel(model);
    }
    unsafe {
        let c_buf = coin_cbc_sys::Cbc_getVersion();
        let c_str = std::ffi::CStr::from_ptr(c_buf);
        let version = c_str.to_str().unwrap();
        println!("{}", version);
    }
}
