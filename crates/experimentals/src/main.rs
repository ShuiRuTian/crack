use futures::StreamExt;
use tokio::io::{self};
use tokio_util::codec::{FramedRead, LinesCodec};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tmp1 = do_the_line();
    let tmp2 = do_the_line();

    tokio::join!();
    futures::join!();
    Ok(())
}

async fn do_the_line() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut reader = FramedRead::new(stdin, LinesCodec::new());
    let line = reader.next().await.transpose()?.unwrap();
    println!("You typed: {}", line);
    Ok(())
}
