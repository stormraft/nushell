use super::util::get_runtime;
use nu_engine::CallExt;
use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};
//use nu_protocol::IntoInterruptiblePipelineData;
use nu_protocol::{
    Category, Example, PipelineData, ShellError, Signature, Spanned, SyntaxShape, Value,
};
//use rand::prelude::SliceRandom;
//use rand::thread_rng;

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
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let sql: Spanned<String> = call.req(engine_state, stack, 0)?;
        let sql_result = tokio_block_sql(&sql);

        println!("{:?}", sql_result);

        Ok(PipelineData::Value(
            Value::Nothing { span: call.head },
            None,
        ))
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Shuffle rows randomly (execute it several times and see the difference)",
            example: r#"echo [[version patch]; [1.0.0 false] [3.0.1 true] [2.0.0 false]] | shuffle"#,
            result: None,
        }]
    }
}

pub fn tokio_block_sql(sql: &Spanned<String>) -> Result<String, std::io::Error> {
    use influxdb_iox_client::{connection::Builder, repl::Repl};
    let num_threads: Option<usize> = None;
    let tokio_runtime = get_runtime(num_threads)?;

    let sql_result = tokio_runtime.block_on(async move {
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

        // println!("{:?}", x);
        x
    });

    Ok(sql_result)
}
