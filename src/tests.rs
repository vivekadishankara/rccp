#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::{Operators, eval_input, shunting_yard, eval_rpn, Tokens};

    #[rstest]
    #[case("8 + 8 * 8\n", 72.0)]
    fn test_e2e(#[case] input: &str, #[case] expected_result: f64) {
        let input = String::from(input);

        let ops = Operators {};

        let output = eval_input(&input);
        let rpn = shunting_yard(&output);
        let actual_result = eval_rpn(rpn, &ops);

        assert_eq!(actual_result, expected_result);
    }

    #[rstest]
    #[case("8 + 8 * 8\n", &vec![Tokens::Number(8.0), Tokens::Operations('+'), Tokens::Number(8.0),
                                Tokens::Operations('*'), Tokens::Number(8.0)])]
    fn test_eval_input(#[case]input: &str, #[case] expected_result: &Vec<Tokens>){
        let input = String::from(input);

        let actual_output = eval_input(&input);

        assert_eq!(actual_output, *expected_result);
    }

    #[rstest]
    #[case(&vec![Tokens::Number(8.0), Tokens::Operations('+'), Tokens::Number(8.0),
    Tokens::Operations('*'), Tokens::Number(8.0)],
    &vec![Tokens::Number(8.0), Tokens::Number(8.0), Tokens::Number(8.0), 
    Tokens::Operations('*'), Tokens::Operations('+')])]
    fn test_shunting_yard(#[case] input: &Vec<Tokens>, #[case] expected_result: &Vec<Tokens>) {
        let actual_result = shunting_yard(&input);

        assert_eq!(actual_result, *expected_result);
    }
}