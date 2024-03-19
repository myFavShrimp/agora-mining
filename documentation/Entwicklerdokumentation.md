# Entwicklerdokumentation

## 1. **Übersicht und Architektur:**
   
   Dieses Projekt ist eine Serveranwendung und wurde in Rust umgesetzt. Das Frontend besteht aus Templates welche vom Server gerendert und zurückgegeben werden. Für Interaktivität wurde das htmx-Framework verwendet,       welches es ermöglich HTML-Komponenten dynamisch auszutauschen. Als Datenbank wurde das Open-Source-Datenbanksystem PostgreSQL verwendet. 
   
## 2. **Installation und Einrichtung:**
    - Rust installieren ([rust-lang.org](https://www.rust-lang.org/learn/get-started))
    - Docker installieren ([docker.com](https://www.docker.com/))
    - `.env`-Datei verlinken: `make link-env`
    - Datenbanksystem hochfahren: `docker compose up -d`
    - Datenbank erstellen und migrieren: `make init-db`
    - Serveranwendung kompilieren und starten: `cargo run`

## 3. **Datenbankmodell und Schema:**
   - Beschreibung des Datenbankmodells und der Tabellenschemata.
   - Beispiele für SQL-Abfragen und Datenbankoperationen.

## 4. **API-Dokumentation:**
   
   Es gibt keine API im klassischen Sinne, sondern wir haben einen Endpunkt (`/graph`) welcher Formdaten entgegennimmt und eine HTML-Response liefert.
   Der Endpunkt erwartet `used_data_sets`, eine Liste von ausgewählten Datensätzen (bspw. PowerGeneration und PowerEmission), ein `from`- und `to`-Datum, nach ISO-8601, und `use_average` welches die Werte `None`,          `Daily`, `Monthly`, `Yearly` annimmt. <br>
   Weiter gibt es die Routen `/about`, `/refresh` und  `/`.
   - `/about` führt zu einer Seite mit einer kurzen Beschreibung des Projektes und einem Verweis auf das Repository.
   - `/refresh` aktualisiert die Daten welche in der Datenbank hinterlegt sind
   - `/` ist die Landingpage
   
## 5. **Code-Struktur und -Organisation:**

   Es gibt insgesamt fünf Module, `database`, `agora`, `templates`, `plotting` und `config`.<br>
   Das Modul `database` enthält die Datenbankentitäten und die Querylogik.
   In `agora` ist die Logik für die API-Anfragen gekapselt.
   Das Modul `templates` enthält die HTML-Templates welche gerendert werden können, sowie das Untermodul `plotting`, zum umwandeln der Datenbankdaten in Graphdaten.
   Und das letzte Modul ist `config`, in welchem die Einstellungen des Servers definiert sind.


## 6. **Externe Ressourcen und Bibliotheken:**

Rustbibliotheken:

- **askama**, um HTML-Templates mit Rust zu generieren
    - **askama_axum**, zur Integration von askama mit dem axum Framework
- **axum**, als Framework für Webrequests
    - **axum-extra** für die Deserialisierung von Form-Daten in Request-URLs
- **dotenv**, zur Verwendung von Umgebungsvariablen aus einer `.env`-Datei
- **eyre**, für bessere Fehlermeldungen beim Initialisieren des Servers
- **reqwest**, als HTTP-Client
- **serde**, zum serialiseren und deserialisieren von Daten
    - **serde-env**, um Umgebungsvariablen zu serialiseren und deserialisieren
    - **serde_json**, um Daten im JSON-Format zu serialiseren und deserialisieren
- **sqlx**, zur Anbindung und Verwendung der PostgreSQL-Datenbank
- **thiserror**, um eigene Fehlermeldungen zu bauen, welche dem Rust-Fehler-Datentypen entsprechen 
- **time**, als Zeitenbibliothek
- **tokio**, für eine Asynchrone-Laufzeit
- **tower**, für Komponenten zum Aufbau eines Servers, wie timeouts, ratelimiting und loadbalancing
- **turf**, eine Bibliothek die es ermöglicht CSS und SCSS während der Compilezeit zu erzeugen und in die entstehende Binary einzupflegen

Javascriptbibliotheken:
   - **chart.js**, eine Bibliothek zur Darstellung von Graphen mit JavaScript
   - **htmx**, ein Framework welches es ermöglich Requests direkt aus HTML-Elementen zu feuern und diese dynamisch auszutauschen

CSS-Bibliotheken:
   - **Normalize.css**, um Abhängigkeiten von Browserstylings so weit wie möglich aufzuheben
