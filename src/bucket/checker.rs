use tonic::Status;

pub trait Checker {
    fn check(&self, unchecked: String) -> Result<String, Status>;
}

struct CheckFn<F> {
    checker: F,
}

impl<F> Checker for CheckFn<F>
where
    F: Fn(String) -> Result<String, Status>,
{
    fn check(&self, unchecked: String) -> Result<String, Status> {
        (self.checker)(unchecked)
    }
}

pub fn checker_new<F>(checker: F) -> impl Checker
where
    F: Fn(String) -> Result<String, Status>,
{
    CheckFn { checker }
}

pub fn uuid_check_simple(unchecked: String) -> Result<String, Status> {
    u128::from_str_radix(unchecked.as_str(), 16)
        .map(|_: u128| unchecked)
        .map_err(|e| Status::invalid_argument(format!("Invalid uuid: {e}")))
}

pub fn uuid_checker_new<F>(checker: F, prefix: String) -> impl Checker
where
    F: Fn(String) -> Result<String, Status>,
{
    checker_new(move |unchecked: String| {
        let checked: String = checker(unchecked)?;
        Ok(format!("{prefix}{checked}"))
    })
}

pub fn uuid_checker_new_simple(prefix: String) -> impl Checker {
    uuid_checker_new(uuid_check_simple, prefix)
}

pub fn nop_checker() -> impl Checker {
    checker_new(Ok)
}

#[cfg(test)]
mod test_checker {

    mod uuid {
        mod uuid_check_simple {
            use crate::bucket::checker;

            #[test]
            fn test_empty() {
                let r: Result<_, _> = checker::uuid_check_simple("".into());
                assert!(r.is_err());
            }

            #[test]
            fn test_invalid() {
                let r: Result<_, _> = checker::uuid_check_simple("zz".into());
                assert!(r.is_err());
            }

            #[test]
            fn test_long() {
                let r: Result<_, _> =
                    checker::uuid_check_simple("cafef00d-dead-beaf-face-864299792458".into());
                assert!(r.is_err());
            }

            #[test]
            fn test_short() {
                let i: &str = "cafef00ddeadbeafface864299792458";
                let r: Result<_, _> = checker::uuid_check_simple(i.into());
                assert!(r.is_ok());
                let s: String = r.unwrap(); // unwrap inside test
                assert_eq!(s.as_str(), i);
            }
        }
    }
}
