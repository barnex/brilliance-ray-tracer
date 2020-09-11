use crate::*;

// Parametric constructs a mesh approximating the parametric surface defined by
// 	x,y,z = f(u,v)
// where u, v are varied from 0 to 1 (inclusive).
pub fn parametric<F>(mat: DynMaterial, (num_u, num_v): (u32, u32), f: F) -> DynObj
where
	F: Fn(f64, f64) -> Point,
{
	// E.g.: numU: 3, numV: 2 => vertices: 2*3 = 6, faces: 2*((3-1)*(2-1)) = 4
	//  [0,0]***********[1,0]*************[2,0]
	//    *  *            *   *             *
	//    *      *        *       *         *
	//    *         *     *          *      *
	//    *            *  *              *  *
	//  [0,1]***********[1,1]*************[2,1]
	// TODO: split along diagonal which minimizes error.

	let mut faces: Vec<Face> = Vec::new();

	let mut push = |a: Vertex, b: Vertex, c: Vertex| {
		faces.push(Face::new(a, b, c));
	};

	let d = 1e-4;
	let max_u = (num_u + 1) as f64;
	let max_v = (num_v + 1) as f64;
	for iu in 0..(num_u + 1) {
		for iv in 0..(num_v + 1) {
			let vertex = |iu, iv| {
				let u = (iu as f64) / max_u;
				let v = (iv as f64) / max_v;
				let pos = f(u, v);
				let shading_normal = (f(u + d, v) - f(u - d, v)).cross(f(u, v + d) - f(u, v - d)).normalized();
				Vertex {
					pos: pos.into(),
					attr: Attr::new(shading_normal.into(), TexCoords::from((u, v))),
				}
			};

			push(vertex(iu, iv), vertex(iu + 1, iv), vertex(iu + 1, iv + 1));
			push(vertex(iu, iv), vertex(iu + 1, iv + 1), vertex(iu, iv + 1));
		}
	}

	DynObj::new(QTree::new(faces).paint(mat))
}
