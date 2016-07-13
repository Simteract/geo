use num::Float;
use types::{LineString, Polygon};
use algorithm::contains::Contains;

/// Checks if the geometry A intersects the geometry B.

pub trait Intersects<Rhs = Self> {
    /// Checks if the geometry A intersects the geometry B.
    ///
    /// ```
    /// use geo::{Coordinate, Point, LineString};
    /// use geo::algorithm::intersects::Intersects;
    ///
    /// let p = |x, y| Point(Coordinate { x: x, y: y });
    /// let linestring = LineString(vec![p(3., 2.), p(7., 6.)]);
    ///
    /// assert!(linestring.intersects(&LineString(vec![p(3., 4.), p(8., 4.)])));
    /// assert!(!linestring.intersects(&LineString(vec![p(9., 2.), p(11., 5.)])));
    ///
    /// ```
    ///
    fn intersects(&self, rhs: &Rhs) -> bool;
}

impl<T> Intersects<LineString<T>> for LineString<T>
    where T: Float
{
    // See: https://github.com/brandonxiang/geojson-python-utils/blob/33b4c00c6cf27921fb296052d0c0341bd6ca1af2/geojson_utils.py
    fn intersects(&self, linestring: &LineString<T>) -> bool {
        let vect0 = &self.0;
        let vect1 = &linestring.0;
        if vect0.is_empty() || vect1.is_empty() {
            return false;
        }
        for (a1, a2) in vect0.iter().zip(vect0[1..].iter()) {
            for (b1, b2) in vect1.iter().zip(vect1[1..].iter()) {
                let u_b = (b2.y() - b1.y()) * (a2.x() - a1.x()) -
                          (b2.x() - b1.x()) * (a2.y() - a1.y());
                if u_b == T::zero() {
                    continue;
                }
                let ua_t = (b2.x() - b1.x()) * (a1.y() - b1.y()) -
                           (b2.y() - b1.y()) * (a1.x() - b1.x());
                let ub_t = (a2.x() - a1.x()) * (a1.y() - b1.y()) -
                           (a2.y() - a1.y()) * (a1.x() - b1.x());
                let u_a = ua_t / u_b;
                let u_b = ub_t / u_b;
                if (T::zero() <= u_a) && (u_a <= T::one()) && (T::zero() <= u_b) && (u_b <= T::one()) {
                    return true;
                }
            }
        }
        false
    }
}

