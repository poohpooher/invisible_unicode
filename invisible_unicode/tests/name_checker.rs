use invisible_unicode;
use rand::Rng;
use invisible_unicode::{INVISIBILITIES, InvisibleChecker};

const TEST_NAME: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_";

#[test]
fn find_invisible_char() {
    let checker = InvisibleChecker::builder().build();

    let normal_name = TEST_NAME;

    // invisible 문자가 없으면 정상
    assert_eq!(false, checker.has_invisible_char(&normal_name), "contains invisible char");


    let invalid_name = format!("{}{}", INVISIBILITIES[0], normal_name);
    // invisible 문자가 있어야 정상
    assert_eq!(true, checker.has_invisible_char(&invalid_name), "not contains invisible char");
}

#[test]
fn ignore_invisible_char() {
    let ignore = INVISIBILITIES[0];

    let checker = InvisibleChecker::builder().remove(ignore).build();

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..TEST_NAME.len());
    // 임의의 위치에 예외 문자 삽입
    let mut normal_name = TEST_NAME.to_string();
    normal_name.insert(index, ignore);

    // invisible 문자가 없으면 정상
    assert_eq!(false, checker.has_invisible_char(&normal_name), "contains invisible char");

    let invalid_name = format!("{}{}", ignore, normal_name);
    // 예외 처리한 항목은 검사에서 제외
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
        // 임의의 위치에 예외 문자 삽입
        normal_name.insert(index, *ignore);
    }

    // invisible 문자가 없으면 정상
    assert_eq!(false, checker.has_invisible_char(&normal_name), "contains invisible char");

    let invalid_name = format!("{}{}{}", ignores[0], ignores[1], normal_name);
    // 예외 처리한 항목은 검사에서 제외
    assert_eq!(false, checker.has_invisible_char(&invalid_name), "not contains invisible char");
}