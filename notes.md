## Move vs Borrow
```rust
move
let x = 5;  // x -> 5
let y = x;  // y -> 5

borrow (immutable reference)
let x = 5;   // x -> 5
let y = &x;  // y -> x -> 5
*y    + 3  // 5 + 3
*(&x) + 3  // 5 + 3

let &z = &x;  // let z = *&x
let &(z) = &(5)
```

```
'4d616e'
[4, 13, 6, 1, 6, 14]
[77, 97, 110]  -> 3 x (8 bits)
[0b01001101, 0b01100001, 0b01101110]  -> 3 x (8 bits)
0b01001101|01100001|01101110
0b010011|010110|000101|101110
[...]  -> 4 x (6 bits)

77 * 256 = 0b01001101 * 0b100000000
         = 0b0100110100000000
    + 97 =         0b01100001
         = 0b0100110101100001
```

## Iterable
Base `Iterator` trait
```
.iter()
         vvvv ---- Iterable
for x in list:
    x
```
`.map(func)` method
```python
for x in list:
    func(x)
```
`.fold(init, func)` method
```python
acc = init
for x in list:
   acc += func(x)
```

## UTF8 Decoding of Hex
```
char -> utf8 -> value
'4'  -> 52   -> 4
'd'  -> 100  -> 13
```

## Bitwise XOR Operation

`a ^ b` with `^` caret operator
`true ^ true == false`
`true ^ false == true`
`false ^ false == false`

```
  01101010
^ 00001111
----------
  01100101
```