use super::util::get_runtime;
use nu_engine::CallExt;
use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};
//use nu_protocol::IntoInterruptiblePipelineData;

use nu_protocol::{
    Category, Example, PipelineData, ShellError, Signature, Spanned, SyntaxShape, Value,
};

/*
use nu_protocol::{
    Category, Example, PipelineData, ShellError, Signature, Span, Spanned, SyntaxShape, Value,
};
*/

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
        let lp_data: Spanned<String> = call.req(engine_state, stack, 0)?;
        let nol_result = tokio_block_write(&lp_data);

        println!("{:?}", nol_result);

        /*
        let no_infer = false;
        let noheaders = false;

        let separator: char = ',';

        let trim = Trim::None;
        */

        // let input = Value(String { val: "a,b,c\n1,2,10\n3,4,20\n", span: Span { start: 28605, end: 28611 } }, None);

        /*
        let input = PipelineData::Value(
            Value::String {
                val: nol_result.unwrap(),
                span: call.head,
            },
            None,
        );
        */

        /* This works !!
        let input = PipelineData::Value(
            Value::String {
                val: "a,b,c\n1,2,10\n3,4,20\n".to_string(),
                span: call.head,
            },
            None,
        );
        */

        // This compiles and returns nothing
        // let input = PipelineData::Value(Value::Nothing { span: call.head }, None);

        // let name = Span::new(0, 0);
        // let config = engine_state.get_config();

        // from_delimited_data(noheaders, no_infer, separator, trim, input, name, config)

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

pub fn tokio_block_write(lp_data: &Spanned<String>) -> Result<usize, std::io::Error> {
    //use influxdb_iox_client::{connection::Builder, repl::Repl};

    use influxdb_iox_client::{connection::Builder, write::Client};

    let num_threads: Option<usize> = None;
    let tokio_runtime = get_runtime(num_threads)?;

    let nol_result = tokio_runtime.block_on(async move {
        let connection = Builder::default()
            .build("http://127.0.0.1:8081")
            .await
            .expect("client should be valid");

        let mut client = Client::new(connection);

        let _dbname_from_env = std::env::var("INFLUXDB_IOX_CATALOG_DSN").unwrap();

        let nol = client
            .write_lp("pears", lp_data.item.to_string(), 0)
            .await
            .expect("failed to write to IOx");

        // write a line of line protocol data
        /*
        let nol = client
            .write_lp("pears", "cpu,region=north user=50.32 20000000", 0)
            .await
            .expect("failed to write to IOx");
            */

        // let mut repl = Repl::new(connection);
        // repl.use_database(dbname);
        // repl.use_database("postgresql:///iox_shared".to_string());

        //let _output_format = repl.set_output_format("csv");

        /*
        let x = repl
            //.run_sql("select * from h2o_temperature".to_string())
            .run_sql(sql.item.to_string())
            .await
            .expect("run_sql");
            */
        // println!("{:?}", x);
        // x
        nol
    });

    Ok(nol_result)
}
