#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let d = evdev::Device::open("/dev/input/event0").unwrap();
    println!("{}", d);
    println!("Events:");
    let mut events = d.into_event_stream()?;
    loop {
        let ev = events.next_event().await?;
        println!("{:?}", ev);
    }
}
