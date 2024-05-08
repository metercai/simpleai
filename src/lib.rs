use std::path::Path;
use std::net::Ipv4Addr;
use std::str::FromStr;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;

use pyo3::prelude::*;
use crate::token::SimpleAI;
use crate::claim::IdClaim;
use crate::systeminfo::SystemInfo;

mod claim;
mod env_utils;
mod error;
mod rathole;
mod token;
mod systeminfo;


#[pyfunction]
fn init_local(nick: String) -> PyResult<SimpleAI> {
    Ok(SimpleAI::new(nick))
}

#[pyfunction]
fn sha256(input: &[u8]) -> String {
    URL_SAFE_NO_PAD.encode(env_utils::calc_sha256(input))
}

#[pyfunction]
fn file_hash_size(path: String) -> (String, u64) {
    let Ok((hash, size)) = env_utils::get_file_hash_size(Path::new(&path))
        else { return ("".to_string(), 0) };
    (hash, size)
}

#[pyfunction]
fn get_ipaddr_from_stream() -> String {
    match env_utils::get_ipaddr_from_stream(None) {
        Ok(ip) => ip.to_string(),
        Err(_) => "".to_string(),
    }
}

#[pyfunction]
fn get_ipaddr_from_public(py: Python, is_out: bool) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        match env_utils::get_ipaddr_from_public(is_out).await {
            Ok(ip) => Ok(ip.to_string()),
            Err(_) => Ok("".to_string()),
        }
    })
}

#[pyfunction]
fn get_port_availability(py: Python, ip: String, port: u16) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        Ok(env_utils::get_port_availability(Ipv4Addr::from_str(&ip).unwrap(), port).await)
    })
}

#[pymodule]
fn simpleai(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init_local, m)?)?;
    m.add_function(wrap_pyfunction!(sha256, m)?)?;
    m.add_function(wrap_pyfunction!(file_hash_size, m)?)?;
    m.add_function(wrap_pyfunction!(get_ipaddr_from_public, m)?)?;
    m.add_function(wrap_pyfunction!(get_port_availability, m)?)?;
    m.add_function(wrap_pyfunction!(get_ipaddr_from_stream, m)?)?;
    m.add_class::<SimpleAI>()?;
    m.add_class::<IdClaim>()?;
    m.add_class::<SystemInfo>()?;

    Ok(())
}
