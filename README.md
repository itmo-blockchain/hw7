# Biletgen (HW7)

## Description

This is a simple generator of bilet number for students.

## Build

```bash
cargo build --release

# and then you can find the binary in ./target/release/biletgen

```

## Usage

### Help

```bash
./biletgen --help
              
A application to generate task number for students

Usage: biletgen --numbilets <NUMBILETS> --file <FILE> --parameter <PARAMETER>

Options:
  -n, --numbilets <NUMBILETS>  Count of bilets
  -f, --file <FILE>            Path to file with students
  -p, --parameter <PARAMETER>  Parameter for randomize
  -h, --help                   Print help information
  -V, --version                Print version information
```

### File format

File with students should be in format:

```text
Surname Name Patronymic
Surname Name Patronymic
...
```

### Example

```bash
./biletgen --numbilets 10 --file example.in --parameter 10

Иванов Георгий Петрович: 2
Клишов Пётр Михайлович: 7
Иванов Павел Алексеев: 8
Тестов Тест Тестович: 1
Петров Яша Петрович: 0
Выдрин Клим Саныч: 7
Дмитриев Александр Владимирович: 9
Кузнецов Михаил Витальевич: 0
Харитонов Роман Александрович: 6
Кузнецов Александр Владимирович: 7
Ярцев Ярослав Ярославович: 6
```



