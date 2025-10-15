#define_import_path smud_demo::sdf::heart

#import smud

fn sdf(input: smud::SdfInput) -> f32 {
    let p = input.pos;
    let s = input.params.x + 150.;
    return smud::sd_heart((p / s) - vec2(0., -0.5)) * s;
}
