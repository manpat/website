fn main() {
	if std::env::var("PROFILE").unwrap() == "debug" {
		println!("cargo:rustc-cfg=debug");
	}
}
