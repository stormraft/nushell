use super::util::get_runtime;
use nu_engine::CallExt;
use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};

use nu_protocol::{
    Category, Example, PipelineData, ShellError, Signature, Spanned, SyntaxShape, Value,
};

#[derive(Clone)]
pub struct Ioxwrite;

impl Command for Ioxwrite {
    fn name(&self) -> &str {
        "ioxwrite"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("ioxwrite")
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

        let nol_result = tokio_block_write(&dbname, &lp_data);

        println!("{:?}", nol_result);

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

pub fn tokio_block_write(
    dbname: &String,
    lp_data: &Spanned<String>,
) -> Result<usize, std::io::Error> {
    use influxdb_iox_client::{connection::Builder, write::Client};

    let num_threads: Option<usize> = None;
    let tokio_runtime = get_runtime(num_threads)?;

    let nol_result = tokio_runtime.block_on(async move {
        let connection = Builder::default()
            .build("http://127.0.0.1:8081")
            .await
            .expect("client should be valid");

        let mut client = Client::new(connection);

        let nol = client
            .write_lp(dbname.to_string(), lp_data.item.to_string(), 0)
            .await
            .expect("failed to write to IOx");

        nol
    });

    Ok(nol_result)
}
