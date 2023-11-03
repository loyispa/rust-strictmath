# rust-strictmath
This crate is inspired by java  [StrictMath](https://github.com/openjdk/jdk/blob/jdk8-b120/jdk/src/share/native/java/lang/StrictMath.c). If obtaining a completely predictable result is more important than running speed, then the crate should be used.This crate provide series of float functions to ensure they could produce the same results across different platforms. These algorithms are available from the well-known network library netlib as the package "Freely Distributable Math Library," [fdlibm](https://netlib.org/fdlibm/). 

# function list
- acos
- asin
- atan
- atan2
- cbrt
- cosh
- expm1
- hypot
- log
- log1p
- sinh
- sqrt
- tan
- tanh

# Usage

Run the following Cargo command in your project directory:
```
cargo add rust-strictmath
```


Or add the following line to your Cargo.toml:
```
rust-strictmath = "0.1.1"
```
