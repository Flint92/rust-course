trait Logger {
    fn log(&self, verbosity: u8, message: &str);
}

struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

struct VerbosityFilter<L> {
    max_verbosity: u8,
    inner: L,
}

impl<L: Logger> VerbosityFilter<L> {
    fn new(max_verbosity: u8, inner: L) -> Self {
        VerbosityFilter {
            max_verbosity,
            inner,
        }
    }
}

impl<L: Logger> Logger for VerbosityFilter<L> {
    fn log(&self, verbosity: u8, message: &str) {
        if verbosity <= self.max_verbosity {
            self.inner.log(verbosity, message);
        }
    }
}

fn main() {
    let logger = VerbosityFilter::new(3, StderrLogger);
    logger.log(5, "FYI");
    logger.log(2, "Uhoh");
}
