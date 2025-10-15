#define_import_path smud_demo::fill::cubic_falloff

#import smud

// Same as bevy_smud::DEFAULT_FILL_HANDLE
// Source: https://github.com/johanhelsing/bevy_smud/blob/main/assets/fills/cubic_falloff.wgsl

fn fill(input: smud::FillInput) -> vec4<f32> {
    let d = input.distance;
    let c = input.color;
    let d2 = 1. - (d * 0.13);
    let alpha = clamp(d2 * d2 * d2, 0., 1.) * c.a;
    let shadow_color = 0.2 * c.rgb;
    let aaf = 0.7 / fwidth(d);
    let final_color = mix(c.rgb, shadow_color, clamp(d * aaf, 0., 1.));
    return vec4(final_color, alpha);
}
