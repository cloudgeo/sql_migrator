mod ops;
use ops::*;

mod engines;
use engines::postgres::PostgresEngine;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    database_engine: Option<String>,

    #[arg(short, long)]
    connection_string: String,

    #[arg(short, long)]
    migrations_location: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let args = Args::parse();
    
    println!("Looking at Directory: {:?}", args.migrations_location);

    let engine: PostgresEngine;


    let database_engine = args.database_engine.unwrap_or("postgres".to_string());
    match database_engine.as_ref() {
        "postgres" => {
            engine = PostgresEngine::new(&args.connection_string).await;
        },
        _ => unimplemented!()
    }

    run::run(&args.migrations_location, &engine).await?;

    std::mem::drop(engine);

    return Ok(());
}