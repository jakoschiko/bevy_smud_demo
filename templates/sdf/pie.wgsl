#define_import_path smud_demo::sdf::pie

#import smud

fn sdf(input: smud::SdfInput) -> f32 {
    let p = input.pos;
    let r = input.params.x + 100.;
    let c = (input.params.y + 80) / 100.;
    return smud::sd_pie(p, smud::sin_cos(c), r);
}
