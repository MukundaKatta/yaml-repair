# yaml-repair

[![crates.io](https://img.shields.io/crates/v/yaml-repair.svg)](https://crates.io/crates/yaml-repair)

Repair messy YAML emitted by LLMs into something a real YAML parser
will accept.

```rust
use yaml_repair::repair;
let raw = "```yaml\n  name: Claude\n  tools:\n    - read\n```";
let fixed = repair(raw);
// fixed parses with serde_yaml / saphyr
```

Strips ``` fences, normalizes CRLF, converts leading tabs to spaces,
dedents, trims trailing whitespace. Zero deps. MIT or Apache-2.0.
