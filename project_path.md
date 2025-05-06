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
    fn test_not_answer() {
        assert_eq!(not_answer(), 43);
    }
}
EOF
```

## Add method call to fn main inside /src/main.rs

- sed /a => append

```bash <!-- markdownlint-disable-line code-block-style -->
sed -i  '/println!("Hello, world!");/a\
\tprintln!("{} {} ", answer(), not_tested());' src/main.rs
```

> [!NOTE]
> [Insert line after match using sed](https://stackoverflow.com/questions/15559359/insert-line-after-match-using-sed)
&nbsp;
> [!NOTE]
> [When should I use \A in a regex?](https://stackoverflow.com/questions/2650549/when-should-i-use-a-in-a-regex)

weiter
https://www.google.com/search?q=What+is+%27%5Ca%27+in+regex%3F&client=firefox-b-e&sca_esv=274d2c2221d041eb&channel=entpr&sxsrf=AHTn8zpBUdtjWPA3hrqo-QMjpZU0xgnZxw%3A1746519625394&ei=ScYZaJ7xF9Hs7_UPjbX88AI&ved=0ahUKEwjeufXOtI6NAxVR9rsIHY0aHy4Q4dUDCBA&uact=5&oq=What+is+%27%5Ca%27+in+regex%3F&gs_lp=Egxnd3Mtd2l6LXNlcnAiFldoYXQgaXMgJ1xhJyBpbiByZWdleD8yBhAAGAcYHjIGEAAYBxgeMgYQABgHGB4yBhAAGAcYHjIGEAAYBxgeMgYQABgHGB4yBhAAGAcYHjIGEAAYBxgeMgYQABgHGB4yBhAAGAcYHkjUbFC7KVinPHACeAGQAQCYAVGgAe4BqgEBM7gBA8gBAPgBAZgCBaAC1gLCAgoQABiwAxjWBBhHmAMAiAYBkAYHkgcDNC4xoAfeEbIHAzIuMbgHsAI&sclient=gws-wiz-serp 

## Run unit test

```bash <!-- markdownlint-disable-line code-block-style -->
cargo test
```

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
