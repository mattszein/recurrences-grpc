fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/recurrence_rule.proto")?;
    Ok(())
}
