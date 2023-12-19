use pyo3::prelude::*;
use std::{thread, time};

#[pyclass(name = "MyClass")]
pub struct MyClass {
    pub name: String,
    pub description: String,
    pub is_display: bool
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
        MyClass{name, description, is_display: false}
    }

    // Print the informations after 10 seconds
    fn display(&mut self) {
        println!("Wait 10 seconds before print informations....");
        let sleep_time = time::Duration::from_millis(10000);
        thread::sleep(sleep_time);
        println!("My name is {} and my description is : {}", self.name, self.description);
        self.is_display = true;
    }

    fn check(&self) {
        if self.is_display {
            println!("I've already been displayed");
        }
    }
}