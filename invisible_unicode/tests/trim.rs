use invisible_unicode;
use invisible_unicode::InvisibleChecker;

#[test]
pub fn left_trim() {
    let checker = InvisibleChecker::builder().build();

    // space is invisible character
    let name = " abcd ";
    assert_eq!("abcd ", checker.left_trim(name));

    // invisible unicode in random order
    let name = "  	abcd ";
    assert_eq!("abcd ", checker.left_trim(name));
}

#[test]
pub fn right_trim() {
    let checker = InvisibleChecker::builder().build();

    // space is invisible character
    let name = " abcd ";
    assert_eq!(" abcd", checker.right_trim(name));

    // invisible unicode in random order
    let name = " abcd   	";
    assert_eq!(" abcd", checker.right_trim(name));
}

#[test]
pub fn trim() {
    let checker = InvisibleChecker::builder().build();

    // space is invisible character
    let name = " abcd ";
    assert_eq!("abcd", checker.trim(name));

    // invisible unicode in random order
    let name = "  	abcd   	";
    assert_eq!("abcd", checker.trim(name));
}