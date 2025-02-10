# Project management with Cargo

set the toolchain for a specific project or directory

```sh
# Only applies to the current directory and its children
$ rustup override set nightly
```

| Specification | Example        | Range Start | Range End | Allowed |
| ------------- | -------------- | ----------- | --------- | ------- |
| Caret         | ^2.3.4         | >=2.3.4     | <3.0.0    | Yes     |
| Caret         | ^2.3           | >=2.3.0     | <3.0.0    | Yes     |
| Caret         | ^0.2.3         | >=0.2.3     | <0.3.0    | Yes     |
| Caret         | ^2             | >=2.0.0     | <3.0.0    | Yes     |
| Tilde         | ~2.3.4         | >=2.3.4     | <2.4.0    | Yes     |
| Tilde         | ~2.3           | >=2.3.0     | <2.4.0    | Yes     |
| Tilde         | ~0.2           | >=0.2.0     | <0.3      | Yes     |
| Wildcard      | 2.3.\_         | >=2.3.0     | <2.4.0    | Yes     |
| Wildcard      | 2.\_           | >=2.0.0     | <3.0.0    | Yes     |
| Wildcard      | \*             | None        | None      | Yes     |
| Comparison    | =2.3.4         | =2.3.4      | =2.3.4    | No      |
| Comparison    | >=2.3.4        | >=2.3.4     | None      | Yes     |
| Comparison    | >=2.3.4,<3.0.0 | >=2.3.4     | <3.0.0    | Yes     |

## Cargo.lock

To library: Leaving out the Cargo.lock file allows downstream packages to
update indirect dependencies as needed.

For applications, it’s recommended you always include Cargo.lock alongside
Cargo.toml. This helps to ensure consistent behavior in published releases, should
third-party libraries change in the future.
