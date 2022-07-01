use nu_engine::CallExt;
use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::{
    Category, Example, IntoInterruptiblePipelineData, PipelineData, ShellError, Signature, Spanned,
    SyntaxShape,
};
use rand::prelude::SliceRandom;
use rand::thread_rng;

use super::util::tokio_block03;

#[derive(Clone)]
pub struct Ioxsql;

impl Command for Ioxsql {
    fn name(&self) -> &str {
        "ioxsql"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("ioxsql")
            .required(
                "query",
                SyntaxShape::String,
                "SQL to execute against the database",
            )
            .category(Category::Filters)
    }

    fn usage(&self) -> &str {
        "Sql query against the Iox Database."
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let sql: Spanned<String> = call.req(engine_state, stack, 0)?;
        let _ = tokio_block03(&sql);

        let metadata = input.metadata();
        let mut v: Vec<_> = input.into_iter().collect();
        v.shuffle(&mut thread_rng());
        let iter = v.into_iter();
        Ok(iter
            .into_pipeline_data(engine_state.ctrlc.clone())
            .set_metadata(metadata))
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Shuffle rows randomly (execute it several times and see the difference)",
            example: r#"echo [[version patch]; [1.0.0 false] [3.0.1 true] [2.0.0 false]] | shuffle"#,
            result: None,
        }]
    }
}
