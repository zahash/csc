<div align="center">

<pre>
 ██████╗███████╗ ██████╗
██╔════╝██╔════╝██╔════╝
██║     ███████╗██║     
██║     ╚════██║██║     
╚██████╗███████║╚██████╗
 ╚═════╝╚══════╝ ╚═════╝
------------------------
Command Line Scientific Calculator. Free Forever. Made with ❤️ using 🦀
</pre>

[![Crates.io](https://img.shields.io/crates/v/csc.svg)](https://crates.io/crates/csc)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

</div>

## Download

[https://github.com/zahash/csc/releases](https://github.com/zahash/csc/releases)

( or )

```
cargo install csc
```

## Usage examples

launch the interactive prompt by typing `csc` to run multiple computations

```sh
a = 10
b = a + 1.14
c = log(b, 3) + sin(PI)
```

or run one off computations by simply providing them

```sh
$ csc 10 + 1.14
$ csc '10 + 1.14 * ln(50)'
```

## Features

```sh
# basic arithmetic and assignment
a = 1
b = -2 % a * (3^2 / 4)
b += 100

# functions
exp(x)
sqrt(x)
cbrt(x)
abs(x)
floor(x)
ceil(x)
round(x)

ln(x)
log2(x)
log10(x)
log(x, b)

sin(rad)
cos(rad)
tan(rad)

sinh(rad)
cosh(rad)
tanh(rad)

asin(rad)
acos(rad)
atan(rad)

asinh(rad)
acosh(rad)
atanh(rad)
```

## Meta

M. Zahash – zahash.z@gmail.com

Distributed under the MIT license. See `LICENSE` for more information.

[https://github.com/zahash/](https://github.com/zahash/)

## Contributing

1. Fork it (<https://github.com/zahash/csc/fork>)
2. Create your feature branch (`git checkout -b feature/fooBar`)
3. Commit your changes (`git commit -am 'Add some fooBar'`)
4. Push to the branch (`git push origin feature/fooBar`)
5. Create a new Pull Request
