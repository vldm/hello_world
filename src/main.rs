#![no_std]
#![feature(libc, lang_items, start, plugin)]

extern crate libc;

static HELLO_WORLD : [u8; 24] = utf16!("hello world!");

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    0
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
