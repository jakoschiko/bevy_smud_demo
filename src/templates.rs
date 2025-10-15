use bevy::prelude::*;

use crate::{consts, state::ShaderKind};

#[derive(Resource)]
pub struct Templates {
    sdf: Vec<Template>,
    fill: Vec<Template>,
    default_sdf: Option<usize>,
    default_fill: Option<usize>,
}

impl Default for Templates {
    fn default() -> Self {
        let sdf: Vec<_> = consts::SDF_TEMPLATE_DIR
            .files()
            .map(Template::new)
            .collect();
        let fill: Vec<_> = consts::FILL_TEMPLATE_DIR
            .files()
            .map(Template::new)
            .collect();
        let default_sdf = sdf
            .iter()
            .position(|t| t.name == consts::DEFAULT_SDF_TEMPLATE);
        let default_fill = fill
            .iter()
            .position(|t| t.name == consts::DEFAULT_FILL_TEMPLATE);
        Self {
            sdf,
            fill,
            default_sdf,
            default_fill,
        }
    }
}

impl Templates {
    pub fn all_templates(&self, shader: ShaderKind) -> &[Template] {
        match shader {
            ShaderKind::Sdf => &self.sdf,
            ShaderKind::Fill => &self.fill,
        }
    }

    pub fn default_template(&self, shader: ShaderKind) -> Option<&Template> {
        let (all, index) = match shader {
            ShaderKind::Sdf => (&self.sdf, self.default_sdf),
            ShaderKind::Fill => (&self.fill, self.default_fill),
        };
        index.and_then(|i| all.get(i))
    }
}

pub struct Template {
    pub name: String,
    pub code: String,
}

impl Template {
    fn new(file: &include_dir::File) -> Self {
        let name = file
            .path()
            .file_stem()
            .expect("Template must be a .wgsl file")
            .to_string_lossy()
            .into_owned();
        let code = file
            .contents_utf8()
            .expect("Template must contain valid utf-8")
            .to_owned();
        Self { name, code }
    }
}
