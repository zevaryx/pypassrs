use cpython::{exc, py_fn, py_module_initializer, PyErr, PyResult, Python};
use passrs;

py_module_initializer!(pypassrs, |py, m| {
    m.add(py, "__doc__", "Python wrapper for passrs.")?;
    m.add(py, "show", py_fn!(py, show(name: String)))?;
    m.add(py, "init", py_fn!(py, path_init(path: String)))?;
    m.add(py, "edit", py_fn!(py, edit(path: String, password: String)))?;
    m.add(
        py,
        "mv",
        py_fn!(
            py,
            mv(
                src: String,
                dest: String,
                force: bool = false,
                remove: bool = false
            )
        ),
    )?;
    m.add(
        py,
        "insert",
        py_fn!(
            py,
            insert(path: String, password: String, force: bool = false)
        ),
    )?;
    m.add(
        py,
        "tree",
        py_fn!(
            py,
            ls(
                path: Option<String> = None,
                query: Option<Vec<String>> = None
            )
        ),
    )?;
    m.add(
        py,
        "generate",
        py_fn!(
            py,
            generate(
                symbols: bool = true,
                qr: bool = false,
                force: bool = false,
                length: usize = 32,
                path: Option<String> = None,
            )
        ),
    )?;
    m.add(
        py,
        "rm",
        py_fn!(py, rm(path: String, recursive: bool = false)),
    )?;
    Ok(())
});

fn show(_py: Python, name: String) -> PyResult<String> {
    let result = passrs::password::show(name, false, false);
    match result {
        Ok(r) => match r {
            Some(p) => Ok(p),
            None => Err(PyErr::new::<exc::Exception, _>(
                _py,
                "Failed to get password",
            )),
        },
        Err(e) => Err(PyErr::new::<exc::Exception, _>(_py, e.to_string())),
    }
}

fn path_init(_py: Python, path: String) -> PyResult<bool> {
    let result = passrs::directory::init(Some(path.to_owned()));
    match result {
        Ok(_) => Ok(true),
        Err(e) => Err(PyErr::new::<exc::Exception, _>(_py, e.to_string())),
    }
}

fn insert(_py: Python, path: String, password: String, force: bool) -> PyResult<bool> {
    let result = passrs::password::insert(path.to_owned(), force, password.to_owned());
    match result {
        Ok(_) => Ok(true),
        Err(e) => Err(PyErr::new::<exc::Exception, _>(_py, e.to_string())),
    }
}

fn edit(_py: Python, path: String, password: String) -> PyResult<bool> {
    let result = passrs::password::edit(path.to_owned(), password.to_owned());
    match result {
        Ok(_) => Ok(true),
        Err(e) => Err(PyErr::new::<exc::Exception, _>(_py, e.to_string())),
    }
}

fn mv(_py: Python, src: String, dest: String, force: bool, remove: bool) -> PyResult<bool> {
    let result = passrs::password::mv(src.to_owned(), dest.to_owned(), force, remove);
    match result {
        Ok(_) => Ok(true),
        Err(e) => Err(PyErr::new::<exc::Exception, _>(_py, e.to_string())),
    }
}

fn rm(_py: Python, path: String, recursive: bool) -> PyResult<bool> {
    let result = passrs::password::rm(path.to_owned(), recursive);
    match result {
        Ok(_) => Ok(true),
        Err(e) => Err(PyErr::new::<exc::Exception, _>(_py, e.to_string())),
    }
}

fn generate(
    _py: Python,
    symbols: bool,
    qr: bool,
    force: bool,
    length: usize,
    path: Option<String>,
) -> PyResult<Option<String>> {
    let result = passrs::password::generate(path.to_owned(), symbols, false, qr, force, length);
    match result {
        Ok(r) => match r {
            Some(p) => Ok(Some(p)),
            None => match path {
                Some(_) => Ok(None),
                None => Err(PyErr::new::<exc::Exception, _>(
                    _py,
                    "Failed to generate password",
                )),
            },
        },
        Err(e) => Err(PyErr::new::<exc::Exception, _>(_py, e.to_string())),
    }
}

fn ls(_py: Python, path: Option<String>, query: Option<Vec<String>>) -> PyResult<String> {
    let result = passrs::directory::show_tree(path, query);
    match result {
        Ok(t) => Ok(t),
        Err(e) => Err(PyErr::new::<exc::Exception, _>(_py, e.to_string())),
    }
}
