#define_import_path smud_demo::sdf::circle

#import smud

fn sdf(input: smud::SdfInput) -> f32 {
    let p = input.pos;
    let r = input.params.x + 100.;
    return smud::sd_circle(p, r);
}
