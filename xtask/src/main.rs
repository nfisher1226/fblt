use {
    clap_complete::{generate_to, shells},
    clap_mangen::Man,
    std::{env, error::Error, fs, path::PathBuf, process},
    tiny_skia::Transform,
    usvg::{FitTo, Options, Tree},
};

include!("../../src/cli/opts.rs");

fn completions() -> Result<(), Box<dyn Error>> {
    println!("Generating completions:");
    let mut cmd = build();
    let outdir: PathBuf = ["target", "dist", "share", "bash-completion", "completions"]
        .iter()
        .collect();
    if !outdir.exists() {
        fs::create_dir_all(&outdir)?;
    }
    let path = generate_to(shells::Bash, &mut cmd, "gfret", outdir)?;
    println!("    {}", path.display());
    let outdir: PathBuf = ["target", "dist", "share", "zsh", "site-functions"]
        .iter()
        .collect();
    if !outdir.exists() {
        fs::create_dir_all(&outdir)?;
    }
    let path = generate_to(shells::Zsh, &mut cmd, "gfret", outdir)?;
    println!("    {}", path.display());
    let outdir: PathBuf = ["target", "dist", "share", "fish", "completions"]
        .iter()
        .collect();
    if !outdir.exists() {
        fs::create_dir_all(&outdir)?;
    }
    let path = generate_to(shells::Fish, &mut cmd, "gfret", outdir.to_path_buf())?;
    println!("    {}", path.display());
    // Disabling this for now because I don't know where powershell looks for completions
    let outdir: PathBuf = ["target", "dist", "share", "pwsh", "completions"]
        .iter()
        .collect();
    if !outdir.exists() {
        fs::create_dir_all(&outdir)?;
    }
    let path = generate_to(shells::PowerShell, &mut cmd, "gfret", outdir.to_path_buf())?;
    println!("    {}", path.display());
    Ok(())
}

fn manpage() -> Result<(), Box<dyn Error>> {
    println!("Creating man pages:");
    let cmd = build();
    let outdir: PathBuf = ["target", "dist", "share", "man", "man1"].iter().collect();
    if !outdir.exists() {
        fs::create_dir_all(&outdir)?;
    }
    let file: PathBuf = [outdir.to_str().unwrap(), "gfret.1"].iter().collect();
    let man = Man::new(cmd);
    let mut buffer: Vec<u8> = Vec::new();
    man.render(&mut buffer)?;
    std::fs::write(&file, buffer)?;
    println!("    {}", file.display());
    let cmd = build_cli();
    let file: PathBuf = [outdir.to_str().unwrap(), "gfret-cli.1"].iter().collect();
    let man = Man::new(cmd);
    let mut buffer: Vec<u8> = Vec::new();
    man.render(&mut buffer)?;
    std::fs::write(&file, buffer)?;
    println!("    {}", file.display());
    Ok(())
}

fn png(tree: &Tree, size: u32) -> Result<(), Box<dyn Error>> {
    let fit = FitTo::Size(size, size);
    let transform = Transform::from_scale(1.0, 1.0);
    let mut pixmap = match tiny_skia::Pixmap::new(size, size) {
        Some(p) => p,
        None => return Err(String::from("Error creating png").into()),
    };
    resvg::render(&tree, fit, transform, pixmap.as_mut());
    let sizedir = format!("{}x{}", size, size);
    let outdir: PathBuf = [
        "target", "dist", "share", "icons", "hicolor", &sizedir, "apps",
    ]
    .iter()
    .collect();
    if !outdir.exists() {
        fs::create_dir_all(&outdir)?;
    }
    let mut outfile = outdir;
    outfile.push("gfret.png");
    let infile: PathBuf = ["data", "gfret.svg"].iter().collect();
    println!("    {} -> {}", infile.display(), outfile.display());
    pixmap.save_png(outfile)?;
    Ok(())
}

