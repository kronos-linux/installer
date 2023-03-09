#[cfg(test)]
use crate::*;

#[test]
fn creation() {
    let _ = Error::Generic("A generic error from a test".into());
}
