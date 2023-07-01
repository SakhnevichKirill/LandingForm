use dotenvy::dotenv;
use landing_form::run;

#[tokio::main]
async fn main() {
    // Load environmental variables.
    dotenv().ok();

    // Run the main function which makes the server up and
    // continues until termination.
    run().await;
}
