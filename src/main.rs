mod exercise;

use exercise::{
    control_flow_strings::run_control_flow_exercise, functions::run_function_exercise,
    ownership_referencs::run_ownership_exercise, simple_types::run_simple_types_exercise,
    variables::run_variables_exercise,
};

fn main() {
    run_variables_exercise();
    run_function_exercise();
    run_simple_types_exercise();
    run_control_flow_exercise();
    run_ownership_exercise();
}
