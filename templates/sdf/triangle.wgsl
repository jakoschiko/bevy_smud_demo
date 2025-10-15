#define_import_path smud_demo::sdf::triangle

#import smud

fn sdf(input: smud::SdfInput) -> f32 {
    let p = input.pos;
    let p0 = (input.params.x + 100.) * vec2(0., 1.);
    let p1 = (input.params.y + 100.) * vec2(-0.866, -0.5);
    let p2 = (input.params.z + 100.) * vec2(0.866, -0.5);
    return smud::sd_triangle(p, p0, p1, p2);
}
