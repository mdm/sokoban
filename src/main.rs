mod levels;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let levels = levels::LevelCollection::from_file("levels/Thinking-Rabbit-Original-Plus-Extra.txt")?;
    let levels = levels::LevelCollection::from_file("levels/single.txt")?;

    for (i, level) in levels.into_iter().enumerate() {
        println!("\nLevel {}", i);
        dbg!(level.walls().collect::<Vec<_>>());
    }

    Ok(())
}
