use std::path::Path;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;

use pyo3::prelude::*;
use crate::token::TokenDid;
use crate::claim::IdClaim;


mod claim;
mod env_utils;
mod error;
mod rathole;
mod token;


#[pyfunction]
fn init_local_did(nick: String) -> PyResult<TokenDid> {
    print!("begin to TokenDid");
    let token = TokenDid::new(nick);
    print!("TokenDid init success");
    Ok(token)
}

#[pyfunction]
fn sha256(input: &[u8]) -> String {
    URL_SAFE_NO_PAD.encode(env_utils::calc_sha256(input))
}

#[pyfunction]
fn file_hash_size(path: String) -> (String, u64) {
    print!("begin to file_hash_size");
    let Ok((hash, size)) = env_utils::get_file_hash_size(Path::new(&path))
        else { return ("".to_string(), 0) };
    (hash, size)
}

#[pyfunction]
fn get_current_dir() -> String {
    env_utils::get_current_dir()
}
#[pymodule]
fn tokendid(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init_local_did, m)?)?;
    m.add_function(wrap_pyfunction!(sha256, m)?)?;
    m.add_function(wrap_pyfunction!(file_hash_size, m)?)?;
    m.add_class::<TokenDid>()?;
    m.add_class::<IdClaim>()?;
    Ok(())
}
