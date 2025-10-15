use bevy::prelude::*;

use crate::state::GlobalState;

pub fn create_shader(
    shaders: &mut Assets<Shader>,
    global_state: &mut GlobalState,
    code: &str,
) -> Handle<Shader> {
    let fill_shader_code = add_unique_shader_import_path(code, global_state);
    let fill_shader = Shader::from_wgsl(fill_shader_code, file!());
    shaders.add(fill_shader)
}

fn add_unique_shader_import_path(code: &str, global_state_state: &mut GlobalState) -> String {
    let id = global_state_state.create_shader();
    let import_path_directive = "#define_import_path ";
    let unique_shader_import_path =
        format!("{import_path_directive}smud_demo::temp::shader_{id}\n");
    let mut result = unique_shader_import_path;
    for line in code.lines() {
        if !line.contains("#define_import_path") {
            result.push_str(line);
            result.push('\n');
        }
    }
    result
}
