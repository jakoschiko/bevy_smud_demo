#define_import_path smud_demo::fill::outline

#import smud

// Source: https://github.com/johanhelsing/bevy_smud/blob/main/assets/fills/outline.wgsl

fn fill(input: smud::FillInput) -> vec4<f32> {
    let d = input.distance;
    let c = input.color;
    let d2 = abs(d - 1.) - 1.;
    let a = smud::sd_fill_alpha_fwidth(d2);
    return vec4(c.rgb, a * c.a);
}
