use crate::input_handler::{operate, CmdArgument};
use nu_engine::CallExt;
use nu_protocol::ast::Call;
use nu_protocol::ast::CellPath;
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::Category;
use nu_protocol::{Example, PipelineData, ShellError, Signature, Span, SyntaxShape, Type, Value};

struct Arguments {
    substring: String,
    cell_paths: Option<Vec<CellPath>>,
    case_insensitive: bool,
}

impl CmdArgument for Arguments {
    fn take_cell_paths(&mut self) -> Option<Vec<CellPath>> {
        self.cell_paths.take()
    }
}

#[derive(Clone)]
pub struct SubCommand;

impl Command for SubCommand {
    fn name(&self) -> &str {
        "str ends-with"
    }

    fn signature(&self) -> Signature {
        Signature::build("str ends-with")
            .input_output_types(vec![(Type::String, Type::Bool)])
            .vectorizes_over_list(true)
            .required("string", SyntaxShape::String, "the string to match")
            .rest(
                "rest",
                SyntaxShape::CellPath,
                "For a data structure input, check strings at the given cell paths, and replace with result",
            )
            .switch("ignore-case", "search is case insensitive", Some('i'))
            .category(Category::Strings)
    }

    fn usage(&self) -> &str {
        "Check if an input ends with a string."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["suffix", "match", "find", "search"]
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let cell_paths: Vec<CellPath> = call.rest(engine_state, stack, 1)?;
        let cell_paths = (!cell_paths.is_empty()).then_some(cell_paths);
        let args = Arguments {
            substring: call.req::<String>(engine_state, stack, 0)?,
            cell_paths,
            case_insensitive: call.has_flag("ignore-case"),
        };
        operate(action, args, input, call.head, engine_state.ctrlc.clone())
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Checks if string ends with '.rb'",
                example: "'my_library.rb' | str ends-with '.rb'",
                result: Some(Value::test_bool(true)),
            },
            Example {
                description: "Checks if string ends with '.txt'",
                example: "'my_library.rb' | str ends-with '.txt'",
                result: Some(Value::test_bool(false)),
            },
            Example {
                description: "Checks if string ends with '.RB', case-insensitive",
                example: "'my_library.rb' | str ends-with -i '.RB'",
                result: Some(Value::test_bool(true)),
            },
        ]
    }
}

fn action(input: &Value, args: &Arguments, head: Span) -> Value {
    match input {
        Value::String { val: s, .. } => {
            let ends_with = if args.case_insensitive {
                s.to_lowercase().ends_with(&args.substring.to_lowercase())
            } else {
                s.ends_with(&args.substring)
            };
            Value::boolean(ends_with, head)
        }
        Value::Error { .. } => input.clone(),
        _ => Value::Error {
            error: ShellError::OnlySupportsThisInputType {
                exp_input_type: "string".into(),
                wrong_type: input.get_type().to_string(),
                dst_span: head,
                src_span: input.expect_span(),
            },
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() {
        use crate::test_examples;

        test_examples(SubCommand {})
    }
}
