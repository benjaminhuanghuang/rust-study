#[derive(Debug)]
struct ErrorA;


impl std::fmt::Display for ErrorA {
    fn fmt(&self, &mut std::fmt::Formatter<'_>) -> std:: fmt:: Result {
        write!(f, "ErrorA")
    }
}