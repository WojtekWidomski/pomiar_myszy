# Program do mierzenia szybkości celowania myszką

Program do mierzenia szybkości celowania myszką użytkownika napisany w Ruscie.

Wyświetla w losowych miejscach na monitorze czarny kwadrat i wypisuje na
standardowe wyjście początkową odległość od wskaźnika myszy i czas w formacie
csv. Zaczyna mierzyć czas dopiero, kiedy użytkownik ruszy myszką, żeby mierzyć
czas najechania i kliknięcia, a nie czas reakcji. Może nie liczyć kilku
pierwszych kliknięć.

## Obsługa

```
cargo run --release
```

Żeby zapisać do pliku wyjście.

```
cargo run --release > output.csv
```

Żeby zmienić ustawienia można zmienić stałe na początku `main.rs`.

## Technologie

W pliku `Cargo.toml` są wszystkie biblioteki.

- Język Rust
- [Biblioteka graficzna Piston](https://github.com/PistonDevelopers/graphics)
- glfw
