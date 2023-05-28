# detect-login-shell-rs

## Usage

```rust
use detect_login_shell;

fn main() {
    let shell = detect_login_shell::detect().expect("Failed to detect");

    println!("Login shell is {}", shell)
}
```
