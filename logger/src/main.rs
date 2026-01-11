
trait Logger {
    fn log(&self, verbosity: u8, message: &str);
}

struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

struct VerbosityFilter {
    max_verbosity: u8,
    inner: Box<dyn Logger>
}

impl VerbosityFilter {
    fn new(max_verbosity: u8) -> Self {
        VerbosityFilter{max_verbosity, inner: Box::new(StderrLogger)}
    }
}

impl Logger for VerbosityFilter {
    fn log(&self, verbosity: u8, message: &str) {
        if  verbosity <= self.max_verbosity {
            self.inner.log(verbosity, message);
        }
    }
}

fn main() {
    let logger = VerbosityFilter::new(3);
    logger.log(5, "FYI");
    logger.log(2, "Uhoh");
}
