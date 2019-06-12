use std::env;
use std::fs;
use std::path::*;
use std::process::Command;

fn program_files_x86 () -> PathBuf {
    if let Ok(path) = env::var("ProgramFiles(x86)") { return PathBuf::from(path); }
    if let Ok(path) = env::var("ProgramFiles")      { return PathBuf::from(path); }
    panic!("Couldn't find Program Files");
}

struct FxcShaderCompiler {
    fxc_exe: PathBuf,
}

impl FxcShaderCompiler {
    fn new () -> Self {
        let fsc = Self {
            // XXX: I should seriously consider using D3DCompile for better portability.
            fxc_exe: program_files_x86().join(r"Windows Kits\10\bin\10.0.17763.0\x64\fxc.exe"),
        };
        assert!(fsc.fxc_exe.exists());
        fsc
    }

    fn compile (&self, input: &str, output: &str, profile: &str) -> Result<(),&'static str> {
        println!("cargo:rerun-if-changed={}", input);

        if let Some(output_dir) = Path::new(output).parent() {
            if !output_dir.exists() {
                let _ = fs::create_dir(output_dir);
            }
        }

        let status = Command::new(&self.fxc_exe)
            .arg("/T").arg(profile)
            .arg("/Fo").arg(output)
            .arg(input)
            .status()
            .unwrap();

        if status.code() == Some(0) { Ok(()) }
        else { Err("fxc.exe returned nonzero") }
    }
}

fn main () {
    let scc = FxcShaderCompiler::new();
    scc.compile(r"res\ps.hlsl", r"target\assets\ps.bin", "ps_5_0").unwrap();
    scc.compile(r"res\vs.hlsl", r"target\assets\vs.bin", "vs_5_0").unwrap();
}
