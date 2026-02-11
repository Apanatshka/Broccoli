use crate::broccoli::environments::environment::{Environment, EnvironmentInfo};
use crate::broccoli::evaluators::evaluator::Evaluator;
use pyo3::prelude::*;

mod broccoli;

#[pyo3::pymodule]
mod broccoli_python {
    use crate::broccoli::environments::environment::{EnvironmentInfo, Interval};
    use crate::PythonBasedEnvironment;
    use pyo3::prelude::*;
    use pyo3::types::PyList;

    fn environment_info(obj: &Bound<PyAny>) -> PyResult<EnvironmentInfo> {
        let num_actions = obj.getattr("num_actions")?.extract()?;
        let feature_ranges_list: Bound<PyList> = obj.getattr("feature_ranges")?.extract()?;
        let feature_ranges = intervals(&feature_ranges_list)?;
        let start_ranges_list: Bound<PyList> = obj.getattr("start_ranges")?.extract()?;
        let start_ranges = intervals(&start_ranges_list)?;
        Ok(EnvironmentInfo {
            num_actions,
            feature_ranges,
            start_ranges,
        })
    }

    fn intervals(list: &Bound<PyList>) -> PyResult<Vec<Interval>> {
        let feature_ranges_result: PyResult<_> = list
            .iter()
            .map(|obj| {
                let name = obj.getattr("name")?.extract()?;
                let min = obj.getattr("min")?.extract()?;
                let max = obj.getattr("max")?.extract()?;
                Ok(Interval { name, min, max })
            })
            .collect();
        feature_ranges_result
    }

    #[pyfunction]
    fn run_solver<'py>(
        apply_action: Bound<PyAny>,
        observe_state: Bound<PyAny>,
        is_at_terminal_state: Bound<PyAny>,
        reset: Bound<PyAny>,
        #[pyo3(from_py_with = environment_info)] environment_info: EnvironmentInfo,
        minimise: bool,
    ) {
        let env = PythonBasedEnvironment {
            apply_action,
            observe_state,
            is_at_terminal_state,
            reset,
            environment_info,
            minimise,
        };
        todo!()
    }
}

struct PythonBasedEnvironment<'py> {
    apply_action: Bound<'py, PyAny>,
    observe_state: Bound<'py, PyAny>,
    is_at_terminal_state: Bound<'py, PyAny>,
    reset: Bound<'py, PyAny>,
    environment_info: EnvironmentInfo,
    minimise: bool,
}

impl Environment for PythonBasedEnvironment<'_> {
    fn minimise(&self) -> bool {
        self.minimise
    }
    fn apply_action(&mut self, action: usize) {
        self.apply_action
            .call1((action,))
            .expect("apply_action should succeed");
    }

    fn observe_state(&self) -> Vec<f64> {
        self.observe_state
            .call0()
            .expect("observe_state should succeed")
            .extract()
            .expect("observe_state should return a list of floats")
    }

    fn is_at_terminal_state(&self) -> bool {
        self.is_at_terminal_state
            .call0()
            .expect("is_at_terminal_state should succeed")
            .extract()
            .expect("is_at_terminal_state should return a bool")
    }

    fn reset(&mut self, initial_state: &[f64]) {
        self.observe_state
            .call1((initial_state,))
            .expect("observe_state should succeed");
    }

    fn environment_info(&self) -> &EnvironmentInfo {
        &self.environment_info
    }

    fn evaluator<'a>(
        &'a mut self,
        initial_states: &[Vec<f64>],
        max_num_states: u32,
    ) -> Box<dyn Evaluator + 'a> {
        todo!()
    }
}
