use super::util::tokio_block_write;
use nu_engine::CallExt;
use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};

use nu_protocol::{
    Category, Example, PipelineData, ShellError, Signature, Spanned, SyntaxShape, Value,
};

#[derive(Clone)]
pub struct Ioxwritefile;

impl Command for Ioxwritefile {
    fn name(&self) -> &str {
        "ioxwritefile"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("ioxwritefile")
            .required(
                "data",
                SyntaxShape::String,
                "Line protocol string to write to Iox",
            )
            .named(
                "dbname",
                SyntaxShape::String,
                "name of the database to write to",
                Some('d'),
            )
            .category(Category::Filters)
    }

    fn usage(&self) -> &str {
        "Write data to the Iox Database."
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let lp_data: Spanned<String> = call.req(engine_state, stack, 0)?;
        let db: Option<String> = call.get_flag(engine_state, stack, "dbname")?;

        let dbname = if let Some(name) = db {
            name
        } else {
            std::env::var("IOX_DBNAME").unwrap()
        };

        println!("dbname = {:?}", dbname);

        let nol_result = tokio_block_write(&dbname, &lp_data);

        println!("{:?}", nol_result);

        Ok(PipelineData::Value(
            Value::Nothing { span: call.head },
            None,
        ))
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Write some line protocol data out to Iox using the bananas db",
                example: r#"ioxwrite -d bananas "cpu,region=la user=955111599 222522"#,
                result: None,
            },
            Example {
                description: "Write some line protocol data out to Iox using the default db",
                example: r#"ioxwrite "cpu,region=pa user=9599 222522"#,
                result: None,
            },
        ]
    }
}
