# Simple Progress Bar

This is a progress bar implementation in Rust that can be used to display the progress of long-running tasks.

## Usage

To use the progress bar, first clone this repository and add it as a dependency to your Rust project by adding the following to your Cargo.toml file:

```toml
[dependencies]
progress_bar = { path = "<path to simple-progress-bar-rs>" }
```

Then, in your Rust code, import the ProgressBar struct from the progress_bar crate:

```rust
use progress_bar::ProgressBar;
```

You can then create a new ProgressBar instance with the desired length:

```rust
let mut progress_bar = ProgressBar::new(30);
```

After creating the progress bar, set the total number of units that the progress bar is tracking:

```rust
progress_bar.set_total(100);
```

Finally, you can update the progress bar with the current progress and a message:

```rust
progress_bar.update(50, "Processing...");
```

The progress bar will be displayed in the console, showing the current progress as a percentage and a bar that grows as the progress increases.

## Example

Here's an example usage of the progress bar:

```rust
use progress_bar::ProgressBar;
use std::{thread, time};

fn main() {
    let mut progress_bar = ProgressBar::new(30);
    progress_bar.set_total(100);

    for i in 0..=100 {
        progress_bar.update(i, "Processing...");
        thread::sleep(time::Duration::from_millis(50));
    }
}
```

## License

This project is licensed under the MIT license. See the [LICENSE](LICENSE.md) file for details.
