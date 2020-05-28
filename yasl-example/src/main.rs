use yasl_macro::yasl_vert;

yasl_vert! {
    static num: i32 = 0 + 1;

    layout<input,0> pos: vec2<f32>;

    fn add(a: f32,b: f32) -> f32{
        return a + b;
    }

    fn main() {
        let a: f32 = add(1.0,1.0);
        let num: i32 = 1.0 as i32;

        {
            let num3: f32 = 0.0;
        }

        let num2: f32 = a;

        num2 = 1.0;

        if num2 == 1.0 {
            num2 += 1.0;
        } else if true{
            num2 = -1.0;
        } else{
            num2 = 0.0;
        }

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
