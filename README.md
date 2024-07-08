# odota-rust

Copy of [OpenDota parser](https://github.com/odota/parser) in Rust.

### About

The way it works is similar - it creates vector of events that you can analyze.

### Installation

```toml
[dependencies]
odota-rust = { git = "https://github.com/Rupas1k/odota-rust" }
```

## Example

```rust
use odota_rust::prelude::*;

fn main() -> std::io::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();

    let Some(filepath) = args.get(1) else {
        eprintln!("Usage: {} <demofile>", args[0]);
        return Ok(());
    };

    let Ok(file) = std::fs::File::open(filepath) else {
        eprintln!("Failed to open file: {}", filepath);
        return Ok(());
    };

    let start = std::time::Instant::now();
    
    let replay = unsafe { memmap2::Mmap::map(&file)? };
    parse_replay(&replay)
        .expect("Parser error")
        .into_iter()
        .filter(|entry| entry.r#type.as_ref().is_some_and(|t| t == "DotaCombatlogPurchase"))
        .for_each(|x| {
            println!("[{}] {} purchased {}",
                     x.time,
                     x.targetname.unwrap(),
                     x.valuename.unwrap()
            )
        });
    
    println!("Elapsed: {:?}", start.elapsed());

    Ok(())
}
```