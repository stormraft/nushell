use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::{
    Category, Example, IntoInterruptiblePipelineData, PipelineData, ShellError, Signature,
};
use rand::prelude::SliceRandom;
use rand::thread_rng;

use super::util::tokio_block02;

#[derive(Clone)]
pub struct Ioxshuffle;

impl Command for Ioxshuffle {
    fn name(&self) -> &str {
        "ioxshuffle"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("ioxshuffle").category(Category::Filters)
    }

    fn usage(&self) -> &str {
        "Shuffle rows randomly."
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        _call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let me = stack
            .get_env_var(engine_state, "IOX_DBNAME")
            .map(|v| v.as_string().unwrap_or_default())
            .filter(|v| !v.is_empty());

        /*
        let dbname = if let Some(name) = db {
            name
        } else {
            std::env::var("IOX_DBNAME").unwrap()
        };

        */

        let me1 = if let Some(env_name) = me {
            env_name
        } else {
            "you need to throw an error if the environment variable does not exist".to_string()
        };

        println!("me1 {:?}", me1);

        /*
                let me = match stack.get_env_var(engine_state, "IOX_DBNAME") {
                    Some(v) => v,
                    None => Value::Nothing { span: call.head },
                };
        */
        //println!("me: {:?}\n\n\n\n", me);
        println!("bye...");

        let _ = tokio_block02();

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
