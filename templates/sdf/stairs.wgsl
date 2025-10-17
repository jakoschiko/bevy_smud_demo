#define_import_path smud_demo::sdf::stairs

#import smud

fn sdf(input: smud::SdfInput) -> f32 {
    let p = input.pos;
    let s = max(input.params.x + 150., 0.1);
    let n = max(floor(input.params.y / 10.) + 4., 1.);
    let a = max(input.params.z / 10. + 1.0, 0.1);
    let b = min(max(input.params.w / 10. + 1.0, 0.1), a);
    let ab = vec2(a, b);
    let sn = s / n;
    let p2 = input.pos + ab * s / 2.;
    return smud::sd_stairs(p2 / sn, ab, n) * sn;
}
