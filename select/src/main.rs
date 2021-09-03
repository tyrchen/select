use std::env;

use anyhow::Result;
use queryer::query;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    env::set_var("POLARS_FMT_MAX_COLS", "16");
    env::set_var("POLARS_FMT_MAX_ROWS", "128");
    let args: Vec<String> = env::args().collect();
    let mut sql = String::from("select ");
    sql.push_str(args[1..].join(" ").as_str());
    let data = query(sql).await?;

    println!("{:?}", data);

    Ok(())
}
