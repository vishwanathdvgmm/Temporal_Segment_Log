#![cfg(feature = "python")]

use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::Bound;

use crate::{Event, TSL};

#[pyclass]
pub struct PyTSL {
    inner: TSL,
}

#[pymethods]
impl PyTSL {
    #[new]
    fn new(capacity: usize) -> Self {
        Self {
            inner: TSL::new(capacity),
        }
    }

    fn append(&mut self, timestamp: u64, payload: Vec<u8>) {
        self.inner.append(Event::new(timestamp, payload));
    }

    fn latest(&self, n: usize) -> Vec<(u64, Vec<u8>)> {
        self.inner
            .latest(n)
            .into_iter()
            .map(|e| (e.timestamp, e.payload))
            .collect()
    }

    fn range_query(&self, start: u64, end: u64) -> Vec<(u64, Vec<u8>)> {
        self.inner
            .range_query(start, end)
            .into_iter()
            .map(|e| (e.timestamp, e.payload))
            .collect()
    }
}

#[pymodule]
fn tsl(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyTSL>()?;
    Ok(())
}
