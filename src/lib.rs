use pyo3::prelude::*;
use std::{thread, time};

#[pyclass(name = "MyClass")]
pub struct MyClass {
    pub name: String,
    pub description: String,
}

#[pymodule]
fn pyo3_example(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<MyClass>()?;
    Ok(())
}

#[pymethods]
impl MyClass {
    #[new]
    fn new(name: String, description: String) -> Self {
        MyClass{name, description}
    }

    // Print the informations after 10 seconds
    fn display(&mut self) {
        println!("Wait 10 seconds before print informations....");
        let sleep_time = time::Duration::from_millis(10000);
        thread::sleep(sleep_time);
        println!("My name is {} and my description is : {}", self.name, self.description);
    }
}