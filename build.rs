use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=assets/icon.ico");

    #[cfg(target_os = "windows")]
    {
        let icon_path = Path::new("assets/icon.ico");

        if icon_path.exists() {
            winresource::WindowsResource::new()
                .set_icon(icon_path.to_string_lossy().as_ref())
                .compile()
                .expect("No s'ha pogut aplicar la icona de Windows");
        }
    }
}
