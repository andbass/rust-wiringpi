#![allow(dead_code)]

/* automatically generated by rust-bindgen */

#[link(name = "wiringpi", kind = "static")]
extern "C" {
    pub fn pinMode(arg1: ::libc::c_int, arg2: ::libc::c_int);
    pub fn getAlt(arg1: ::libc::c_int) -> ::libc::c_int;
    pub fn pullUpDnControl(arg1: ::libc::c_int, arg2: ::libc::c_int);
    pub fn digitalWrite(arg1: ::libc::c_int, arg2: ::libc::c_int);
    pub fn digitalWriteByte(arg1: ::libc::c_int);
    pub fn gpioClockSet(arg1: ::libc::c_int, arg2: ::libc::c_int);
    pub fn pwmWrite(arg1: ::libc::c_int, arg2: ::libc::c_int);
    pub fn setPadDrive(arg1: ::libc::c_int, arg2: ::libc::c_int);
    pub fn digitalRead(arg1: ::libc::c_int)-> ::libc::c_int;
    pub fn pwmSetMode(arg1: ::libc::c_int);
    pub fn pwmSetRange(arg1: ::libc::c_uint);
    pub fn pwmSetClock(arg1: ::libc::c_int);
    pub fn waitForInterrupt(arg1: ::libc::c_int, arg2: ::libc::c_int) -> ::libc::c_int;
    pub fn wiringPiSetup() -> ::libc::c_int;
    pub fn wiringPiSetupSys() -> ::libc::c_int;
    pub fn wiringPiSetupGpio() -> ::libc::c_int;
    pub fn wiringPiSetupPiFace() -> ::libc::c_int;
    pub fn piBoardRev() -> ::libc::c_int;
    pub fn wpiPinToGpio(wpiPin: ::libc::c_int) -> ::libc::c_int;
    pub fn wiringPiSetupPiFaceForGpioProg() -> ::libc::c_int;
    pub fn wiringPiISR(pin: ::libc::c_int, mode: ::libc::c_int,
                       function: ::std::option::Option<extern "C" fn()>) ->
     ::libc::c_int;
    pub fn piThreadCreate(_fn:
                              ::std::option::Option<extern "C" fn
                                                        (arg1:
                                                             *mut ::libc::c_void)
                                                        ->
                                                            *mut ::libc::c_void>)
     -> ::libc::c_int;
    pub fn piLock(key: ::libc::c_int);
    pub fn piUnlock(key: ::libc::c_int);
    pub fn piHiPri(pri: ::libc::c_int) -> ::libc::c_int;
    pub fn delay(howLong: ::libc::c_uint);
    pub fn delayMicroseconds(howLong: ::libc::c_uint);
    pub fn millis() -> ::libc::c_uint;
    pub fn micros() -> ::libc::c_uint;
}
