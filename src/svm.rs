use ffi;
use std::ffi::CString;

pub fn load_model(filename: String) -> Result<*const ffi::svm::Struct_svm_model, &'static str>
{
    unsafe {
        let c_filename = CString::new(filename).unwrap();
        let mut_model = ffi::svm::svm_load_model(c_filename.as_ptr());
        let model: *const ffi::svm::Struct_svm_model = mut_model;
        if !model.is_null() {
            return Ok(model);
        } else {
            return Err("Could not load model!");
        }
    }
}

pub fn save_model(filename: String, model: *const ffi::svm::Struct_svm_model) -> Result<(), &'static str>
{
    unsafe {
        let c_filename = CString::new(filename).unwrap();
        let result = ffi::svm::svm_save_model(c_filename.as_ptr(), model);
        if result == 0 {
            return Ok(());
        } else {
            return Err("Could not save model!");
        }
    }
}
