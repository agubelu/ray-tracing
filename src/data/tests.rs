#[cfg(test)]
mod container_testing {
    use crate::data::{Vec3, VecElem};
    use crate::vec3;

    const ERROR_MARGIN: VecElem = VecElem::EPSILON;

    fn elems_eq(f1: VecElem, f2: VecElem) -> bool {
        (f1 - f2) < ERROR_MARGIN
    }

    #[test]
    fn check_macro() {
        assert_eq!(vec3![1.0, 2.0, 3.0], Vec3::from(1.0, 2.0, 3.0));
    }

    #[test]
    fn check_coords() {
        let vec = vec3![1.0, 2.0, 3.0];
        assert!(elems_eq(vec.x(), 1.0));
        assert!(elems_eq(vec.y(), 2.0));
        assert!(elems_eq(vec.z(), 3.0));
    }

    #[test]
    fn check_indices() {
        let vec = vec3![1.0, 2.0, 3.0];
        assert!(elems_eq(vec[0], 1.0));
        assert!(elems_eq(vec[1], 2.0));
        assert!(elems_eq(vec[2], 3.0));
    }

    #[test]
    fn check_mut() {
        let mut vec = vec3![];
        vec[0] = 1.0;
        vec[1] = 2.5;
        vec[2] = 7.0;
        assert_eq!(vec, vec3![1.0, 2.5, 7.0]);
    }

    #[test]
    fn check_mag() {
        let vec = vec3![1.0, 2.0, 3.0];
        assert!(elems_eq(vec.mag(), 14_f32.sqrt()));
    }

    #[test]
    fn check_add() {
        let mut vec1 = vec3![1.0, 2.0, 3.0];
        let vec2 = vec3![4.0, 5.0, 6.0];
        let vec3 = vec3![0.5, 1.0, 1.5];
        assert_eq!(vec1 + vec2, vec3![5.0, 7.0, 9.0]);
        vec1 += &vec3;
        assert_eq!(vec1, vec3![1.5, 3.0, 4.5]);
    }

    #[test]
    fn check_minus() {
        let mut vec1 = vec3![1.0, 2.0, 3.0];
        let vec2 = vec3![4.0, 4.0, 4.0];
        let vec3 = vec3![0.5, 1.0, 1.5];
        assert_eq!(vec2 - vec1, vec3![3.0, 2.0, 1.0]);
        vec1 -= &vec3;
        assert_eq!(vec1, vec3![0.5, 1.0, 1.5]);
    }

    #[test]
    fn check_mult() {
        let mut vec1 = vec3![1.0, 2.0, 3.0];
        assert_eq!(vec1 * 2.0, vec3![2.0, 4.0, 6.0]);
        vec1 *= 3.0;
        assert_eq!(vec1, vec3![3.0, 6.0, 9.0]);
    }

    #[test]
    fn check_div() {
        let mut vec1 = vec3![10.0, 20.0, 30.0];
        assert_eq!(vec1 / 2.0, vec3![5.0, 10.0, 15.0]);
        vec1 /= 10.0;
        assert_eq!(vec1, vec3![1.0, 2.0, 3.0]);
    }

    #[test]
    fn check_unit() {
        let vec1 = vec3![5.0, 6.5, 7.0];
        let unit = vec1.unit();
        let f = vec1[0] / unit[0];
        assert!(elems_eq(vec1[1] / f, unit[1]));
        assert!(elems_eq(vec1[2] / f, unit[2]));
        assert!(elems_eq(unit.mag(), 1.0));
    }

    #[test]
    fn check_dot_prod() {
        let vec1 = vec3![1.0, 2.0, 3.0];
        let vec2 = vec3![4.0, 5.0, 6.0];
        assert!(elems_eq(vec1 * vec2, 32.0));
        assert!(elems_eq(vec1.dot_prod(&vec2), 32.0));
    }

    #[test]
    fn check_cross_prod() {
        let vec1 = vec3![1.0, 2.0, 3.0];
        let vec2 = vec3![4.0, 5.0, 7.0];
        assert_eq!(vec1 % vec2, vec3![-1.0, 5.0, -3.0]);
        assert_eq!(vec1.cross_prod(&vec2), vec3![-1.0, 5.0, -3.0]);
    }
}