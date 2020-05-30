use yasl_macro::yasl_vert;

#[yasl_vert]
note_vert! {
    layout<input,0> i_color: vec3<f32>;
    layout<input,1> i_uv: vec2<f32>;
    layout<input,2> i_size: vec2<f32>;
    layout<input,3> i_radius: f32;

    static c: f32 = 0.0;

    fn add(a: f32, b: f32) -> f32{
        return a + b;
    }

    fn main() {
        let b = c;
        let d = add(1.0,2.0);


        // let color: vec3<f32> = i_color;

        // let alpha: f32 = 1.0;

        // let pos: vec2<f32> = i_uv * i_size;


        // let xMax: f32 = i_size.x - i_radius;

        // let a = 0.0 + 1.0;
        // let b = c;
        // let b = a;

//
        // let a: f32 = add(1.0,1.0);
        // let num: i32 = 1.0 as i32;

        // {
        //     let num3: f32 = 0.0;
        // }

        // let num2: f32 = a;

        // num2 = 1.0;

        // if num2 == 1.0 {
        //     num2 += 1.0;
        // } else if true{
        //     num2 = 1.0;
        // } else{
        //     num2 = 0.0;
        // }

        // let f : vec2<f32> = vec2(1.0,1.0);

        // let f1 : vec2<f32> = f32::vec2(1.0,1.0);
        // let f2 : vec2<f64> = f64::vec2(0.0,0.0);

        // let i1 : vec2<i32> = i32::vec2(0,0);
        // let i2 : vec2<u32> = u32::vec2(0,0);

        // glsl::gl_Position = vec4(0.0,0.0,0.0,1.0);
    }
}

fn main() {
    println!("Hello, world!");
}
