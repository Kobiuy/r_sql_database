Struktura projektu:
Projekt został podzielony na 13 modułów

- ui:
  Moduł zawierajacy metody generujące interfejs użytkownika, dla każdej strony zdefiniowanej w app.rs.
- app:
  Moduł zawierajacy definicje enumó CurrentCommand i CurrentScreen, struktury app (zawierającej przechowującej aktualny stan interfejsu użytkownika) i metodę definiującej kolejne przejścia między ekranami aplikacji.
- command_history:
  Moduł definiujący struktrę przechowujacą wywołane w aplikacji komendy.
- commands:
  Moduł definiujący komendy dostępne w aplikacji oraz 2 traity "Command" i "Serialize".
- condition:
  Moduł definiujący warunek dla komendy select oraz enum definiujący możliwe porównania "Op".
- custom_error:
  Moduł definiujący enum CustomError, definiuący wszystkie błędy, jakie możńa napotkać w apliakcji.
- database:
  Moduł zawierający strukturę bazy danych, tabelę i typy wartości mogęce wystąpić.
- event_handler:
  Moduł definiujący zachowanie programu w trybie UI, po wciśnięciu wybranych przycisków.
- handlers:
  Moduł pośredniczący między komendami, a interfejsami użytkownika.
- arguments:
  Moduł zawierający strukturę argumentów i funkcje z pętlami programu (1 od CLI i jedną od TUI).
- parsers;
  Moduł pomagający parsować listy pól do odpowiednich struktur.
- server:
  Moduł odpowiedzialny za uruchamianie serwera UDP, odbieranie i wysyłanie wiadomości.
- main:
  Moduł zawierający funkcję wywołującą funkcję z pętlą programu, na podstawie podanego przez użytkownika argumentów.

Ulubiony moduł:
Mój ulubiony moduł to "server". Dlaczego? Mały, prosty, ale interesująca funkcjonalność.

Uruchomienie Programu:

- Serwer: cargo run --bin server
- Konsola: cargo run --bin console <[string, int]> <[command, graphic]>

Używanie TUI:

- Pg Up -> Dodanie wybranych wartości do listy elementów (Np. przy wpisywaniu pól, jakie mają się pojawić jako wynik zapytania 'Select')
- Esc -> Anulacja polecenia/zamknięcie programu
- Arrow Left/Right -> Nawigacja prawo lewo, między polami (Np. Key <-> Type)
- Arrow Up/Down -> Nawigacja góra <-> dół, w listach poziomych (Np. wybieranie operacji do warunku w 'Select')
- Enter -> Zatwierdzanie wyborów
