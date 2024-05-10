use crate::utils::file_ctl;

pub fn create_fhe_header(method: &str) -> String {
    return file_ctl::create_fhe_header(&method);
}
