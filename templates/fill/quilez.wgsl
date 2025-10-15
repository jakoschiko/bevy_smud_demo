#define_import_path smud_demo::fill::quilez

#import smud

// Similar to the coloring used by Inigo Quilez
// Source: https://iquilezles.org/articles/distfunctions2d

fn fill(input: smud::FillInput) -> vec4<f32> {
    let p = input.pos;
    let d = input.distance;
    let c = input.color;
    var rgb = c.rgb;
    if (d > 0.) {
        rgb = vec3(1.);
    }
    rgb *= 1. - exp(-0.006 * abs(d));
    rgb *= 0.8 + 0.4 * cos(1.5 * d);
    rgb = mix(rgb, vec3(1.), 1. - smoothstep(0., 1., abs(d)));
    return vec4(rgb, 1.);
}
