# Entwicklerdokumentation

1. **Übersicht und Architektur:**
   - Allgemeine Beschreibung des Projekts, seiner Ziele und der zugrunde liegenden Architektur.
   - Diagramme oder Grafiken, die die Gesamtstruktur des Systems illustrieren.

2. **Installation und Einrichtung:**
   - Schritt-für-Schritt-Anleitung zur lokalen Entwicklungsumgebung und zur Einrichtung des Projekts.
   - Abhängigkeiten und Bibliotheken, die für die Entwicklung benötigt werden.

3. **Coding Guidelines und Best Practices:**
   - Festgelegte Codierungsrichtlinien und -standards für das Projekt.
   - Best Practices für Design, Entwicklung, und Dokumentation.

4. **API-Dokumentation:**
   - Ausführliche Dokumentation aller APIs, einschließlich Endpunkte, Parameter, Rückgabewerte und Beispiele.
   - Authentifizierungs- und Autorisierungsanleitungen für API-Zugriff.

5. **Datenbankmodell und Schema:**
   - Beschreibung des Datenbankmodells und der Tabellenschemata.
   - Beispiele für SQL-Abfragen und Datenbankoperationen.

6. **Code-Struktur und -Organisation:**
   - Erklärung der Code-Struktur, Verzeichnisse und Module.
   - Beschreibung der Verantwortlichkeiten verschiedener Teile des Codes.

7. **Funktionale und Nicht-funktionale Anforderungen:**
   - Liste der funktionalen Anforderungen und deren Implementierung.
   - Nicht-funktionale Anforderungen wie Leistung, Sicherheit und Skalierbarkeit.

8. **Entwicklungs- und Testumgebung:**
   - Konfiguration der Entwicklungsumgebung und Tools.
   - Anleitung zur Ausführung von Tests und zur Verwendung von Testdaten.

9. **Debugging und Fehlerbehebung:**
   - Methoden zum Debuggen von Code und Lösungsansätze für häufige Probleme.
   - Protokollierung und Fehlermeldungen.

10. **Beitragsrichtlinien und Collaboration:**
    - Informationen für Entwickler, die zum Projekt beitragen möchten (Pull Request Guidelines, Coding Standards).
    - Prozess für Code-Rezensionen und Zusammenarbeit im Team.

11. **Aktualisierungen und Versionskontrolle:**
    - Richtlinien zur Versionsnummerierung und zum Veröffentlichungsprozess.
    - Informationen zur Verwendung von Versionskontrollsystemen (z. B. Git).

12. **Externe Ressourcen und Bibliotheken:**
      - **askama**, um HTML-Templates mit Rust zu generieren
         - **askama_axum**, zur Integration von askama mit dem axum Framework
      - **axum**, als Framework für Webrequests
      - **chart.js**, eine Bibliothek zur Darstellung von Graphen mit JavaScript
      - **dotenv**, zur Verwendung von Umgebungsvariablen
      - **eyre**, für individuelle Fehlermeldungen
      - **Normalize.css**, um Abhängigkeiten vn Browserstylings so weit wie möglich aufzuheben
      - **reqwest**, als HTTP-Client
      - **serde**, zu serialiseren und deserialisieren von Daten
         - **serde-env**, um Umgebungsvariablen zu serialiseren und deserialisieren
         - **serde_json**, um Daten im JSON-Format zu serialiseren und deserialisieren
       - **sqlx**, zur Anbindung und Verwendung der PostgreSQL-Datenbank
       - **thiserror**, um eigene Fehlermeldungen zu bauen, welche dem Rust-Fehler-Datentypen entsprechen 
       - **time**, als Zeitenbibliothek
       - **tokio**, für eine Event-getriebene Laufzeit
       - **tower**, für Komponenten zum Aufbau eines Servers, wie timeouts, ratelimiting und loadbalancing
       -  **turf**, eine Bibliothek die es ermöglicht CSS und SCSS während der Compilezeit zu erzeugen und in die entstehende Binary einzupflegen


15. **Sicherheitsrichtlinien:**
    - Sicherheitsüberlegungen und bewährte Verfahren im Code.
    - Verwendung von Verschlüsselung, Validierung und Schutz vor bekannten Sicherheitslücken.

16. **Leistungs- und Skalierbarkeitsaspekte:**
    - Anleitung zur Leistungsüberwachung und Optimierung.
    - Skalierbarkeitsrichtlinien und bewährte Verfahren.

17. **Lizenzinformationen:**
    - Klare Angaben zur Lizenzierung des Codes und der verwendeten Bibliotheken.
