#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{fork, mail_read, mail_write, sleep, wait, exit};

#[no_mangle]
fn main() -> i32 {
    const buf_len: usize = 256;
    const mail_num: usize = 256;
    let pid = fork();
    if pid == 0 {
        println!("I am child");
        let mut buffer = [0u8; buf_len];
        assert_eq!(mail_read(&mut buffer), -1);
        println!("child read 1 mail fail");
        println!("child sleep 2s");
        sleep(2000 as usize);
        for i in 0..16 {
            let mut buffer = [0u8; buf_len];
            assert_eq!(mail_read(&mut buffer), buf_len as isize);
            assert_eq!(buffer, [i as u8; buf_len]);
        }
        println!("child read 16 mails succeed");
        assert_eq!(mail_read(&mut buffer), -1);
        println!("child read 1 mail fail");
        println!("child sleep 1s");
        sleep(1000 as usize);
        assert_eq!(mail_read(&mut buffer), buf_len as isize);
        assert_eq!(buffer, [16 as u8; buf_len]);
        println!("child read 1 mail succeed");
        println!("child exit");
        exit(0);
    }
    println!("I am father");
    println!("father sleep 1s");
    sleep(1000 as usize);
    for i in 0..16 {
        let buffer = [i as u8; buf_len];
        assert_eq!(mail_write(pid as usize, &buffer), buf_len as isize);
    }
    println!("father wirte 16 mails succeed");
    let mut buffer = [16 as u8; buf_len];
    assert_eq!(mail_write(pid as usize, &buffer), -1);
    println!("father wirte 1 mail fail");
    println!("father sleep 2s");
    sleep(2000 as usize);
    assert_eq!(mail_write(pid as usize, &buffer), buf_len as isize);
    println!("father wirte 1 mail succeed");

    let mut xstate: i32 = -100;
    assert!(wait(&mut xstate) > 0);
    assert_eq!(xstate, 0);
    println!("mail test OK!");
    0
}
