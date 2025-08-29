# Ustvari svoj sudoku
## Projektna naloga pri predmetu Programiranje 2
Za projektno nalogo pri predmetu Programiranje 2 sva sestavili program, ki omogoči uporabniku, da sestavi svoj sudoku, ki ga potem program reši. 

Kdor želi program preizkusiti, naj najprej klonira git repozitorij in ga odpre v VSC. Nato naj se v terminalu z ukazom `cd sudoku` premakne v mapo sudoku. Tu naj zažene ukaza
-  `wasm-pack build --target web --release`
- `basic-http-server -a 0.0.0.0:4000`.
V terminalu se tedaj izpiše naslov spletne strani. S klikom nanjo se odpre spletna stran projekta.
Na spletni strani je pod naslovom gumb z napisom "Navodila", ki uporabniku dodatno razloži, kaj lahko s programom počne. 

Uporabili sva knjižnice sauron, wasm_bindgen in web_sys.

