use crate::broccoli::{
    broccoli_helper_functions::{broccoli_greater_or_equal, broccoli_is_integer},
    environments::environment::{run_simulation_with_rewards, Environment, EnvironmentInfo},
    trees::decision_tree::DecisionTree,
};

use super::evaluator::Evaluator;

pub struct EnvironmentEvaluatorMaximiseReturn<'a, E: Environment + ?Sized> {
    environment: &'a mut E,
    initial_states: Vec<Vec<f64>>,
    max_num_states: u32,
    best_score: Option<u32>,
    num_environment_calls: usize,
}

impl<'a, E: Environment + ?Sized> EnvironmentEvaluatorMaximiseReturn<'a, E> {
    pub fn new(environment: &'a mut E, initial_states: &[Vec<f64>], max_num_states: u32) -> Self {
        EnvironmentEvaluatorMaximiseReturn {
            environment,
            initial_states: initial_states.to_vec(),
            max_num_states,
            best_score: None,
            num_environment_calls: 0,
        }
    }
}

impl<'a, E: Environment + ?Sized> Evaluator for EnvironmentEvaluatorMaximiseReturn<'a, E> {
    fn environment_info(&self) -> &EnvironmentInfo {
        self.environment.environment_info()
    }

    fn evaluate(&mut self, decision_tree: DecisionTree) -> (DecisionTree, Result<f64, ()>) {
        if let Some(optimal_value) = self.best_score {
            if optimal_value == self.max_num_states {
                return (decision_tree, Err(()));
            }
        }

        let mut controller_global: Option<DecisionTree> = None;
        let mut new_global_score: Option<u32> = None;
        for initial_state in &self.initial_states {
            let mut controller = decision_tree.clone();
            let total_return = run_simulation_with_rewards(
                &mut *self.environment,
                initial_state,
                &mut controller,
                self.max_num_states,
            );
            self.num_environment_calls += 1;

            if total_return <= self.best_score.unwrap_or(0) {
                return (controller, Err(()));
            } else {
                match new_global_score {
                    Some(new_global_value) => {
                        if new_global_value > total_return {
                            new_global_score = Some(total_return);
                        }
                    }
                    None => new_global_score = Some(total_return),
                }

                match controller_global.as_mut() {
                    Some(_) => {
                        controller_global
                            .as_mut()
                            .unwrap()
                            .merge_threshold_distances(&controller);
                    }
                    None => controller_global = Some(controller),
                }
            }
        }
        (
            controller_global.unwrap(),
            Ok(new_global_score.unwrap() as f64),
        )
    }

    fn register_new_best_score(&mut self, new_best_score: f64) {
        assert!(broccoli_is_integer(new_best_score));
        assert!(broccoli_greater_or_equal(new_best_score, 0.0));

        assert!(self.best_score.is_none() || (new_best_score as u32) > self.best_score.unwrap());
        self.best_score = Some(new_best_score as u32);
    }

    fn num_environment_calls(&self) -> usize {
        self.num_environment_calls
    }
}
