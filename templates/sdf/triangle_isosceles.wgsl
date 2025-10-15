#define_import_path smud_demo::sdf::triangle_isosceles

#import smud

fn sdf(input: smud::SdfInput) -> f32 {
    let p = input.pos;
    let qx = input.params.x + 100.;
    let qy = input.params.y + 100.;
    let p2 = p + vec2(0., qy / 2.);
    return smud::sd_triangle_isosceles(p2, vec2(qx, qy));
}
