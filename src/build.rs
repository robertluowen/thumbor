fn main() {
    let result = prost_build::Config::new()
        .out_dir("src/pb")
        .compile_protos(&["src/abi.proto"], &["."]);

    match result {
        Ok(_) => println!("Protobuf files compiled successfully."),
        Err(e) => {
            eprintln!("Failed to compile protobuf files: {:?}", e);
            std::process::exit(1);
        }
    }
}
