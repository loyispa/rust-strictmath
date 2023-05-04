#ifndef RFDLIBM_H
#define RFDLIBM_H

/*
 * In order to resolve the conflict between fdlibm and compilers
 * (such as keywords and built-in functions), the following
 * function names have to be re-mapped.
 */

#define acos    racos
#define asin    rasin
#define atan    ratan
#define atan2   ratan2
#define cos     rcos
#define exp     rexp
#define log     rlog
#define log10   rlog10
#define pow     rpow
#define sin     rsin
#define sqrt    rsqrt
#define cbrt    rcbrt
#define tan     rtan
#define floor   rfloor
#define ceil    rceil
#define cosh    rcosh
#define fmod    rmod
#define log10   rlog10
#define sinh    rsinh
#define fabs    rfabs
#define tanh    rtanh
#define remainder rremainder
#define hypot   rhypot
#define log1p   rlog1p
#define expm1   rexpm1

#define __ieee754_sqrt          __r__ieee754_sqrt
#define __ieee754_acos          __r__ieee754_acos
#define __ieee754_log           __r__ieee754_log
#define __ieee754_atanh         __r__ieee754_atanh
#define __ieee754_asin          __r__ieee754_asin
#define __ieee754_atan2         __r__ieee754_atan2
#define __ieee754_exp           __r__ieee754_exp
#define __ieee754_cosh          __r__ieee754_cosh
#define __ieee754_fmod          __r__ieee754_fmod
#define __ieee754_pow           __r__ieee754_pow
#define __ieee754_log10         __r__ieee754_log10
#define __ieee754_sinh          __r__ieee754_sinh
#define __ieee754_hypot         __r__ieee754_hypot
#define __ieee754_remainder     __r__ieee754_remainder
#define __ieee754_rem_pio2      __r__ieee754_rem_pio2
#define __ieee754_scalb         __r__ieee754_scalb
#define __kernel_standard       __r__kernel_standard
#define __kernel_sin            __r__kernel_sin
#define __kernel_cos            __r__kernel_cos
#define __kernel_tan            __r__kernel_tan
#define __kernel_rem_pio2       __r__kernel_rem_pio2
#define __ieee754_log1p         __r__ieee754_log1p
#define __ieee754_expm1         __r__ieee754_expm1
#endif/*RFDLIBM_H*/