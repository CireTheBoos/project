use std::path::PathBuf;

use shaderc::{Compiler, ShaderKind};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

const ENTRY: &'static str = "main";

/////////////////////////////////////////////////////////////////////////////
// Main
/////////////////////////////////////////////////////////////////////////////

fn main() -> Result<()> {
    let compiler = Compiler::new()?;

    // vertex
    compile_glsl_into_spv(
        &compiler,
        GlslFile {
            path: PathBuf::from("shaders/vertex.glsl"),
            kind: ShaderKind::Vertex,
        },
    )?;

    // fragment
    compile_glsl_into_spv(
        &compiler,
        GlslFile {
            path: PathBuf::from("shaders/fragment.glsl"),
            kind: ShaderKind::Fragment,
        },
    )?;

    // geometry
    compile_glsl_into_spv(
        &compiler,
        GlslFile {
            path: PathBuf::from("shaders/geometry.glsl"),
            kind: ShaderKind::Geometry,
        },
    )?;

    Ok(())
}

fn compile_glsl_into_spv(compiler: &Compiler, mut file: GlslFile) -> Result<()> {
    // read glsl
    let glsl_code = std::fs::read_to_string(file.path.to_str().unwrap())?;

    // compile
    let binary = compiler.compile_into_spirv(
        &glsl_code,
        file.kind,
        file.path.file_name().unwrap().to_str().unwrap(),
        ENTRY,
        None,
    )?;
    let compiled_code = binary.as_binary_u8();

    // write spv
    file.path.set_extension("spv");
    std::fs::write(file.path, compiled_code)?;

    Ok(())
}

pub struct GlslFile {
    pub path: PathBuf,
    pub kind: ShaderKind,
}
