#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::{Operators, eval_input, shunting_yard, eval_rpn};

    #[rstest]
    #[case("8 + 8 * 8\n", 72.0)]
    fn test_e2e(#[case] input: &str, #[case] expected_result: f64) {
        let input = String::from(input);

        let ops = Operators {};

        let output = eval_input(&input);
        let rpn = shunting_yard(output);
        let actual_result = eval_rpn(rpn, &ops);

        assert_eq!(actual_result, expected_result);
    }
}