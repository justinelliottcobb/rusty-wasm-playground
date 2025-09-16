#[cfg(feature = "math")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "math")]
use nalgebra as na;

#[cfg(feature = "math")]
#[wasm_bindgen]
pub fn matrix_operations(a11: f64, a12: f64, a21: f64, a22: f64) -> String {
    let matrix = na::Matrix2::new(a11, a12, a21, a22);
    
    let determinant = matrix.determinant();
    let trace = matrix.trace();
    let transpose = matrix.transpose();
    
    let symmetric = na::Matrix2::from_fn(|i, j| {
        if i == j { matrix[(i, j)] } else { (matrix[(i, j)] + matrix[(j, i)]) / 2.0 }
    });
    let eigen = symmetric.symmetric_eigen();
    
    let inverse_result = if determinant.abs() > 1e-10 {
        match matrix.try_inverse() {
            Some(inv) => format!("Inverse:\n[{:.3}, {:.3}]\n[{:.3}, {:.3}]", 
                inv[(0,0)], inv[(0,1)], inv[(1,0)], inv[(1,1)]),
            None => "Inverse: Not invertible".to_string(),
        }
    } else {
        "Inverse: Matrix is singular (det ≈ 0)".to_string()
    };
    
    format!(
        "Matrix Analysis\n\nOriginal Matrix:\n[{:.3}, {:.3}]\n[{:.3}, {:.3}]\n\nDeterminant: {:.6}\nTrace: {:.6}\n\nTranspose:\n[{:.3}, {:.3}]\n[{:.3}, {:.3}]\n\n{}\n\nEigenvalues: [{:.4}, {:.4}]",
        a11, a12, a21, a22,
        determinant, trace,
        transpose[(0,0)], transpose[(0,1)], transpose[(1,0)], transpose[(1,1)],
        inverse_result,
        eigen.eigenvalues[0], eigen.eigenvalues[1]
    )
}

#[cfg(feature = "math")]
#[wasm_bindgen]
pub fn geometric_transformation(x: f64, y: f64, z: f64, angle_deg: f64) -> String {
    let point = na::Point3::new(x, y, z);
    let angle = angle_deg.to_radians();
    
    let rot_x = na::Rotation3::from_axis_angle(&na::Vector3::x_axis(), angle);
    let rot_y = na::Rotation3::from_axis_angle(&na::Vector3::y_axis(), angle);
    let rot_z = na::Rotation3::from_axis_angle(&na::Vector3::z_axis(), angle);
    
    let rotated_x = rot_x * point;
    let rotated_y = rot_y * point;
    let rotated_z = rot_z * point;
    
    let combined = rot_z * rot_y * rot_x * point;
    
    let scale = na::Scale3::new(1.5, 0.75, 2.0);
    let scaled = scale * point;
    
    let translation = na::Translation3::new(1.0, -2.0, 3.0);
    let translated = translation * point;
    
    format!(
        "3D Geometric Transformations\n\nOriginal point: ({:.2}, {:.2}, {:.2})\nAngle: {:.1}°\n\nRotations:\n  Around X-axis: ({:.3}, {:.3}, {:.3})\n  Around Y-axis: ({:.3}, {:.3}, {:.3})\n  Around Z-axis: ({:.3}, {:.3}, {:.3})\n  Combined (X→Y→Z): ({:.3}, {:.3}, {:.3})\n\nScaling (1.5, 0.75, 2.0): ({:.3}, {:.3}, {:.3})\nTranslation (1, -2, 3): ({:.3}, {:.3}, {:.3})",
        x, y, z, angle_deg,
        rotated_x.x, rotated_x.y, rotated_x.z,
        rotated_y.x, rotated_y.y, rotated_y.z,
        rotated_z.x, rotated_z.y, rotated_z.z,
        combined.x, combined.y, combined.z,
        scaled.x, scaled.y, scaled.z,
        translated.x, translated.y, translated.z
    )
}