fn iconvert() -> Result<(), Box<dyn Error>> {
    println!("Creating png icons from svg:");
    let infile: PathBuf = ["data", "gfret.svg"].iter().collect();
    let data = fs::read(&infile)?;
    let tree = Tree::from_data(&data, &Options::default().to_ref())?;
    for size in [128, 64, 48, 32] {
        png(&tree, size)?;
    }
    Ok(())
}

fn copy_data() -> Result<(), Box<dyn Error>> {
    println!("Copying data files:");
    let appdir: PathBuf = ["target", "dist", "share", "applications"].iter().collect();
    if !appdir.exists() {
        fs::create_dir_all(&appdir)?;
    }
    let mut outfile = appdir;
    outfile.push("gfret.desktop");
    let infile: PathBuf = ["data", "gfret.desktop"].iter().collect();
    fs::copy(&infile, &outfile)?;
    println!("    {} -> {}", infile.display(), outfile.display());
    let icondir: PathBuf = [
        "target", "dist", "share", "icons", "hicolor", "scalable", "apps",
    ]
    .iter()
    .collect();
    if !icondir.exists() {
        fs::create_dir_all(&icondir)?;
    }
    let mut outfile = icondir;
    outfile.push("gfret.svg");
    let infile: PathBuf = ["data", "gfret.svg"].iter().collect();
    fs::copy(&infile, &outfile)?;
    println!("    {} -> {}", infile.display(), outfile.display());
    let schemadir: PathBuf = ["target", "dist", "share", "glib-2.0", "schemas"].iter().collect();
    if !schemadir.exists() {
        fs::create_dir_all(&schemadir)?;
    }
    let mut outfile = schemadir;
    outfile.push("org.hitchhiker_linux.gfret.gschema.xml");
    let infile: PathBuf = ["data", "org.hitchhiker_linux.gfret.gschema.xml"].iter().collect();
    fs::copy(&infile, &outfile)?;
    println!("    {} -> {}", infile.display(), outfile.display());
    Ok(())
}

fn copy_bin() -> Result<(), Box<dyn Error>> {
    println!("Copying binary:");
    let bindir: PathBuf = ["target", "dist", "bin"].iter().collect();
    if !bindir.exists() {
        fs::create_dir_all(&bindir)?;
    }
    let mut outfile = bindir;
    outfile.push("gfret");
    let infile: PathBuf = ["target", "release", "gfret"].iter().collect();
    if !infile.exists() {
        eprintln!("Error: you must run \"cargo build --release\" first");
    }
    fs::copy(&infile, &outfile)?;
    println!("    {} -> {}", infile.display(), outfile.display());
    Ok(())
}

fn compile_translation(potfile: &str, lang: &str) -> Result<(), Box<dyn Error>> {
    let infile: PathBuf = ["po", potfile].iter().collect();
    let lcdir: PathBuf = ["target", "dist", "share", "locale", lang, "LC_MESSAGES"]
        .iter()
        .collect();
    if !lcdir.exists() {
        fs::create_dir_all(&lcdir)?;
    }
    let mut outfile = lcdir.clone();
    outfile.push("zond.mo");
    let output = process::Command::new("msgfmt")
        .args([
            infile.to_str().unwrap(),
            "-o",
            outfile.to_str().unwrap(),
        ])
        .output()?;
    assert!(output.status.success());
    println!("    {} -> {}", infile.display(), outfile.display());
    Ok(())
}

fn translations() -> Result<(), Box<dyn Error>> {
    //println!("Compiling translations:");
    //compile_translation("it.po", "it")?;
    Ok(())
}

fn usage() {
    println!("Usage: xtask dist");
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage();
        process::exit(0);
    }
    if &args[1] == "dist" {
        let outdir: PathBuf = ["target", "dist"].iter().collect();
        if outdir.exists() {
            fs::remove_dir_all(&outdir)?;
        }
        copy_bin()?;
        copy_data()?;
        completions()?;
        manpage()?;
        iconvert()?;
        translations()?;
    } else {
        usage();
    }
    Ok(())
}
