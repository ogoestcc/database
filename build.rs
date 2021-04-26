fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=protos/database/");

        tonic_build::configure()
        .build_client(false)
        .compile(
            &["protos/database/users.proto", "protos/database/alerts.proto"],
            &["protos/database"],
        )?;

    // tonic_build::compile_protos("protos/database/users.proto")?;
    // tonic_build::compile_protos("protos/database/alerts.proto")?;
    Ok(())
}
