# Fluent-Onyx

Система локализации для BYOND.

## Компиляция

Для компиляции требуется `rustup`.

Перед компиляции необходимо добавить компоненты для сборки под 32-битные системы:

- На Windows:

```shell
$ rustup target add i686-pc-windows-msvc
```

- На Linux:

```shell
$ rustup target add i686-unknown-linux-gnu
```

Затем установить `cargo-make`:

```shell
$ cargo install cargo-make
```

- Для компиляции в режиме `development` (без оптимизации):

```shell
$ cargo make dev
```

- Для компиляции в режиме `release`:

```shell
$ cargo make
```
