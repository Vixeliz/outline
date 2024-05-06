struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
}

struct Stripes {
    stripe_size: f32,
    border_size: f32,
	stroke_size: f32,
    time: f32,
    speed: f32,
	tilt: f32,
	freq: f32,
}

@group(1) @binding(0)
var t: texture_2d<f32>;

@group(1) @binding(1)
var s: sampler;

@group(3) @binding(0)
var<uniform> stripes: Stripes;

fn modulo_euclidean (a: f32, b: f32) -> f32 {
	var m = a % b;
	if (m < 0.0) {
		if (b < 0.0) {
			m -= b;
		} else {
			m += b;
		}
	}
	return m;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
	var color = vec4(0.0, 1.0, 0.0, 1.0);
	// Area to draw stripes in
	if 
	(in.uv.x > (1.0 - stripes.border_size - stripes.stroke_size) && in.uv.x < (1.0 - (stripes.stroke_size)) 
	||
	in.uv.x < (stripes.border_size + stripes.stroke_size) && in.uv.x > stripes.stroke_size 
	||
	in.uv.y > (1.0 - stripes.border_size - stripes.stroke_size) && in.uv.y < (1.0 - (stripes.stroke_size)) 
	||
	in.uv.y < (stripes.border_size + stripes.stroke_size) && in.uv.y > stripes.stroke_size) {
		let w = cos(stripes.tilt) * in.uv.x + sin(stripes.tilt) * in.uv.y - stripes.speed * stripes.time;
		if (floor(modulo_euclidean(w * stripes.freq, stripes.stripe_size)) < 0.0001) {
			color = vec4(1.0, 1.0, 1.0, 1.0);
		} else {
			color = vec4(0.0, 0.0, 0.0, 1.0);
		}
	}

	// Borders
	if !(in.uv.x < (1.0 - stripes.stroke_size) && in.uv.x > stripes.stroke_size &&
       in.uv.y < (1.0 - stripes.stroke_size)&& in.uv.y > stripes.stroke_size) 
	|| (
		(
		(in.uv.x > (1.0 - stripes.border_size - (stripes.stroke_size * 2.0)) && in.uv.x < (1.0 - stripes.border_size - stripes.stroke_size) ||
		in.uv.x < (stripes.border_size + (stripes.stroke_size * 2.0)) && in.uv.x > (stripes.border_size + stripes.stroke_size)) 
		&&
		in.uv.y < (1.0 - stripes.border_size - stripes.stroke_size) && in.uv.y > (stripes.border_size + stripes.stroke_size)
		)   
		||
		(
		(in.uv.y > (1.0 - stripes.border_size - (stripes.stroke_size * 2.0)) && in.uv.y < (1.0 - stripes.border_size - stripes.stroke_size) ||
		in.uv.y < (stripes.border_size + (stripes.stroke_size * 2.0)) && in.uv.y > (stripes.border_size + stripes.stroke_size))
		&&
		in.uv.x < (1.0 - stripes.border_size - stripes.stroke_size) && in.uv.x > (stripes.border_size + stripes.stroke_size)
		
		)
	  )  
	{
		color = vec4(0.0, 0.0, 0.0, 1.0);
	}


	return color;
}
