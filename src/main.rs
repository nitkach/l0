use futures::{FutureExt, TryFutureExt};
use log::error;
use std::{panic::AssertUnwindSafe, process::ExitCode};

#[tokio::main]
async fn main() -> ExitCode {
    let website = AssertUnwindSafe(async {
        let result = l0::run().await;

        match result {
            Ok(()) => ExitCode::SUCCESS,
            Err(err) => {
                eprintln!("{err}");
                ExitCode::FAILURE
            }
        }
    })
    .catch_unwind()
    .unwrap_or_else(|_| {
        error!("Exiting due to a panic...");
        ExitCode::FAILURE
    });

    website.await
}
