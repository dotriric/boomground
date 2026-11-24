//! Windows : intègre l'icône dans l'exécutable (explorateur, barre des
//! tâches, raccourcis). Sans effet ailleurs.

fn main() {
    // Canal de distribution : `BOOM_CHANNEL=early-access` à la compilation
    // marque le binaire (titre de fenêtre « early access ») — utilisé par le
    // workflow pour les builds du canal itch.io restreint au Discord.
    println!("cargo:rerun-if-env-changed=BOOM_CHANNEL");
    if std::env::var("BOOM_CHANNEL").as_deref() == Ok("early-access") {
        println!("cargo:rustc-cfg=boom_early_access");
    }
    println!("cargo:rerun-if-changed=assets/icon/boomground.ico");
    #[cfg(windows)]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/icon/boomground.ico");
        res.set("ProductName", "BoomGround");
        res.set("FileDescription", "BoomGround — destruction de voxels");
        if let Err(e) = res.compile() {
            println!("cargo:warning=icône Windows non intégrée : {e}");
        }
    }
}
