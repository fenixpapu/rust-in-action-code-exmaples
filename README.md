# Rust in action code examples

- My source code follow [Rust in action](https://www.amazon.com/Rust-Action-TS-McNamara/dp/1617294551/ref=sr_1_1?keywords=rust+in+action&link_code=qs&qid=1671208706&sr=8-1)

- Hopefully I do it well :D


## Some note ?
- Strings in Rust are able to include a wide range of characters

- Sometime we got errors like: `E0373` we can use command to see more detail: `rustc --explain E0373`

- Rust doesn't protect you from logical errors. It ensure that your data is never able to be written in two places at the same time. It does not ensure that your program is free from all security issues.

- [Rust at npm](https://www.rust-lang.org/static/pdfs/Rust-npm-Whitepaper.pdf) :D

- Fun fact current (12/2022) rust-lang.org hosted on heroku :D

- ![rust-lang-error](./rust-lang-error.png)

- Dont push ';' at the end of expression if you want return it's result and without `return` keyword. This changes the semantics, returning () (unit) rather than i32. See `c2-first-steps` to understand.

- It's safest to cast the smaller type to a larger one. Use cast with caution: 300_i32 as i8 will return 44.

- The three form of `for` each map to difference method:

  | Shorthand | Equivalent to | Access |
  | ---       | ---           | ---    |
  | for item in collection | for item in IntoIterator::into_iter(collection) | Ownership |
  | for item in &collection | for item in collection.iter() | Read-Only |
  | for item in &mut collection | for item in collection.iter_mut() | Read-Write |

- Rust has no concepts of `truthy` and `falsey`. The only value can used for `true` is `true` and for `false`, use `false`. -> Free comment: I like this feature :D.

- Some principles in Rust:
  - Terms in lowercase (`i`, `j`) denote variables.
  - Single uppercase letters(`T`) denote generic type variables.
  - Terms beginning with uppercase (Add) are either `traits` or concrete types, such as `String` or `Duration`.
  - Lables (`'a`) denote lifetime parameters.

- To install cargo doc locally( sometime helpful). Run command `cargo doc` and `cargo doc --open` to open in web browser. Or Rust doc with `rustup doc`.

- By convention, global variables use `ALL CAPS`.

- Four general strategies can help with ownership issues:
  - Use references where full ownership is not required:
    - For read-only use `&T` for read-write use `&mut T`.
  - Duplicate the value.
  - Refactor code to reduce the number of long-lived objects.
  - Wrap your data in a type designed to assist with movement issues.

- Rust difference `Copy` and `Clone` , the reason why copy is faster than clone. [Here](https://github.com/rust-lang/rust/blob/2e6eaceedeeda764056eb0e2134735793533770d/src/libcore/marker.rs#L272)

- `std::rc::Rc` -> `Rc<T>` reads: `R. C. of T` and stands for `a reference-counted value of type T`.

- Làm sao tìm được nhanh nhất 2 số có cùng biểu diễn nhị phân nhưng khác kiểu dữ liệu. Ví dụ 1 số kiểu unsigned int và 1 số kiểu int: Lấy đại 1 số thuộc unsigned int rồi chuyển về nhị phân và quy đổi nhị phân đó theo kiểu mới ( int :D). Check ví dụ: `c5-int-vs-int` chứa 1 cặp như vậy.

- `impl From<T> for U` explain how to convert from type `T` to type `U`.