#[cfg(feature = "math")]
#[wasm_bindgen]
pub fn vector_operations(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> String {
    let v1 = na::Vector3::new(x1, y1, z1);
    let v2 = na::Vector3::new(x2, y2, z2);
    
    let sum = v1 + v2;
    let diff = v1 - v2;
    let dot_product = v1.dot(&v2);
    let cross_product = v1.cross(&v2);
    
    let norm_v1 = v1.norm();
    let norm_v2 = v2.norm();
    
    let cos_angle = dot_product / (norm_v1 * norm_v2);
    let angle = cos_angle.acos().to_degrees();
    
    let projection = if norm_v2 > 1e-10 {
        (dot_product / norm_v2.powi(2)) * v2
    } else {
        na::Vector3::zeros()
    };
    
    let orthogonal = dot_product.abs() < 1e-10;
    let parallel = cross_product.norm() < 1e-10;
    
    format!(
        "Vector Operations\n\nVector 1: [{:.2}, {:.2}, {:.2}]\nVector 2: [{:.2}, {:.2}, {:.2}]\n\nSum: [{:.3}, {:.3}, {:.3}]\nDifference (v1-v2): [{:.3}, {:.3}, {:.3}]\n\nDot Product: {:.4}\nCross Product: [{:.3}, {:.3}, {:.3}]\n\nMagnitudes: |v1| = {:.4}, |v2| = {:.4}\nAngle between vectors: {:.2}°\n\nProjection of v1 onto v2: [{:.3}, {:.3}, {:.3}]\n\nOrthogonal: {}\nParallel: {}",
        x1, y1, z1, x2, y2, z2,
        sum.x, sum.y, sum.z,
        diff.x, diff.y, diff.z,
        dot_product,
        cross_product.x, cross_product.y, cross_product.z,
        norm_v1, norm_v2, angle,
        projection.x, projection.y, projection.z,
        orthogonal, parallel
    )
}

#[cfg(feature = "math")]
#[wasm_bindgen]
pub fn solve_linear_system(a11: f64, a12: f64, b1: f64, a21: f64, a22: f64, b2: f64) -> String {
    let a = na::Matrix2::new(a11, a12, a21, a22);
    let b = na::Vector2::new(b1, b2);
    
    let det = a.determinant();
    
    let solution = if det.abs() > 1e-10 {
        match a.lu().solve(&b) {
            Some(x) => format!("Solution: x1 = {:.6}, x2 = {:.6}\n\nVerification:\n  {:.3}*{:.6} + {:.3}*{:.6} = {:.6} (expected: {:.3})\n  {:.3}*{:.6} + {:.3}*{:.6} = {:.6} (expected: {:.3})",
                x[0], x[1],
                a11, x[0], a12, x[1], a11*x[0] + a12*x[1], b1,
                a21, x[0], a22, x[1], a21*x[0] + a22*x[1], b2),
            None => "No unique solution (system is singular)".to_string(),
        }
    } else {
        let rank_a = if det.abs() < 1e-10 { 1 } else { 2 };
        
        if rank_a == 1 {
            let ratio1 = if a11.abs() > 1e-10 { b1 / a11 } else { 0.0 };
            let ratio2 = if a21.abs() > 1e-10 { b2 / a21 } else { 0.0 };
            
            if (ratio1 - ratio2).abs() < 1e-10 {
                "Infinite solutions (dependent system)".to_string()
            } else {
                "No solution (inconsistent system)".to_string()
            }
        } else {
            "System analysis failed".to_string()
        }
    };
    
    format!(
        "Linear System Solver\n\nSystem of equations:\n  {:.3}x₁ + {:.3}x₂ = {:.3}\n  {:.3}x₁ + {:.3}x₂ = {:.3}\n\nMatrix determinant: {:.6}\n\n{}",
        a11, a12, b1, a21, a22, b2, det, solution
    )
}