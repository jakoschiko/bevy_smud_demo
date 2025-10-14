#define_import_path smud_demo::fill::simple

#import smud

fn fill(input: smud::FillInput) -> vec4<f32> {
    let d = input.distance;
    let c = input.color;
    let a = smud::sd_fill_alpha_fwidth(d);
    return vec4<f32>(c.rgb, a * c.a);
}
