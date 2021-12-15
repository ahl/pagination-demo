include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

#[tokio::main]
async fn main() -> Result<(), String> {
    let client = Client::new("http://localhost:8080");
    let mut stream = client.list_words_stream(Some(u32::MAX.try_into().unwrap()));
    loop {
        use futures::TryStreamExt;

        match stream.try_next().await {
            Err(e) => {
                println!("oh noes! {}", e);
                break;
            }
            Ok(None) => {
                println!("done");
                break;
            }
            Ok(Some(x)) => println!("{:?}", x),
        }
    }

    println!(
        "generated client: {}",
        concat!(env!("OUT_DIR"), "/codegen.rs")
    );

    Ok(())
}
