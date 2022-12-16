# Zaawansowane programowanie w C++ 

Gra na przeglądarkę z użyciem WebAssembly

#### Dokumentacja wstępna projektu

Skład zespołu: Katarzyna Glaza, Jakub Nitkiewicz


[Ogólny opis projektu	1](#_Toc138335219)

[Temat projektu	1](#_Toc373063069)

[Zasady gry	1](#_Toc1065200622)

[Dodatkowe założenia	2](#_Toc750165419)

[Technologie	2](#_Toc793952479)

[Zadania do wykonania	2](#_Toc2088189025)

[Etap 1.	2](#_Toc2045757164)

[Etap 2.	3](#_Toc535475920)

[Planowane testy	3](#_Toc1717203983)

# Ogólny opis projektu
Zadanie polega na napisaniu gry w architekturze klient-serwer, klient powinien wykorzystywać jedynie przeglądarkę www. Dodatkowo klient powinien być zaimplementowany przy użyciu WebAssembly.

Gra powinna być przynajmniej dwuosobowa. Reguły gry - wikipedia lub inne źródło. Zespół może wybrać następujące opcje: Gra dwuosobowa, aplikacja dba o przestrzeganie zasad Gra jednoosobowa z komputerem, wtedy należy dostarczyć algorytm dla sztucznego gracza, wykorzystujący np. drzewo gry. Przy wyborze pierwszej opcji implementacja logiki gry nie musi znajdować się na serwerze.

WASM można generować m.in. na podstawie kodu w języku Rust albo C++.

# Temat projektu
Grą dwuosobową, którą zaimplementujemy są warcaby w wariancie polskim. Zostaną one napisane w dwóch trybach: jako gra dwuosobowa oraz gra z komputerem. Dodatkowo, zaimplementowany zostanie algorytm wyboru ruchów komputera - algorytm minimax z cięciami 𝛼−𝛽 . 

# Zasady gry
Zasady gry, według których zaimplementowana zostanie gra są następujące:

Pionki graczy umieszczone są na ciemnych polach planszy o rozmiarze 10x10. Każdy gracz rozpoczyna grę z dwudziestoma pionkami swojego koloru (białymi lub czarnymi). Pionki ustawione są na planszy w ten sposób, że dwa środkowe rzędy planszy są wolne. Jako pierwszy ruch wykonuje gracz poruszający się białymi pionkami, ruchy obydwu graczy są naprzemienne. Bicie pionków jest obowiązkowe do końca (jeżeli gracz ma możliwość bicia więcej niż jednego pionka, musi wykonać maksymalną liczbę bić). Celem gry jest zbicie wszystkich pionków przeciwnika lub uniemożliwienie wykonania przez niego ruchu. Pionek może poruszać się po przekątnej na wolne pole i ma możliwość swobodnego przejścia o jedno pole. W przypadku bicia, pionek może poruszać się do przodu oraz do tyłu wykonując więcej skoków. Pionek, który dotrze na przeciwny koniec planszy zamienia się w damkę. Damka porusza się o dowolną liczbę pól do przodu lub do tyłu po przekątnej. Damka bijąc pionka przeskakuje na pole za zbitym pionkiem i może kontynuować bicie na tej samej lub prostopadłej linii. 

# Dodatkowe założenia
Poza zastosowaniem zasad zgodnymi z polską wersją gry, wprowadzone zostaną także następujące założenia:

Użytkownik przed rozpoczęciem gry może wybrać jeden z jej dwóch dostępnych trybów: grę z innym użytkownikiem oraz grę z komputerem. Przy wyborze gry dwuosobowej użytkownik jest pytany o swoją nazwę i oczekuje aż do pojawienia się drugiego gracza. Przed rozpoczęciem rozgrywki, będzie miał możliwość wyboru koloru pionków, przyjmiemy również uproszczenie, że każdy użytkownik ma umiejscowione swoje pionki na spodzie planszy, a zbite pionki zostają usunięte z planszy po wykonaniu bicia do końca. Dodatkowo, na ekranie będą znajdowały się liczniki zbitych pionków dla obydwu graczy.

# Technologie
Gra zostanie zaimplementowana w języku Rust, w implementacji backendu wykorzystamy WebAssembly, a frontendu - JavaScript. Dodatkowo, skorzystamy z takich narzędzi jak:

- wasm-pack, który posłuży do budowania, testowania i publikowania WebAssembly generowanego przez Rust,
- cargo, który jest menadżerem pakietów Rust,
- npm, który jest menadżerem pakietów JavaScript,
- rustfmt, który służy do formatowania kodu,
- clippy, które służy do statycznej analizy kodu

# Zadania do wykonania
W realizacji praktycznej części naszego projektu wyróżniamy 2 podstawowe etapy.

W etapie 1. konfigurujemy wszystkie narzędzia i setupujemy środowisko, implementujemy frontend i backend aplikacji oraz zapewniamy przepływ informacji pomiędzy nimi, natomiast w etapie 2. implementujemy logikę gry w warcaby w obu trybach.

## Etap 1.
1. Skonfigurowanie zestawu narzędzi Rust do kompilacji do WebAssembly, podstawowe połączenie WebAssembly z JavaScript
1. Stworzenie GUI
1. Zapewnienie przepływu informacji pomiędzy wszystkimi komponentami
## Etap 2.
1. Implementacja logiki gry w trybie dla dwóch osób
1. Implementacja algorytmu minimax z cięciem 𝛼−𝛽 do wersji trybu gry z komputerem

# Planowane testy
Zostaną przeprowadzone testy jednostkowe poprawności działania metod związanych z logiką gry, a także testy integracyjne sprawdzające czy wszystkie komponenty współdziałają ze sobą poprawnie. Do napisanie testów wykorzystane zostanie narzędzie wbudowane cargo-test oraz wasm-pack. Dodatkowo, w algorytmie minimax dostrojony zostanie parametr głębokości tak, aby komputer nie grał “zbyt dobrze”, a użytkownik miał szansę wygrania rozgrywanej z nim partii.

# Szkielet Aplikacji
## Instrukcja zbudowania i przetestowania aplikacji:
**Wymagania wstępne:**

Narzędzia:
- rustup
- rustc
- rustdoc
- typedoc
- cargo
- cargo-generate
- npm

Pobranie ich jest opisane na stronie [https://rustwasm.github.io/book/game-of-life/setup.html](url)

**Następnie należy wykonać następujące kroki:**
- Sklonować projekt
- Będąc w katalogu projektu uruchomić w terminalu `npm install`
- Następnie można przetestować rozwiązanie wpisując `npm start`

## Generowanie dokumetacji
Dla rust:
`rustdoc rust-wasm-lib/src/lib.rs --crate-name docs`

Dla typesript:
`npx typedoc src/index.tsx`

