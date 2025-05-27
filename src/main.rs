use screenshots::Screen;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📸 Taking screenshot in...");

    // 5 second countdown
    for i in (1..=5).rev() {
        println!("\r⏱️  {} ", i);
        thread::sleep(Duration::from_secs(1));
    }

    print!("\r📷 Taking screenshot... ");

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
    println!("✅ Screenshot saved as screenshot.png");

    Ok(())
}
