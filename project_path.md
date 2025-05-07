# rust_example_cov

    - These repo show how works rust coverage inside MS Vscode using the expansion coverage-gutters
    - a rocky road
  :smiley:

## date today

```bash <!-- markdownlint-disable-line code-block-style -->
date
Tue May  6 09:30:59 AM CEST 2025
```

## os- version

```bash <!-- markdownlint-disable-line code-block-style -->
uname -a
Linux debian 6.1.0-28-amd64 #1 SMP PREEMPT_DYNAMIC Debian 6.1.119-1 (2024-11-22) x86_64 GNU/Linux
```

## create project folder on linux console e.g. inside your own home directory

```bash <!-- markdownlint-disable-line code-block-style -->
# cd && mkdir <project_name> && cd $_
cd && mkdir rust-example-cov && cd $_ 
```

## init

```bash <!-- markdownlint-disable-line code-block-style -->
touch README.md \
&& ln -s README.md README \
&& cargo init "." \
&& cargo add rustfmt \
&& rustup component add rustfmt \
&& mkdir examples \
&& cp src/main.rs examples/example.rs \
&& sed -i -e 's/world/example/g' examples/example.rs \
&& rustup  show \
&& rustup  check \
&& rustup toolchain uninstall stable \
&& rustup toolchain install stable \
&& rustup update  --force \
&& rustup show \
&& mkdir tests
```

## Add this testcase to /src/main.rs [From Here](https://stackoverflow.com/questions/22697688/how-to-cat-eof-a-file-containing-code)

> [!TIP]
> Run the script for generate the testcase inside bash terminal

```bash <!-- markdownlint-disable-line code-block-style -->
cat << EOF >> ./src/main.rs
fn main() {
    println!("Dummy fn main!");
}

// prettier-ignore
pub fn answer() -> u32 {
    // for coverage is a instruction necessary
    42 + 0
}

// prettier-ignore
pub fn not_tested() -> u32 {
    // for coverage is a instruction necessary
    43 + 0
}

// prettier-ignore
pub fn greater_than_5(num: u32) -> u32 {
    if num > 5 {
        1
    }else {
        0
    }
}

#[allow(dead_code)]
// prettier-ignore
fn smaller_than_5(num: u32) -> u32 {
    if num < 5 {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_answer() {
    assert_eq!(answer(), 42);
  }

  #[test]
  fn test_not_tested() {
    assert_eq!(not_tested(), 43);
  }

  #[test]
    fn test_greater_than_5() {
    assert_eq!(greater_than_5(6), 1
);
  }

  #[test]
    fn test_no_greater_than_5() {
    assert_eq!(greater_than_5(4), 0);
  }
}
EOF 
```

## [Inserting an additional command line to call a function in the same file](https://stackoverflow.com/questions/15559359/insert-line-after-match-using-sed)

### [option - Command option for the command itself](https://linux.die.net/man/1/sed)

- -i &rarr; replace insert the file

### [regex flags - Inside the regular expression](https://www.codeguage.com/courses/regexp/flags)

- /a &rarr; append to the match find by regex
- \t &rarr; add a tabulator

```bash <!-- markdownlint-disable-line code-block-style -->
sed -i  '/println!("Hello, world!");/a\
\tprintln!("{} {} ", answer(), not_tested());' src/main.rs
```

