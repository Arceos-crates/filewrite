#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[macro_use]
#[cfg(feature = "axstd")]
extern crate axstd as std;

use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let path = "test1.txt";
    let mut sum: u128 = 0;
    let buffer = "1234567890fortest\0".as_bytes();

    for i in 0..10 {
        let start = Instant::now();
        for _ in 0..50 {
            let mut f = File::create(path).unwrap();
            f.write_all(buffer).unwrap();
            drop(f);
            // 不需要关闭文件
        }
        let duration = start.elapsed();
        sum = sum + duration.as_nanos();
        println!("i: {} duration: {}", i, duration.as_nanos());
    }

    // 以纳秒单位输出消耗的时间
    println!("Write: Time: {}", sum / 10);
}
