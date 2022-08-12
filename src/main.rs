mod exercise;

use exercise::control_flow_strings::run_control_flow_exercise;
use exercise::functions::run_function_exercise;
use exercise::simple_types::run_simple_types_exercise;
use exercise::variables::run_variables_exercise;

fn main() {
    run_variables_exercise();
    run_function_exercise();
    run_simple_types_exercise();
    run_control_flow_exercise();
}
