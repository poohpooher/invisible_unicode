# invisible_unicode
- 문자열에서 보이지 않는 문자(unicode) 검사

## generate
https://invisible-characters.com/ 로 부터 Invisible Char 목록을 얻어와 invisibilities.rs 를 제네레이트

### sample
``` Rust
let name = "abcd ";
let checker = InvisibleChecker::builder().build();
if checker.has_invisible_char(&name) {
    println!("contains invisible char");
}

```