use std::f64::consts::PI;

struct RegularPolygon {
    sides: u32,
    side_length: f64,
}

trait Polygon {
    fn perimeter(&self) -> f64;
    fn area(&self) -> f64;
    fn radius(&self) -> f64; 
    fn apothem(&self) -> f64;
}

impl Polygon for RegularPolygon {
    fn perimeter(&self) -> f64 {
        self.sides as f64 * self.side_length
    }

    fn apothem(&self) -> f64 {
        self.side_length / (2.0 * (PI / self.sides as f64).tan())
    }

    fn radius(&self) -> f64 {
        self.side_length / (2.0 * (PI / self.sides as f64).sin())
    }

    fn area(&self) -> f64 {
        0.5 * self.perimeter() * self.apothem()
    }
}

fn main() {
    let side_counts = [4, 8, 16, 32, 64, 128, 256, 512, 2048, 65536];
    let radii = [1.0, 5.0, 10.0];

    for &radius in radii.iter() {
        println!("\nRadius: {}", radius);
        for &sides in side_counts.iter() {
            let side_length = 2.0 * radius * (PI / sides as f64).sin();
            let poly = RegularPolygon { sides, side_length };

            let poly_area = poly.area();
            let circum_area = PI * radius * radius;
            let in_area = PI * poly.apothem() * poly.apothem();
            let ratio_poly_to_circum = poly_area / circum_area; // Add this

            println!(
                "Sides: {:5} | Poly Area: {:.4} | Circum Circle: {:.4} | In Circle: {:.4} | Ratio (Poly/Circum): {:.4}",
                sides, poly_area, circum_area, in_area, ratio_poly_to_circum
            );
        }
    }
}