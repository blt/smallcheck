use Serial;
use std::fmt;
use std::panic;
use tester::Status::{Pass, Fail};

/// The primary `smallcheck` type for setting search depth and running tests.
pub struct SmallCheck<S> where S: Serial {
    depth: usize, 
    serial: S,
}

impl<S> SmallCheck<S> where S: Serial {
    pub fn new() -> SmallCheck<S> {
        SmallCheck {
            depth: 100,
            serial: S::new(),
        }
    }

    pub fn test<A>(&mut self, f: A) -> Result<usize, TestResult> where A: Testable {
        unimplemented!()
    }

    /// Test a property and call `panic!` on failure.
    ///
    /// Because of the way SmallCheck works the `panic!` will contain the
    /// 'smallest' failing case for your input.
    ///
    /// This method plugs into Rust's unit testing infrastructure, unlike
    /// `test`.
    pub fn check<A>(&mut self, f: A) where A: Testable {
        match self.test(f) {
            Ok(total_tests) => println!("(Passed {} tests.)", total_tests),
            Err(result) => panic!(result.failed_msg()),
        }
    }
}

/// `Testable` describes types (e.g., a function) whose values can be
/// tested.
///
/// tbh I don't fully know what this means for `smallcheck` yet. I have vague
/// ideas.
pub trait Testable: Send + 'static {
    fn result<S: Serial>(&self, &mut S) -> TestResult;
}

impl Testable for bool {
    fn result<S: Serial>(&self, _: &mut S) -> TestResult {
        TestResult::from_bool(*self)
    }
}

impl Testable for () {
    fn result<S: Serial>(&self, _: &mut S) -> TestResult {
        TestResult::passed()
    }
}

impl Testable for TestResult {
    fn result<S: Serial>(&self, _: &mut S) -> TestResult { self.clone() }
}

impl<A, E> Testable for Result<A, E>
        where A: Testable, E: fmt::Debug + Send + 'static {
    fn result<S: Serial>(&self, s: &mut S) -> TestResult {
        match *self {
            Ok(ref r) => r.result(s),
            Err(ref err) => TestResult::error(format!("{:?}", err)),
        }
    }
}

macro_rules! testable_fn {
    ($($name: ident),*) => {
        impl<T: Testable, $($name: Serial + fmt::Debug),*> Testable for fn($($name),*) -> T {
            fn result<S: Serial>(&self, s: &mut S) -> TestResult {
                let a: Option<($($name,)*)> = Serial::next(s);
                if let Some(a) = a {
                    let self_ = *self;
                    let ( $($name,)* ) = a.clone();
                    let mut r = safe(move || {self_($($name),*)}).result(s);
                    
                    let ( $($name,)* ) = a.clone();
                    r.arguments = vec![$(format!("{:?}", $name),)*];
                    r
                } else {
                    TestResult::passed()
                }
            }
        }
    }
}

testable_fn!();
testable_fn!(A);
testable_fn!(A, B);
testable_fn!(A, B, C);
testable_fn!(A, B, C, D);
testable_fn!(A, B, C, D, E);
testable_fn!(A, B, C, D, E, F);
testable_fn!(A, B, C, D, E, F, G);
testable_fn!(A, B, C, D, E, F, G, H);
testable_fn!(A, B, C, D, E, F, G, H, I);
testable_fn!(A, B, C, D, E, F, G, H, I, J);
testable_fn!(A, B, C, D, E, F, G, H, I, J, K);
testable_fn!(A, B, C, D, E, F, G, H, I, J, K, L);

fn safe<T, F>(fun: F) -> Result<T, String>
        where F: FnOnce() -> T, F: Send + 'static, T: Send + 'static {
    panic::catch_unwind(panic::AssertUnwindSafe(fun)).map_err(|any_err| {
        // Extract common types of panic payload:
        // panic and assert produce &str or String
        if let Some(&s) = any_err.downcast_ref::<&str>() {
            s.to_owned()
        } else if let Some(s) = any_err.downcast_ref::<String>() {
            s.to_owned()
        } else {
            "UNABLE TO SHOW RESULT OF PANIC.".to_owned()
        }
    })
}

/// Describes the status of a single instance of a test.
///
/// All testable things must be capable of producing a `TestResult`.
#[derive(Clone, Debug)]
pub struct TestResult {
    status: Status,
    arguments: Vec<String>,
    err: Option<String>,
}

/// Whether a test has passed, failed or been discarded.
#[derive(Clone, Debug)]
enum Status { Pass, Fail }

impl TestResult {
    /// Produces a test result that indicates the current test has passed.
    pub fn passed() -> TestResult { TestResult::from_bool(true) }

    /// Produces a test result that indicates the current test has failed.
    pub fn failed() -> TestResult { TestResult::from_bool(false) }

    /// Produces a test result that indicates failure from a runtime error.
    pub fn error<S: Into<String>>(msg: S) -> TestResult {
        let mut r = TestResult::from_bool(false);
        r.err = Some(msg.into());
        r
    }

    /// Converts a `bool` to a `TestResult`. A `true` value indicates that
    /// the test has passed and a `false` value indicates that the test
    /// has failed.
    pub fn from_bool(b: bool) -> TestResult {
        TestResult {
            status: if b { Pass } else { Fail },
            arguments: vec![],
            err: None,
        }
    }

    fn failed_msg(&self) -> String {
        match self.err {
            None => {
                format!("[quickcheck] TEST FAILED. Arguments: ({})",
                        self.arguments.connect(", "))
            }
            Some(ref err) => {
                format!("[quickcheck] TEST FAILED (runtime error). \
                         Arguments: ({})\nError: {}",
                        self.arguments.connect(", "), err)
            }
        }
    }
}
    
