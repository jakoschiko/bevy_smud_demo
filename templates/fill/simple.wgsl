#define_import_path smud_demo::fill::simple

#import smud

// Same as bevy_smud::SIMPLE_FILL_HANDLE
// Source: https://github.com/johanhelsing/bevy_smud/blob/main/assets/fills/simple.wgsl

fn fill(input: smud::FillInput) -> vec4<f32> {
    let d = input.distance;
    let c = input.color;
    let a = smud::sd_fill_alpha_fwidth(d);
    return vec4(c.rgb, a * c.a);
}
