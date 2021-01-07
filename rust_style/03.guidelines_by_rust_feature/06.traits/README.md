Trait objects are useful primarily when heterogeneous collections of objects need to be treated uniformly; it is the closest that Rust comes to object-oriented programming

```
struct Frame  { ... }
struct Button { ... }
struct Label  { ... }

trait Widget  { ... }

impl Widget for Frame  { ... }
impl Widget for Button { ... }
impl Widget for Label  { ... }

impl Frame {
    fn new(contents: &[Box<Widget>]) -> Frame {
        ...
    }
}

fn make_gui() -> Box<Widget> {
    let b: Box<Widget> = box Button::new(...);
    let l: Box<Widget> = box Label::new(...);

    box Frame::new([b, l]) as Box<Widget>
}
```

### Using traits to share implementations

Traits that provide default implmentations for function can provide code reuse across types. For example, a print method can be defined across multiple types as follows:

```
trait Printable {
    // Default method implementation
    fn print(&self) { println!("{:?}", *self) }
}

impl Printable for int {}

impl Printable for String {
    fn print(&self) { println!("{}", *self) }
}

impl Printable for bool {}

impl Printable for f32 {}
```
