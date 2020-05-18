// Folder will be used for attempting to solve Equations with Custom Units.
// More Examples: http://www.sosmath.com/algebra/solve/solve0/solve0.html

// Algebraic Simplification Examples (Solve for X):
// x − 2 = 4
//   - x = 4 + 2
//   - x = 6
// 3x − 6 = 9
//   - 3x = 9 + 6
//   - x = 15 / 3
//   - x = 5
// 5x - 6 = 3x - 8
//   - 5x = 3x - 2
//   - 2x = -2
//   - x = -1


// (x − 3)(x − 2) = 2
//   + Use Foil Method: (a + b)(c + d) = ac + ad + bc + bd
//   - xx + x(-2) + (-3)x + (-3)(-2) = 2
//   - xx - 2x - 3x + 3 * 2 = 2
//   - x^2 - 5x + 6 = 2
//   - x^2 - 5x + 4 = 0
//   + Use Quadratic Formula: ax^2 + bx + c = 0
//   + a = 1, b = -5, c = 4



// x = (-b +- sqrt(b^2 - 4ac)) / (2a)
pub fn solve_quadratic_formula(a: f64, b: f64, c: f64) -> (f64, f64) {
	(
		(-1.0 * b + (b.powf(2.0) - (4.0 * a * c)).sqrt()) / (2.0 * a),
		(-1.0 * b - (b.powf(2.0) - (4.0 * a * c)).sqrt()) / (2.0 * a)
	)
}