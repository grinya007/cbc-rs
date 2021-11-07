extern crate cbc;
use cbc::*;

#[test]
fn knapsack() {
    let mut m = Model::new();
    assert!(Model::version().len() > 4);
    m.load_problem(
        5,
        1,
        &vec![0, 1, 2, 3, 4, 5],
        &vec![0, 0, 0, 0, 0],
        &vec![2., 8., 4., 2., 5.],
        Some(&vec![0., 0., 0., 0., 0.]),
        Some(&vec![1., 1., 1., 1., 1.]),
        Some(&vec![5., 3., 2., 7., 4.]),
        Some(&vec![-std::f64::INFINITY]),
        Some(&vec![10.]),
    );
    assert_eq!(5, m.num_cols());
    assert_eq!(1, m.num_rows());
    m.set_obj_sense(Sense::Maximize);
    assert_eq!(Sense::Maximize, m.obj_sense());
    for i in 0..5 {
        m.set_integer(i);
        assert!(m.is_integer(i));
    }
    m.set_initial_solution(&vec![1., 1., 0., 0., 0.]);
    m.solve();
    assert_eq!(Status::Finished, m.status());
    assert!(m.is_proven_optimal());
    assert!(!m.is_abandoned());
    assert!(!m.is_proven_infeasible());
    assert!(!m.is_continuous_unbounded());
    assert!(!m.is_node_limit_reached());
    assert!(!m.is_seconds_limit_reached());
    assert!(!m.is_solution_limit_reached());
    assert!((m.obj_value() - 16.).abs() < 1e-6);
    assert!((m.best_possible_value() - 16.).abs() < 1e-6);
    let sol = m.col_solution();
    assert!((sol[0] - 1.).abs() < 1e-6);
    assert!((sol[1] - 0.).abs() < 1e-6);
    assert!((sol[2] - 0.).abs() < 1e-6);
    assert!((sol[3] - 1.).abs() < 1e-6);
    assert!((sol[4] - 1.).abs() < 1e-6);
}

#[test]
fn big_row() {
    let mut m = Model::new();
    let numcols = 0;
    let numrows = 1_000;
    let start = [1];
    let value = [0.];
    m.load_problem(
        numcols, numrows, &start, &start, &value, None, None, None, None, None,
    );
    m.set_initial_solution(&[]);
    assert_eq!(&value, &[0.])
}

#[test]
fn sos_multiple_constraints() {
    let mut m = Model::new();
    // Minimize x + 5y + z with -1 <= x <= 1 and -1 <= y <= 1 and -1 <= z <= 1
    m.load_problem(
        3,
        0,
        &vec![0, 0, 0, 0],
        &vec![],
        &vec![],
        Some(&vec![-1., -1., -1.]),
        Some(&vec![1., 1., 1.]),
        Some(&vec![1., 5., 1.]),
        None,
        None,
    );
    // Add a constraint that either x or y must be 0
    m.add_sos(
        &[
            0, 2, // The first constraint is on columns col_indices[0..2]
            4, // The second is on columns col_indices[2..4]
        ],
        &[
            0, 1, // The first constraint is that either x or y must be 0
            1, 2, // The second constraint is that either y or z must be 0
        ],
        &[1., 5., 5., 1.],
        SOSConstraintType::Type1,
    );
    m.set_integer(0);
    m.set_integer(1);
    m.set_integer(2);

    m.solve();
    // The solution is x = -1 and y = 0
    assert_eq!(&[0., -1., 0.], m.col_solution());
}
