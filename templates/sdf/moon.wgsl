#define_import_path smud_demo::sdf::moon

#import smud

fn sdf(input: smud::SdfInput) -> f32 {
    let p = input.pos;
    let d = input.params.x + 50.;
    let ra = input.params.y + 125.;
    let rb = input.params.z + 100.;
    return smud::sd_moon(p, d, ra, rb);
}
