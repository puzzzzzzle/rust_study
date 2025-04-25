mod astar;

use pyo3::prelude::*;
pub use astar::*;
#[pyfunction]
pub fn version() -> String {
    "v0.1.0".to_string()
}

/// astar wrapper for python
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(version, m)?)?;
    m.add_class::<AStar>()?;
    m.add_class::<PathFindError>()?;
    m.add_class::<Pos>()?;
    Ok(())
}
