// WGSL Compute Shader for parallel computation
// Demonstrates GPU compute capabilities in WebAssembly

@group(0) @binding(0)
var<storage, read> input_data: array<f32>;

@group(0) @binding(1)
var<storage, read_write> output_data: array<f32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    
    if (index >= arrayLength(&input_data)) {
        return;
    }
    
    let x = input_data[index];
    
    // Compute f(x) = x * x + x + 1.0
    output_data[index] = x * x + x + 1.0;
}