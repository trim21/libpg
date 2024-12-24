use pyo3::prelude::*;

#[pyfunction]
fn connect(dsn: &str) -> PyResult<String> {
    let conn = libpq::Connection::new(dsn)?;

    conn.exec(query)
}

#[pymodule]
fn libpg(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(connect, m)?)?;
    Ok(())
}
