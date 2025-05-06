# rust_example_cov

    - show how works rust coverage inside MS Vscode using the expansion coverage-gutters
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

```bash <!-- markdownlint-disable-line code-block-style -->
cat << EOF >> ./src/main.rs
pub fn answer() -> u32 {
    42
}

pub fn not_answer() -> u32 {
    43
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
}
EOF
```

## [Inserting an additional command line to call a function in the same file](https://stackoverflow.com/questions/15559359/insert-line-after-match-using-sed)

### [sed option](https://linux.die.net/man/1/sed)

- -i -> replace insert the file

### {regex flags}[https://www.codeguage.com/courses/regexp/flags]

- /a -> append to the match find by regex
- \t -> add a tabulator

```bash <!-- markdownlint-disable-line code-block-style -->
sed -i  '/println!("Hello, world!");/a\
\tprintln!("{} {} ", answer(), not_tested());' src/main.rs
```

> [!NOTE]
> [Insert line after match using sed](https://stackoverflow.com/questions/15559359/insert-line-after-match-using-sed)
&nbsp;
> [!NOTE]
> [When should I use \A in a regex?](https://stackoverflow.com/questions/2650549/when-should-i-use-a-in-a-regex)

## Run unit test

```bash <!-- markdownlint-disable-line code-block-style -->
cargo test
```

## install extension tarpulin

> [!NOTE]
> [How do I list all of the packages I've installed **globally** with cargo install?](https://stackoverflow.com/questions/60857222/how-do-i-list-all-of-the-packages-ive-installed-globally-with-cargo-install)
>
> ```bash<!-- markdownlint-disable-line code-block-style -->
> cargo install --list
> ```
><!--- THis empty line inside the block is necessary for correct format -->
&nbsp;

## GITHUB Marker

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
