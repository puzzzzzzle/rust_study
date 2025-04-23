mod astar;

use pyo3::prelude::*;
pub use astar::AStarPathfinder;

#[pyfunction]
pub fn version() -> String {
    "v0.1.0".to_string()
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(version, m)?)?;
    m.add_class::<AStarPathfinder>()?;  // 注册AStarPathfinder类
    Ok(())
}