> [!NOTE]
> [Insert line after match using sed](https://stackoverflow.com/questions/15559359/insert-line-after-match-using-sed)
<!-- --><!--- THis empty line inside the block is necessary for correct format -->
> [!NOTE]
> [When should I use \A in a regex?](https://stackoverflow.com/questions/2650549/when-should-i-use-a-in-a-regex)
><!--- THis empty line inside the block is necessary for correct format -->

## Run unit test

```bash <!-- markdownlint-disable-line code-block-style -->
cargo test
```

## Install extension cargo-tarpaulin

```bash <!-- markdownlint-disable-line code-block-style -->
cargo install cargo-tarpaulin
```
<!--- THis empty line is necessary for correct format -->
> [!NOTE]
> [How do I list all of the RUST packages are installed **globally** with the command **cargo install**?](https://stackoverflow.com/questions/60857222/how-do-i-list-all-of-the-packages-ive-installed-globally-with-cargo-install)
>
> ```bash<!-- markdownlint-disable-line code-block-style -->
> cargo install --list
> ```
><!--- THis empty line inside the block is necessary for correct format -->
&nbsp;

### Run cargo-tarpaulin inside this project

```bash<!-- markdownlint-disable-line code-block-style -->
cargo tarpaulin --out Lcov
```
<!--- THis empty line inside the block is necessary for correct format -->
&nbsp;

### Speed up of the program flow through the features

- --target-dir $PWD/target/tarpaulin
- --skip-clean

```bash<!-- markdownlint-disable-line code-block-style -->
# run /w --target-dir and --skip-clean
cargo tarpaulin --ignore-tests --out Lcov --target-dir $PWD/target/tarpaulin --skip-clean
```
<!-- --><!--- THis empty line inside the block is necessary for correct format -->
&nbsp;

> [!NOTE]
> The command will generate two files
>
> 1. lcov.info
> 2. rust-example-cov/target/tarpaulin/rust-example-cov-coverage.json
<!-- --><!--- This empty line between the blocks is required for the correct format so that they are displayed separately -->
> [!NOTE]
> [How To Take A Screenshot under Linux | Computing](https://www.maths.cam.ac.uk/computing/linux/X/screenshots#:~:text=Use%20the%20shortcut%20key%20PrintScreen,will%20capture%20the%20whole%20desktop.)
> Use the shortcut key PrintScreen to capture the whole
> desktop and Alt+PrintScreen to capture the current
> window (these also work under Windows).

### Enable Show Line Coverage Show Line Coverage in MS VSCode

- Edit the settings of the MS VSCode extension **coverage-gutters**
- Select for that the settings (gear) icon in the extension overview
- Select **ALL** Boxes extension option show line coverage [see img](./img/coverage-gutters-show-line-coverage.png) - see image
<!-- --><!--- THis empty line inside the block is necessary for correct format -->

### rustfmt

- is rustfmt already installed with which version
  
  ```bash <!-- markdownlint-disable-line code-block-style -->
   rustfmt -V
        rustfmt 1.8.0-stable (05f9846f89 2025-03-31)
  ```

- install rustfmt local
  
```bash <!-- markdownlint-disable-line code-block-style -->
  
  
  # install binary rustfmt
  &   ```


- delete rustfmt local
  
  ```bash <!-- markdownlint-disable-line code-block-style -->
  # list where is rustfmt already installed - system wide - user /home folder
  $ ls -l ~/.cargo/bin
  
  # remove standard way
  rustup component remove rustfmt

  # FORCE delete binary rustfmt
  & rm -rf  ~/.cargo/bin/rustfmt

  # clean up rustup registry
  rustup component remove rustfmt
  ```

  > [!NOTE]
  > If rustfmt is not installed but it is called, the following error message appears
  >
  >```text <!-- markdownlint-disable-line code-block-style -->
  > [Error - 9:38:34 AM] Request textDocument/formatting failed.
  > Message: Failed to spawn cd "/home/trapapa/rust-example-cov/src" && "rustfmt" "--edition" "2024"
  > Code: -32603 
  > ```
  <!-- --><!--- THis empty line inside the block is necessary for correct format -->

### [Prettier-Code Formatter Rust](https://github.com/jinxdash/prettier-plugin-rust)

> [!NOTE]
> [Extension Prettier - Code Formatter - How to ignore things - prettier-ignore](https://github.com/jinxdash/prettier-plugin-rust)
>
> - Add // prettier-ignore above instruction it
> - **DOESN'T WORK FOR WHATEVER REASON**Add #![rustfmt::skip] inside blocks or files
> <!-- --><!--- THis empty line inside the block is necessary for correct format -->
&nbsp;
> [!IMPORTANT]
> **TEST FIRST** YOU HAVE THE CORRECT EXTENSION INSTALLED
> [The correct version of Prettier-Code Formatter for RUST](img/Prettier-Code_Formatter_Rust.png)
> indicates missing/incorrect version
> [Prettier-Code Formatter: Can Not Format Rust Files](img/Prettier-Code_Formatter_Can_Not_Rust_Files.png)
> [The wrong version of Prettier-Code Formatter](img/Prettier-Code_Formatter.png)
<!-- --><!--- THis empty line inside the block is necessary for correct format -->
&nbsp;

<!-- --><!--- THis empty line inside the block is necessary for correct format -->
&nbsp;

## Markdown Marker - works on GitHub

> [!NOTE]
> Useful information that users should know, even when skimming content
<!-- -->
> [!TIP]
> Helpful advice for doing things better or more easily
<!-- -->
> [!IMPORTANT]
> Key information users need to know to achieve their goal
<!-- -->
> [!WARNING]
> Urgent info that needs immediate user attention to avoid problems
<!-- -->
> [!CAUTION]
> Advises about risks or negative outcomes of certain actions
&rarr;
## Markdown arrow - works on GitHub

- Up arrow (↑): `&uarr;`
- Down arrow (↓): `&darr;`
- Left arrow (←): `&larr;`
- Right arrow (→): `&rarr;`
- Double headed arrow (↔): `&harr;`
  