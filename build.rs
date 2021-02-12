use mmrbi::*;

use std::fs;
use std::path::*;
use std::process::Command;

struct FxcShaderCompiler {
    fxc_exe: PathBuf,
}

impl FxcShaderCompiler {
    fn new () -> Self {
        let fsc = Self {
            // XXX: I should seriously consider using D3DCompile for better portability.
            fxc_exe: mmrbi::fs::windows::find_fxc_exe().unwrap_or_else(|err| fatal!("unable to find fxc.exe: {}", err)),
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
