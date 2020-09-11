use super::*;

// 3D Affine transformation.
//	y = A x + b
pub struct Transform {
	pub a: Matrix<f64>,
	pub b: Vec3<f64>,
}

impl Transform {
	pub fn new(a: Matrix<f64>, b: Vector) -> Self {
		Self { a, b }
	}

	// Applies the affine transformation to a point.
	pub fn transform_point(&self, p: Point) -> Point {
		self.a * p + self.b
	}

	// Applies affine transformation to a vector (direction).
	// Directions are invariant against the the translation part of the transform.
	// Use `transform_point` to transform a point, which does undergo translation.
	pub fn transform_vector(&self, x: Vector) -> Vector {
		self.a * x
	}

	// The inverse affine transformation.
	pub fn inverse(&self) -> Self {
		// if
		// 	y = Ax + b
		// then
		// 	x = (A^-1)y + (-A^-1)b
		// so the coefficients of the inverse transform are (A^-1), (-A^-1)b
		let ainv = self.a.recip();
		Self::new(ainv, -ainv * self.b)
	}

	/// A composite transform that applies `rhs` first, followed by `self`.
	pub fn after(&self, s: &Self) -> Self {
		s.before(self)
	}

	/// A composite transform that applies `self` first, followed by `rhs`.
	pub fn before(&self, rhs: &Self) -> Self {
		// y = TAx+Tb
		// z = SAy+Sb
		//   = SA(TAx+Tb)+Sb
		//   = (SA*TA)x + (SA*Tb+Sb)
		Self::new(rhs.a * self.a, rhs.a * self.b + rhs.b)
	}

	/// The identity transform returns its inputs unchanged.
	pub fn unit() -> Self {
		Self::new(Matrix::unit(), Vector::default())
	}

	/// A transform that scales by factor `s`,
	/// with `origin` as the fixed point.
	pub fn scale(origin: Point, s: f64) -> Self {
		Self::with_origin(origin, Matrix::unit() * s)
	}

	pub fn with_origin(origin: Point, a: Matrix<f64>) -> Self {
		// (m * (x-o)) + o
		// m*x - m*o + o
		// m*x + (o - m*o)
		Self::new(a, origin - a * origin)
	}

	// Translate returns a Transform that translates by delta.
	// Translation affects points (TransformPoint), but not directions (TransformDir).
	pub fn translate(delta: Vector) -> Self {
		Self::new(Matrix::unit(), delta)
	}

	//// Rotate returns a Transform that rotates around an arbitrary axis,
	//// with origin as the fixed point.
	//// The rotation is counterclockwise in a right-handed space.
	pub fn rotate(origin: Point, axis: Vector, radians: f64) -> Self {
		Self::with_origin(origin, Self::rotation_matrix(axis, radians))
	}

	/// Matrix for rotation around an arbitrary axis.
	/// https://en.wikipedia.org/wiki/Rotation_matrix#Rotation_matrix_from_axis_and_angle
	pub fn rotation_matrix(axis: Vector, radians: f64) -> Matrix<f64> {
		let axis = axis.normalized();
		let (ux, uy, uz) = (axis[0], axis[1], axis[2]);
		let c = f64::cos(radians); //TODO: round
		let s = f64::sin(radians);
		let c1 = 1.0 - c;
		Matrix::from([
			[c + ux * ux * c1, uy * ux * c1 + uz * s, uz * ux * c1 - uy * s],
			[ux * uy * c1 - uz * s, c + uy * uy * c1, uz * uy * c1 + ux * s],
			[ux * uz * c1 + uy * s, uy * uz * c1 - ux * s, c + uz * uz * c1],
		])
	}

	pub fn yaw_pitch(yaw: f64, pitch: f64) -> Matrix<f64> {
		Self::rotation_matrix(Vector::EY, yaw) * Self::rotation_matrix(Vector::EX, pitch)
	}
	//func MapBaseTo(o, x, y, z Vec) *AffineTransform {
	//	return &AffineTransform{
	//		A: Matrix{
	//			x.Sub(o),
	//			y.Sub(o),
	//			z.Sub(o),
	//		},
	//		B: o,
	//	}
	//}
	//
	//
	//// WithOrigin returns a translated version of t so that o is the new origin
	//// (fixed point). E.g.:
	//// 	Rotate(Vec{}, Ez, θ).WithOrigin(Vec{1,2,0})
	//// rotates around [1, 2, 0] rather than [0, 0, 0]
	//func (t *AffineTransform) WithOrigin(o Vec) *AffineTransform {
	//	return ComposeLR(Translate(o.Mul(-1)), t, Translate(o))
	//}
}

/*
	TODO: Transform does not have an Origin(), so not usable as frame.
	Add frame here, or keep in builder?
	y = Ax + b
	o = Ao + b
	o - Ao = b
	o (I-A) = b
	o = (I-A)^-1 b
	frame and transform concats are in different order.
	for frame, linear operations and translations commute (translate camera, rotate, translate again)
	for trasform, not
*/

//// ComposeLR composes affine transformations left-to-right.
//// I.e., the leftmost argument is applied first.
//func ComposeLR(t ...*AffineTransform) *AffineTransform {
//	comp := UnitTransform()
//	for _, t := range t {
//		comp = comp.Before(t)
//	}
//	return comp
//}

//// Yaw rotates the camera counterclockwise by the given angle (in radians),
//// while keeping it horizontal. E.g.:
//// 	camera.Yaw(  0*Deg) // look North
//// 	camera.Yaw(+90*Deg) // look West
//// 	camera.Yaw(-90*Deg) // look East
//// 	camera.Yaw(180*Deg) // look South
////
//// Yaw, Pitch and Roll are not commutative.
//// Use YawPitchRoll to apply them in canonical order.
//func Yaw(angle float64) *AffineTransform {
//	return Rotate(O, Ey, angle)
//}
//
//// Pitch tilts the camera upwards by the given angle (in radians). E.g.:
//// 	camera.Pitch(  0*Deg) // look horizontally
//// 	camera.Pitch(-90*Deg) // look at your feet
//// 	camera.Pitch(+90*Deg) // look at the zenith
////
//// Yaw, Pitch and Roll are not commutative.
//// Use YawPitchRoll to apply them in canonical order.
//func Pitch(angle float64) *AffineTransform {
//	return Rotate(O, Ex, angle)
//}
//
//// Roll rotates the camera counterclockwise around the line of sight. E.g.:
//// 	camera.Roll( 0*Deg) // horizon runs straight
//// 	camera.Roll(45*Deg) // horizon runs diagonally, from top left to bottom right.
////
//// Yaw, Pitch and Roll are not commutative.
//// Use YawPitchRoll to apply them in canonical order.
//func Roll(angle float64) *AffineTransform {
//	return Rotate(O, Ez, angle)
//}
//
//func YawPitchRoll(yaw, pitch, roll float64) *AffineTransform {
//	return Yaw(yaw).After(Pitch(pitch)).After(Roll(roll))
//}
