mod exercise;

use exercise::functions::run_function_exercise;
use exercise::simple_types::run_simple_types_exercise;
use exercise::variables::run_variables_exercise;

fn main() {
    run_variables_exercise();
    run_function_exercise();
    run_simple_types_exercise();
}
