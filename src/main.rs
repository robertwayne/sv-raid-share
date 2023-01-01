use copypasta::{ClipboardContext, ClipboardProvider};
use screenshots::Screen;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Grab global clipboard context
    let mut ctx = ClipboardContext::new().expect("Failed to create clipboard context");

    // Keep a copy of the last raid code
    let mut active_raid_code = String::new();

    // Attempt to get the EVGA XR1s output device
    let screens = Screen::all().expect("Failed to find any screen devices");
    let target_screen = screens.get(2).expect("Failed to find XR1 screen device");

    loop {
        // Capture the raid code on screen
        let target_area = target_screen.capture_area(600, 200, 300, 75);

        if let Some(screenshot) = target_area {
            let buffer = screenshot.buffer();
            std::fs::write("out/capture.png", buffer)?;
        }

        if let Ok(mut lt) = leptess::LepTess::new(None, "eng") {
            if lt.set_image("out/capture.png").is_err() {
                continue;
            }

            let raid_code = lt.get_utf8_text()?;
            let raid_code = raid_code.trim().replace(" ", "");

            // Ignore any text that isn't a raid code or is the same as the last
            // one
            if raid_code.len() != 6
                || raid_code == active_raid_code
                || !raid_code.chars().all(|c| c.is_alphanumeric())
            {
                continue;
            }

            // Otherwise, update the active raid code
            active_raid_code = raid_code.to_owned();

            // ...and copy it to the clipboard
            println!("Raid Code '{}' copied to clipboard.", raid_code);
            if let Err(e) = ctx.set_contents(raid_code.replace('O', "0").to_owned()) {
                println!("Error copying to clipboard: {e}");
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
