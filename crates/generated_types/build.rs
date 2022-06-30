//! Compiles Protocol Buffers into native Rust types.

use std::env;
use std::path::{Path, PathBuf};

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("protos");

    generate_grpc_types(&root)?;

    Ok(())
}

/// Schema used with IOx specific gRPC requests
///
/// Creates:
///
/// - `influxdata.iox.catalog.v1.rs`
/// - `influxdata.iox.delete.v1.rs`
/// - `influxdata.iox.ingester.v1.rs`
/// - `influxdata.iox.namespace.v1.rs`
/// - `influxdata.iox.object_store.v1.rs`
/// - `influxdata.iox.predicate.v1.rs`
/// - `influxdata.iox.querier.v1.rs`
/// - `influxdata.iox.schema.v1.rs`
/// - `influxdata.iox.write.v1.rs`
/// - `influxdata.iox.write_buffer.v1.rs`
/// - `influxdata.platform.storage.rs`
fn generate_grpc_types(root: &Path) -> Result<()> {
    let catalog_path = root.join("influxdata/iox/catalog/v1");
    let delete_path = root.join("influxdata/iox/delete/v1");
    let ingester_path = root.join("influxdata/iox/ingester/v1");
    let namespace_path = root.join("influxdata/iox/namespace/v1");
    let object_store_path = root.join("influxdata/iox/object_store/v1");
    let predicate_path = root.join("influxdata/iox/predicate/v1");
    let querier_path = root.join("influxdata/iox/querier/v1");
    let schema_path = root.join("influxdata/iox/schema/v1");
    let write_buffer_path = root.join("influxdata/iox/write_buffer/v1");
    let write_summary_path = root.join("influxdata/iox/write_summary/v1");
    let storage_path = root.join("influxdata/platform/storage");

    let proto_files = vec![
        catalog_path.join("parquet_file.proto"),
        catalog_path.join("service.proto"),
        delete_path.join("service.proto"),
        ingester_path.join("parquet_metadata.proto"),
        ingester_path.join("query.proto"),
        ingester_path.join("write_info.proto"),
        namespace_path.join("service.proto"),
        object_store_path.join("service.proto"),
        predicate_path.join("predicate.proto"),
        querier_path.join("flight.proto"),
        root.join("google/longrunning/operations.proto"),
        root.join("google/rpc/error_details.proto"),
        root.join("google/rpc/status.proto"),
        root.join("grpc/health/v1/service.proto"),
        root.join("influxdata/pbdata/v1/influxdb_pb_data_protocol.proto"),
        schema_path.join("service.proto"),
        write_buffer_path.join("write_buffer.proto"),
        write_summary_path.join("write_summary.proto"),
        storage_path.join("predicate.proto"),
        storage_path.join("service.proto"),
        storage_path.join("source.proto"),
        storage_path.join("storage_common.proto"),
        storage_path.join("test.proto"),
    ];

    // Tell cargo to recompile if any of these proto files are changed
    for proto_file in &proto_files {
        println!("cargo:rerun-if-changed={}", proto_file.display());
    }

    let mut config = prost_build::Config::new();

    config
        .compile_well_known_types()
        .disable_comments(&[".google"])
        .extern_path(".google.protobuf", "::pbjson_types")
        .btree_map(&[
            ".influxdata.iox.ingester.v1.IngesterQueryResponseMetadata.unpersisted_partitions",
        ]);

    let descriptor_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("proto_descriptor.bin");
    tonic_build::configure()
        .file_descriptor_set_path(&descriptor_path)
        // protoc in ubuntu builder needs this option
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile_with_config(config, &proto_files, &[root])?;

    let descriptor_set = std::fs::read(descriptor_path)?;

    pbjson_build::Builder::new()
        .register_descriptors(&descriptor_set)?
        .build(&[
            ".influxdata.iox",
            ".influxdata.pbdata",
            ".influxdata.platform.storage",
            ".google.longrunning",
            ".google.rpc",
        ])?;

    Ok(())
}
