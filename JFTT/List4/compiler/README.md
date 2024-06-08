# Autor: 
Albert Kołodziejski

# Kompilacja:
kompilator został napisany w języku rust w wersji 1.75.0 (82e1608df 2023-12-21), aby zainstalować kompilator rust należy postępować zgodnie z instrukcjami na [stronie](https://www.rust-lang.org/tools/install)

Po zainstalowaniu kompilatora rust należy przejść do katalogu z projektem i wykonać polecenie:
> cargo build --release

# Uruchomienie:
Aby uruchomić można wykonać jedno z poleceń:
> cargo run --release -- <nazwa_pliku_do_przetworzenia> <nazwa_pliku_wynikowego>

Podczas sprawdzania poprawności wyszedł problem z tym sposobem, ponieważ za każdym razem program się kompilował od nowa. Dlatego też podaje bezpośredni sposób uruchomienia:

> ./target/release/compiler <nazwa_pliku_do_przetworzenia> <nazwa_pliku_wynikowego>

