use pyo3::prelude::*;

#[pyfunction]
fn hello_from_rust() -> PyResult<String> {
    Ok("Hello from Rust!".to_string())
}

#[pyfunction]
fn printrs(s: String) -> PyResult<()> {
    println!("{}", s);
    Ok(())
}

#[pymodule]
fn pyrusttestrs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello_from_rust, m)?)?;
    m.add_function(wrap_pyfunction!(printrs, m)?)?;
    Ok(())
}
