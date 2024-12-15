# Grpe
Grpe is a 3d renderer for the terminal. It is completely CPU-based (unless your terminal is GPU-accelerated, of course) and does not rely on external libraries other than some platform-specific c-libraries.

## TODO
1. Rotation
2. I/O via raw input mode (extern c)
3. Signal handling (extern c)
4. Draw lines between vertices.
5. Create wrapper script to run with "cargo run --release --example spiral -- $(tput cols) $((($(tput lines) * 2 - 5)))"

## Note
mate-terminal --full-screen --hide-menubar -e "target/release/grpe -m spiral -i"

47.5 * x = 31.5

## What is slow
- The 'update' method of canvas changes size EVERY time. The if-statements are wrong.
- It takes a long time to just CLEAR the buffer (Terminal::clear).
- Writing to the stdout buffer takes a long time, and writes take long. Make stdout_buffer one continous piece of memory for faster writes.

while [ : ]; do cargo run --release -- -i -m plane -r $(tput cols) $((($(tput lines) * 2 - 5))); done


```Rust
struct Data<'a> {
    value: &'a u32,
}

struct Foo<'a> {
    data: Data<'a>,
}

struct Bar<'a> {
    foo: Vec<Foo<'a>>,
}

impl<'a> Bar<'a> {
    pub fn foo_mut(&mut self, index: usize) -> &'a mut Foo {
        &mut self.foo[index]
    }
}

fn set_and_check<'a>(buffer: &'a mut Bar<'a>) {
    for _ in 0..10 {
        let pixel = buffer.foo_mut(0);
    }
}

fn main() {
    let core = 0;
    let data = Data {
        value: &core,
    };
    let foo = Foo {
        data: data,
    };
    let bar = Bar {
        foo: vec![foo],
    };
}
```

Test
