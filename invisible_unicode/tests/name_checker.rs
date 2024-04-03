use invisible_unicode;
use rand::Rng;
use invisible_unicode::{INVISIBILITIES, InvisibleChecker};

const TEST_NAME: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_";

#[test]
fn find_invisible_char() {
    let checker = InvisibleChecker::builder().build();

    let normal_name = TEST_NAME;
    assert_eq!(false, checker.has_invisible_char(&normal_name), "contains invisible char");

    let invalid_name = format!("{}{}", INVISIBILITIES[0], normal_name);
    assert_eq!(true, checker.has_invisible_char(&invalid_name), "not contains invisible char");
}

#[test]
fn ignore_invisible_char() {
    let ignore = INVISIBILITIES[0];

    let checker = InvisibleChecker::builder().remove(ignore).build();

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..TEST_NAME.len());
    // Insert exception characters at arbitrary positions
    let mut normal_name = TEST_NAME.to_string();
    normal_name.insert(index, ignore);

    assert_eq!(false, checker.has_invisible_char(&normal_name), "contains invisible char");

    let invalid_name = format!("{}{}", ignore, normal_name);
    assert_eq!(false, checker.has_invisible_char(&invalid_name), "not contains invisible char");
}

#[test]
fn ignores_invisible_char() {
    let ignores = vec![INVISIBILITIES[0], INVISIBILITIES[1]];
    let checker = InvisibleChecker::builder().removes(&ignores).build();

    let mut normal_name = TEST_NAME.to_string();
    let mut rng = rand::thread_rng();
    for ignore in &ignores {
        let index = rng.gen_range(0..TEST_NAME.len());
        // Insert exception characters at arbitrary positions
        normal_name.insert(index, *ignore);
    }

    assert_eq!(false, checker.has_invisible_char(&normal_name), "contains invisible char");

    let invalid_name = format!("{}{}{}", ignores[0], ignores[1], normal_name);
    assert_eq!(false, checker.has_invisible_char(&invalid_name), "not contains invisible char");
}