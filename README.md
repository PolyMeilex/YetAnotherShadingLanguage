![YASL](https://i.imgur.com/WKXKh1V.png)
# Yet Another Shading Language
YASL is yet another shading language, that's all there is to it

# WIP!

## Example Usage With Rust Macro
```rust
yasl_vert! {
    static num: i32 = 0 + 1;

    layout<input,0> pos: vec2<f32>;

    fn add(a: f32,b: f32) -> f32{
        a + b
    }

    fn main() {
        let a: f32 = add(1.0,1.0);
        let num: i32 = 1.0 as i32;


        let num2: f32 = a;

        num2 = 1.0;
        num2 += 1.0;


        // TODO: Yasl should have its own vec type
        let f1 : vec2<f32> = glsl::vec2(1.0,1.0);
        let f2 : vec2<f64> = glsl::vec2(0.0,0.0);
        let i1 : vec2<i32> = glsl::ivec2(0,0);
        let i2 : vec2<u32> = glsl::uvec2(0,0);

    }
}
```
