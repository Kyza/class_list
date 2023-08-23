# class_list

[<img src="https://img.shields.io/badge/github-Kyza/class_list?style=for-the-badge&color=555555&labelColor=333333&logo=github" alt="GitHub Badge" height="20"/>][GitHub Link] [<img src="https://img.shields.io/crates/v/class_list.svg?style=for-the-badge&color=fc8d62&labelColor=333333&logo=rust" alt="crates.io Badge" height="20"/>][crates.io Link] [<img src="https://img.shields.io/badge/docs.rs-class_list?style=for-the-badge&color=555555&labelColor=333333&logo=docs.rs" alt="docs.rs Badge" height="20"/>][docs.rs Link] [<img src="https://img.shields.io/github/actions/workflow/status/Kyza/class_list/test.yml?branch=trunk&style=for-the-badge&labelColor=333333" alt="Build Status Badge" height="20"/>][Build Status Link]

[GitHub Link]: https://github.com/Kyza/class_list
[crates.io Link]: https://crates.io/crates/class_list
[docs.rs Link]: https://docs.rs/class_list
[Build Status Link]: https://github.com/Kyza/class_list/actions?query=branch%3Atrunk

A reactive helper that ensures normalized class list strings in frontend frameworks like [Leptos](https://github.com/leptos-rs/leptos).

## Usage

Examples provided will be for the [Leptos](https://github.com/leptos-rs/leptos) framework [post-{context removal}](https://github.com/leptos-rs/leptos/discussions/1509), but it will work pre-{context removal} and might work in other similar frameworks.

This library is meant to be agnostic and has no runtime dependencies, but it has only been tested with Leptos.

```bash
cargo add class_list
```

### Example

`class_list![]` by default wraps itself in a move closure, meaning it will be reactive by default.

```rs
let (count, set_count) = create_signal(0);

set_interval(
	move || {
		set_count.update(|count| *count += 1);
	},
	Duration::from_millis(100),
);

let count_class = move || format!("count-{}", count());
let count_is_even = move || count() % 2 == 0;

view! {
	<div class=class_list![
		"default-class-names",
		// Closures get called automatically.
		count_class,
		// Closures can be written directly into the macro.
		move || format!("count-{}", count()),
		// Both Option and Result can be used as values.
		// None and Err result in no class name.
		Some("option"),
		None::<String>,
		// More conveniently, class names can be bound to reactive toggles.
		// "even" will only be applied when `count_is_even()` is true.
		// You also don't need to call closures here.
		"even" <=> count_is_even
	] />
}
```

### Options

Each option must be followed by a `;`.

#### Raw

To generate a non-reactive String, add the `raw` option to the beginning.

```rs
class_list![
	raw;
	"default-class-names",
]
```

#### Clone

Rarely, you may need to clone something before passing it in.

The macro makes this easy with the `clone` option which clones the variable before it gets moved into the closure.

If possible try to avoid needing this option in the first place.

```rs
class_list![
	clone[count_class];
	"default-class-names",
]
```

#### Crate

This should never be needed because it's automatically supplied by a wrapper `macro_rules!`.

In a case where the trait imports cannot be resolved--such as when used inside of another library--, the path can be redefined.

The path should lead to the root of this library.

```rs
__class_list![
	crate = ::your_lib::class_list;
	"default-class-names",
]
```

### Implementing Traits

If you'd like to simply pass a type to the macro instead of converting it every time, you can implement the `ClassList` and `ClassToggle` types.

Check out [traits.rs](https://github.com/Kyza/class_list/blob/trunk/class_list/src/traits.rs) to see the default implementations which are good examples of implementing them.

```rs
// If you're using a type you don't own,
// you must wrap it in a new struct.
struct Bool(bool);

impl ClassList for Bool {
	fn to_class_list(&self, normalize: bool) -> String {
		// If the string could contain multiple class names
		// you should use `normalize` to determine whether
		// or not to call `.to_class_list()` on it before
		// returning.
		// If you're lazy you could always normalize, but
		// then the string will be normalized multiple
		// times for every update in the macro.
		if self.0 {
			"true".into()
		} else {
			"false".into()
		}
	}
}
impl ClassToggle for Bool {
	fn to_class_toggle(&self) -> bool {
		self.0
	}
}

// Option, Result, and Fn are implemented in a way which
// allows any new type you implement to be automatically
// passed through.
assert_eq!(
	class_list![
		// ClassList
		move || Bool(false),
		Bool(true),
		"class",
		// ClassToggle
		"hidden" <=> move || Bool(false),
		"list",
	](),
	"false true class list".to_string()
);
```
