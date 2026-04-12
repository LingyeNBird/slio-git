fn main() {
    if std::env::var("CARGO_CFG_WINDOWS").is_ok() {
        let mut resource = winresource::WindowsResource::new();
        resource.set_icon("assets/AppIcon.ico");
        resource
            .compile()
            .expect("failed to compile Windows icon resource");
    }
}
