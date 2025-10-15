#define_import_path smud_demo::sdf::donut

#import smud

fn sdf(input: smud::SdfInput) -> f32 {
    let p = input.pos;
    let r1 = input.params.x + 100.;
    let r2 = input.params.y + 25;
    return abs(smud::sd_circle(p, r1 - r2)) - r2;
}
