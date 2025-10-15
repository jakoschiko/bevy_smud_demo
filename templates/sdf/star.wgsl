#define_import_path smud_demo::sdf::star

#import smud

fn sdf(input: smud::SdfInput) -> f32 {
    let p = input.pos;
    let r = input.params.x + 100.;
    let n = max(i32(input.params.y), 0) + 4;
    let m = (input.params.z / 100.) + 3.;
    return smud::sd_star(p * 0.5, r, n, m);
}
