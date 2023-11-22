// errors4.rs
//
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a
// hint.



#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// 1 0 0 1 => 9
// 2^3 2^2 2^1 2^0

// -5i8
// 00000101 -neg-> 11111010 -+1-> 11111011

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Hmm... Why is this always returning an Ok value?
        if value <0{
            Err(CreationError::Negative)
        }
        else if value == 0{
            Err(CreationError::Zero)
        }
        else{
            Ok(PositiveNonzeroInteger(value as u64))
        }

    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
