use nu_protocol::Spanned;
use tokio::runtime::{Builder, Runtime};

pub fn tokio_block02() -> Result<(), std::io::Error> {
    use influxdb_iox_client::{
        connection::Builder,
        flight::{generated_types::ReadInfo, Client},
    };

    let num_threads: Option<usize> = None;

    let tokio_runtime = get_runtime(num_threads)?;

    tokio_runtime.block_on(async move {
        let connection = Builder::default()
            .build("http://127.0.0.1:8082")
            .await
            .expect("client should be valid");

        let mut client = Client::new(connection);

        let mut query_results = client
            .perform_query(ReadInfo {
                namespace_name: "postgresql:///iox_shared".to_string(),
                sql_query: "select * from h2o_temperature".to_string(),
            })
            .await
            .expect("query request should work");

        let mut batches = vec![];

        while let Some(data) = query_results.next().await.expect("valid batches") {
            batches.push(data);
        }

        println!("{:?}", batches);
    });

    Ok(())
}

pub fn tokio_block01() -> Result<(), std::io::Error> {
    use influxdb_iox_client::{connection::Builder, health::Client};

    let num_threads: Option<usize> = None;

    let tokio_runtime = get_runtime(num_threads)?;
    tokio_runtime.block_on(async move {
        let connection = Builder::default()
            .build("http://127.0.0.1:8082")
            .await
            .unwrap();

        let mut client = Client::new(connection);

        let x = client.check_storage().await.expect("check_storage failure");
        println!("{:?}", x);
    });

    Ok(())
}

/// Creates the tokio runtime for executing IOx
///
/// if nthreads is none, uses the default scheduler
/// otherwise, creates a scheduler with the number of threads
pub fn get_runtime(num_threads: Option<usize>) -> Result<Runtime, std::io::Error> {
    // NOTE: no log macros will work here!
    //
    // That means use eprintln!() instead of error!() and so on. The log emitter
    // requires a running tokio runtime and is initialised after this function.

    //use tokio::runtime::Builder;
    let kind = std::io::ErrorKind::Other;
    match num_threads {
        None => Runtime::new(),
        Some(num_threads) => {
            println!(
                "Setting number of threads to '{}' per command line request",
                num_threads
            );

            match num_threads {
                0 => {
                    let msg = format!(
                        "Invalid num-threads: '{}' must be greater than zero",
                        num_threads
                    );
                    Err(std::io::Error::new(kind, msg))
                }
                1 => Builder::new_current_thread().enable_all().build(),
                _ => Builder::new_multi_thread()
                    .enable_all()
                    .worker_threads(num_threads)
                    .build(),
            }
        }
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
