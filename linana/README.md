Implementierung der Linearen Analyse Attacke auf ein SPN Netzwerk 

Es wurden mit dem Schlüssel 0xA346 (in jeder Runde gleich) und der SPN Verschlüsselung 8000 Klartext- / Kryptotextpaare erstellt. 
Diese sind im Ordner data abgelegt.

Mit der Linearen Analyse können die Bits 4-7 sowie 12-15 des letzten Rundenschlüssels zu einer hohen Wahrscheinlichkeit richtig ermittelt werden. Dies erleichtert einen Brute Force Angriff auf das SPN Netzwerk erheblich. 

Ausführung des Programmes: cargo run -- [Klartexte Path] [Kryptotexte Path] [Output Path]

Mit der Funktion create_n_pairs(n: &i32, keys: &Vec<u16>) können ebenso beliebig viele zufällige Klartexte und ihre zugehörigen Kryptotexte erzeugt werden. 