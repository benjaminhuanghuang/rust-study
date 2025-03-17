https://doc.rust-lang.org/std/fmt/index.html


{...} surrounds all formatting directives

: separates the name or ordinal of the thing being formatted

The ? is a formatting option that triggers the use of the std::fmt::Debug implementation

{:?} formats the "next" value passed to a formatting macro, and supports anything that implements Debug.
