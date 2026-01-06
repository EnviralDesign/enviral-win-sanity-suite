//! Build script for Windows resource embedding

fn main() {
    // Only run on Windows
    #[cfg(target_os = "windows")]
    {
        // Look for icon source file (PNG or JPG)
        let icon_png = std::path::Path::new("assets/icon.png");
        let icon_jpg = std::path::Path::new("assets/icon.jpg");
        let icon_ico = std::path::Path::new("assets/icon.ico");

        // Determine which source file exists
        let icon_source = if icon_png.exists() {
            Some(icon_png)
        } else if icon_jpg.exists() {
            Some(icon_jpg)
        } else {
            None
        };

        if let Some(source) = icon_source {
            // Convert source image to ICO
            match image::open(source) {
                Ok(img) => {
                    // Resize to 256x256 for ICO compatibility
                    let img = img.resize(256, 256, image::imageops::FilterType::Lanczos3);
                    if let Err(e) = img.save(icon_ico) {
                        println!("cargo:warning=Failed to save icon.ico: {}", e);
                    }
                },
                Err(e) => println!("cargo:warning=Failed to open {}: {}", source.display(), e),
            }
        } else {
            println!("cargo:warning=No icon source file found (assets/icon.png or assets/icon.jpg)");
        }

        // Embed Windows application manifest and icon
        let mut res = winres::WindowsResource::new();
        // Check if icon exists before trying to set it to avoid build error
        if icon_ico.exists() {
            res.set_icon("assets/icon.ico");
        }
        res.set("ProductName", "Sanity Suite");
        res.set("FileDescription", "Windows Diagnostic Utility");
        res.set("LegalCopyright", "Copyright © 2024");

        if let Err(e) = res.compile() {
            println!("cargo:warning=Failed to compile Windows resources: {}", e);
        }
    }
}
