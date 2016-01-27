# Пример использования литералов UTF-16 в стабильном Rust 1.6
Код взят с комментария https://gitter.im/ruRust/general?at=56a7eed4eaf741c118d50f88

Внимание:
1) .gitignore содержит строки *~ чтобы избежать добавление временных файлов в репозиторий.
2) build.rs создаст файл _main.rs~ куда и поместит весь обработанный код.

В результате в _main.rs~ получится что-то:
```rust
#![no_std]
#![feature(libc, lang_items, start, plugin)]
extern crate libc;
static HELLO_WORLD: [u8; 24] =
    [104u8, 0u8, 101u8, 0u8, 108u8, 0u8, 108u8, 0u8, 111u8, 0u8, 32u8, 0u8,
     119u8, 0u8, 111u8, 0u8, 114u8, 0u8, 108u8, 0u8, 100u8, 0u8, 33u8, 0u8];
#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize { 0 }
#[lang = "eh_personality"]
extern "C" fn eh_personality() { }
#[lang = "panic_fmt"]
fn panic_fmt() -> !  { loop  { } }
```
