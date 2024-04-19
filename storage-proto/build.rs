fn main() -> Result<(), std::io::Error> {
    const PROTOC_ENVAR: &str = "PROTOC";
    if std::env::var(PROTOC_ENVAR).is_err() {
        #[cfg(not(windows))]
        std::env::set_var(PROTOC_ENVAR, protobuf_src::protoc());
    }

    let proto_base_path = std::path::PathBuf::from("proto");
    let proto_files = ["confirmed_block.proto", "transaction_by_addr.proto"];
    let mut protos = Vec::new();
    for proto_file in &proto_files {
        let proto = proto_base_path.join(proto_file);
        // qtrade
        // error: unsupported output in build script of `solana-storage-proto v1.14.6 (/workspaces/qtrade/solana/execution/storage-proto)`: `cargo::rerun-if-changed=proto/confirmed_block.proto`
        // Found a `cargo::key=value` build directive which is reserved for future use.
        // Either change the directive to `cargo:key=value` syntax (note the single `:`) or upgrade your version of Rust.
        // See https://doc.rust-lang.org/cargo/reference/build-scripts.html#outputs-of-the-build-script for more information about build script outputs.
        // println!("cargo::rerun-if-changed={}", proto.display());
        println!("cargo:rerun-if-changed={}", proto.display());
        protos.push(proto);
    }

    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .type_attribute(
            "TransactionErrorType",
            "#[cfg_attr(test, derive(enum_iterator::IntoEnumIterator))]",
        )
        .type_attribute(
            "InstructionErrorType",
            "#[cfg_attr(test, derive(enum_iterator::IntoEnumIterator))]",
        )
        .compile(&protos, &[proto_base_path])
}
