use super::util::get_runtime;
use nu_engine::CallExt;
use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::{
    Category, Example, IntoInterruptiblePipelineData, PipelineData, ShellError, Signature, Spanned,
    SyntaxShape,
};
use rand::prelude::SliceRandom;
use rand::thread_rng;

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
        let _ = tokio_block_sql(&sql);

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

pub fn tokio_block_sql(sql: &Spanned<String>) -> Result<(), std::io::Error> {
    use influxdb_iox_client::{connection::Builder, repl::Repl};
    let num_threads: Option<usize> = None;
    let tokio_runtime = get_runtime(num_threads)?;

    tokio_runtime.block_on(async move {
        let connection = Builder::default()
            .build("http://127.0.0.1:8082")
            .await
            .expect("client should be valid");

        let mut repl = Repl::new(connection);

        let dbname = std::env::var("INFLUXDB_IOX_CATALOG_DSN").unwrap();

        repl.use_database(dbname);
        // repl.use_database("postgresql:///iox_shared".to_string());

        let _output_format = repl.set_output_format("csv");

        let x = repl
            //.run_sql("select * from h2o_temperature".to_string())
            .run_sql(sql.item.to_string())
            .await
            .expect("run_sql");

        println!("{:?}", x);
    });

    Ok(())
}
