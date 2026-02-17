fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
		res.set("ProductName", "L7 Surface Mapper");
		res.set("FileDescription", "High-performance stack intelligence engine");
		res.set("CompanyName", "JDarkness / RedTeamingTools");
		res.set("LegalCopyright", "MIT License");
        res.set_icon("assets/App.ico");
        res.compile().unwrap();
    }
}