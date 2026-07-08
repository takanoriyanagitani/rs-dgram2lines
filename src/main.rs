use std::io;
use std::process::ExitCode;

fn path2dgram() -> Result<String, io::Error> {
    std::env::var("ENV_PATH2DGRAM")
        .map_err(|e| format!("env var ENV_PATH2DGRAM invalid: {e}"))
        .map_err(io::Error::other)
}

fn sub() -> Result<(), io::Error> {
    rs_dgram2lines::path2dgram2bytes2lines2stdout_forever_default(path2dgram()?)
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
