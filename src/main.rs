use screenshots::Screen;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let screens = Screen::all()?;

    if screens.is_empty() {
        eprintln!("No screens found");
        return Ok(());
    }

    let screen = &screens[0];
    println!(
        "Capturing screen: {}x{}",
        screen.display_info.width, screen.display_info.height
    );

    let image = screen.capture()?;
    image.save("screenshot.png")?;
    println!("Screenshot saved as screenshot.png");

    Ok(())
}
