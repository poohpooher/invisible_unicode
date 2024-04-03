# invisible_unicode
- 문자열에서 보이지 않는 문자(unicode) 검사

### sample
``` Rust
let name = "abcd ";
let checker = InvisibleChecker::builder().build();
if checker.has_invisible_char(&name) {
    println!("contains invisible char");
}

```