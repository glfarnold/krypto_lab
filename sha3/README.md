Implementierung des SHA3-224 Hashing Algorithmus 

Ausführung des Programms: cargo run -- [input path] [output path]

Die Rundenfunktionen wurden nach https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.202.pdf programmiert. 
Beim Testen mit https://csrc.nist.gov/CSRC/media/Projects/Cryptographic-Standards-and-Guidelines/documents/examples/SHA3-224_Msg5.pdf fiel auf, dass die Theta Funktion nicht das erwartete Ergebnis liefert und somit ein falscher Hashwert herauskommt.