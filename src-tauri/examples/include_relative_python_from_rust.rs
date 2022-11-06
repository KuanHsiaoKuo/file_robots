use pyo3::prelude::*;

fn main() -> PyResult<()> {
    // 从项目根目录添加相对路径文件
    let py_foo_path = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/python_demo/utils/foo.py");
    let py_foo = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/python_demo/utils/foo.py"));
    let py_app = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/python_demo/app.py"));
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        PyModule::from_code(py, py_foo, "utils.foo", "utils.foo")?;
        let app: Py<PyAny> = PyModule::from_code(py, py_app, "", "")?
            .getattr("run")?
            .into();
        app.call0(py)
    });

    println!("py: {}", from_python?);
    Ok(())
}
