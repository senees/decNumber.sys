# Issue-1 Add bindings for decQuadxxx functions

Zadanie składa się z trzech elementów:

- dodanie bindingów C (unsafe),
- dodanie bindingów Rust (safe),
- dodanie testów do bindingów Rust.

Funkcje biblioteczne zaczynające się od przedrostka decQuad operują
na liczbach dziesiętnych z dokładnością do 128-bitów (34 cyfry).
Więcej można poczytać w oryginalnej dokumentacji firmy IBM,
plik PDF jest dostępny w repo w katalogu:

```
./decNumber-icu-368/decnumber.pdf
```

W pliku `./src/dec_quad_c.rs` znajdują się definicje bindingów do C.
W pliku `./src/dec_quad.rs` znajdują się definicje bindingów do Rust.
W pliku `./tests/dec_quad.rs` znajdują się testy bindingów do Rust.

Lista nazw funkcji do zbindowania znajduje się w komentarzu w pliku `./src/dec_quad_c.rs`.