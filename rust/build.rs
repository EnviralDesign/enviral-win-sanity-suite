//! Build script for Windows resource embedding

fn main() {
    // Only run on Windows
    #[cfg(target_os = "windows")]
    {
        // Embed Windows application manifest and icon
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.set("ProductName", "Sanity Suite");
        res.set("FileDescription", "Windows Diagnostic Utility");
        res.set("LegalCopyright", "Copyright © 2024");

        if let Err(e) = res.compile() {
            eprintln!("Warning: Failed to compile Windows resources: {}", e);
        }
    }
}
