# Lexer phase

```js
let x = 2 + 4;

[
  LET,
  IDENTIFIER("×"),
  EQUAL_SIGN,
  INTEGER(2),
  PLUS_SIGN,
  INTEGER(4),
  SEMICOLON,
];
```

## Single character token

```rs
is_letter
skip_whitespace
read_identifier
```

## Support identity

```rs
is_letter
skip_whitespace
read_identifier
```

## Support number

```rs
read_number
```

## Support special / ! < > \*

## Support if else true false return
