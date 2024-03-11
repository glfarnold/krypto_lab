Implementierung der AES Ver- und Entschlüsselung 

Ausführen des Programmes: 
cargo run -- [Betriebsmodus] [input path] [key path] [output path] mode [IV path (optional)]

Dabei ist der Betriebsmodus einer der 4 implementierten Modi: ECB / CBC / OFB / CTR.
mode ist ein bool, wobei bei true verschlüsselt und bei false entschlüsselt wird.

In den Dateien sollen Klartext, Key und optional auch der Initialisierungsvektor byteweise als Hexadezimalzahlen und ohne Zeilenumbrüche angegeben werden (wie die Beispieltexte auf Moodle).

Die Funktionen in dem Programm wurden mit Testvektoren von 
https://nvlpubs.nist.gov/nistpubs/fips/nist.fips.197.pdf
getestet.