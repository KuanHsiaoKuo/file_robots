use pyo3::prelude::*;
use pyo3::types::PyList;
use std::fs;
use std::path::Path;

fn main() -> PyResult<()> {
    // 使用绝对路径
    let path = Path::new("/Users/kuanhsiaokuo/Developer/spare_projects/file_robots/src-tauri/examples/python_demo");
    let py_app = fs::read_to_string(path.join("app.py"))?;
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let syspath: &PyList = py.import("sys")?.getattr("path")?.downcast::<PyList>()?;
        syspath.insert(0, &path)?;
        let app: Py<PyAny> = PyModule::from_code(py, &py_app, "", "")?
            .getattr("run")?
            .into();
        app.call0(py)
    });

    println!("py: {}", from_python?);
    Ok(())
}