impl<T> Intersects<LineString<T>> for Polygon<T>
    where T: Float
{
    fn intersects(&self, linestring: &LineString<T>) -> bool {
        // line intersects inner or outer polygon edge
        if self.0.intersects(linestring) || self.1.iter().any(|inner| inner.intersects(linestring)) {
            return true;
        } else {
            // or if it's contained in the polygon
            return linestring.0.iter().any(|point| self.contains(point))
        }
    }
}
#[cfg(test)]
mod test {
    use types::{Coordinate, Point, LineString, Polygon};
    use algorithm::intersects::Intersects;
    /// Tests: intersection LineString and LineString
    #[test]
    fn empty_linestring1_test() {
        let p = |x, y| Point(Coordinate { x: x, y: y });
        let linestring = LineString(vec![p(3., 2.), p(7., 6.)]);
        assert!(!LineString(Vec::new()).intersects(&linestring));
    }
    #[test]
    fn empty_linestring2_test() {
        let p = |x, y| Point(Coordinate { x: x, y: y });
        let linestring = LineString(vec![p(3., 2.), p(7., 6.)]);
        assert!(!linestring.intersects(&LineString(Vec::new())));
    }
    #[test]
    fn empty_all_linestring_test() {
        assert!(!LineString::<f64>(Vec::new()).intersects(&LineString(Vec::new())));
    }
    #[test]
    fn intersect_linestring_test() {
        let p = |x, y| Point(Coordinate { x: x, y: y });
        let linestring = LineString(vec![p(3., 2.), p(7., 6.)]);
        assert!(linestring.intersects(&LineString(vec![p(3., 4.), p(8., 4.)])));
    }
    #[test]
    fn parallel_linestrings_test() {
        let p = |x, y| Point(Coordinate { x: x, y: y });
        let linestring = LineString(vec![p(3., 2.), p(7., 6.)]);
        assert!(!linestring.intersects(&LineString(vec![p(3., 1.), p(7., 5.)])));
    }
    /// Tests: intersection LineString and Polygon
    #[test]
    fn linestring_in_polygon_test() {
        let p = |x, y| Point(Coordinate { x: x, y: y });
        let linestring = LineString(vec![p(0., 0.), p(5., 0.), p(5., 6.), p(0., 6.), p(0., 0.)]);
        let poly = Polygon(linestring, Vec::new());
        assert!(poly.intersects(&LineString(vec![p(2., 2.), p(3., 3.)])));
    }
    #[test]
    fn linestring_on_boundary_polygon_test() {
        let p = |x, y| Point(Coordinate { x: x, y: y });
        let poly = Polygon(LineString(vec![p(0., 0.), p(5., 0.), p(5., 6.), p(0., 6.), p(0., 0.)]),
                           Vec::new());
        assert!(poly.intersects(&LineString(vec![p(0., 0.), p(5., 0.)])));
        assert!(poly.intersects(&LineString(vec![p(5., 0.), p(5., 6.)])));
        assert!(poly.intersects(&LineString(vec![p(5., 6.), p(0., 6.)])));
        assert!(poly.intersects(&LineString(vec![p(0., 6.), p(0., 0.)])));
    }
    #[test]
    fn intersect_linestring_polygon_test() {
        let p = |x, y| Point(Coordinate { x: x, y: y });
        let poly = Polygon(LineString(vec![p(0., 0.), p(5., 0.), p(5., 6.), p(0., 6.), p(0., 0.)]),
                           Vec::new());
        assert!(poly.intersects(&LineString(vec![p(2., 2.), p(6., 6.)])));
    }
    #[test]
    fn linestring_outside_polygon_test() {
        let p = |x, y| Point(Coordinate { x: x, y: y });
        let poly = Polygon(LineString(vec![p(0., 0.), p(5., 0.), p(5., 6.), p(0., 6.), p(0., 0.)]),
                           Vec::new());
        assert!(!poly.intersects(&LineString(vec![p(7., 2.), p(9., 4.)])));
    }
    #[test]
    fn linestring_in_inner_polygon_test() {
        let p = |x, y| Point(Coordinate { x: x, y: y });
        let v = vec![LineString(vec![p(1., 1.), p(4., 1.), p(4., 4.), p(1., 4.), p(1., 1.)])];
        let poly = Polygon(LineString(vec![p(0., 0.), p(5., 0.), p(5., 6.), p(0., 6.), p(0., 0.)]),
                           v);
        assert!(!poly.intersects(&LineString(vec![p(2., 2.), p(3., 3.)])));
        assert!(poly.intersects(&LineString(vec![p(2., 2.), p(4., 4.)])));
    }
    #[test]
    fn linestring_traverse_polygon_test() {
        let p = |x, y| Point(Coordinate { x: x, y: y });
        let v = vec![LineString(vec![p(1., 1.), p(4., 1.), p(4., 4.), p(1., 4.), p(1., 1.)])];
        let poly = Polygon(LineString(vec![p(0., 0.), p(5., 0.), p(5., 6.), p(0., 6.), p(0., 0.)]),
                           v);
        assert!(poly.intersects(&LineString(vec![p(2., 0.5), p(2., 5.)])));
    }
    #[test]
    fn linestring_in_inner_with_2_inner_polygon_test() {
        //                                        (8,9)
        //     (2,8)                                |                                      (14,8)
        //      ------------------------------------|------------------------------------------
        //      |                                   |                                         |
        //      |     (4,7)            (6,7)        |                                         |
        //      |       ------------------          |                    (11,7)               |
        //      |                                   |                       |                 |
        //      |     (4,6)                (7,6)    |     (9,6)             |     (12,6)      |
        //      |       ----------------------      |       ----------------|---------        |
        //      |       |                    |      |       |               |        |        |
        //      |       |       (6,5)        |      |       |               |        |        |
        //      |       |        /           |      |       |               |        |        |
        //      |       |       /            |      |       |               |        |        |
        //      |       |     (5,4)          |      |       |               |        |        |
        //      |       |                    |      |       |               |        |        |
        //      |       ----------------------      |       ----------------|---------        |
        //      |     (4,3)                (7,3)    |     (9,3)             |     (12,3)      |
        //      |                                   |                    (11,2.5)             |
        //      |                                   |                                         |
        //      ------------------------------------|------------------------------------------
        //    (2,2)                                 |                                      (14,2)
        //                                        (8,1)
        //
        let p = |x, y| Point(Coordinate { x: x, y: y });

        let v = vec![LineString(vec![p(4., 3.), p(7., 3.), p(7., 6.), p(4., 6.), p(4., 3.)]),
                     LineString(vec![p(9., 3.), p(12., 3.), p(12., 6.), p(9., 6.), p(9., 3.)])];
        let poly = Polygon(LineString(vec![p(2., 2.), p(14., 2.), p(14., 8.), p(2., 8.),
                                           p(2., 2.)]),
                           v);
        assert!(!poly.intersects(&LineString(vec![p(5., 4.), p(6., 5.)])));
        assert!(poly.intersects(&LineString(vec![p(11., 2.5), p(11., 7.)])));
        assert!(poly.intersects(&LineString(vec![p(4., 7.), p(6., 7.)])));
        assert!(poly.intersects(&LineString(vec![p(8., 1.), p(8., 9.)])));
    }
}
