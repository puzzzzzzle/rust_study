mod astar;

use pyo3::prelude::*;
pub use astar::*;
#[pyfunction]
pub fn version() -> String {
    "v0.1.0".to_string()
}

// #[pyclass]
// pub struct PyAStar {
//     astar: AStar
// }
// #[pymethods]
// impl PyAStar {
//     #[new]
//     pub fn new(board_lines: Vec<&str>) -> Result<Self,PathFindError> {
//         let created = board_lines.into();
//         if created.is_err() {
//             return Err(astar)
//         }
//         Self {
//             astar: board_lines.into()
//         }
//     }
// }
/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(version, m)?)?;
    Ok(())
}
