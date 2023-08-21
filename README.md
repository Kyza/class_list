# class_list!

A reactive helper that ensures nice class list strings in frontend frameworks like [Leptos](https://github.com/leptos-rs/leptos).

## Usage

Examples provided will be for the [Leptos](https://github.com/leptos-rs/leptos) framework [post-{context removal}](https://github.com/leptos-rs/leptos/discussions/1509), but it will work pre-{context removal} and might work in other similar frameworks.

This library is meant to be agnostic and has no runtime dependencies, but it has only been tested with Leptos.

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