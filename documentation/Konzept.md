# Konzept
## Einleitung
### Projektbeschreibung
Dieses Projekt entsteht im Rahmen der Ausbildung zum Fachinformatiker Anwendungsentwicklung am Oberstufenzentrum Informations- und Medizintechnik in Berlin. Es soll in einer Gruppenarbeit über 5 Wochen eine Software entwickelt werden. Die Aufgabe ist die Entwicklung einer Software zum Extrahieren von Energiedaten der Website [Agora](https://www.agora-energiewende.de/service/agorameter/) und Überführung in eine Datenbank.

### Projektziel
Die herausfordernde Aufgabe beinhaltet die Automatisierung der Datenerfassung, -verarbeitung und -speicherung mit einem klaren Fokus auf die Auswahl und Verarbeitung von Energiedaten für einen bestimmten Zeitraum.

### Projektschnittstellen
> Personelle Schnittstellen & Organisatorische Schnittstellen

Da dieses Projekt im Rahmen der Ausbildung in der Berufsschule bearbeitet und abgegeben wird, gibt es keine konkreten Benutzer und Zielgruppen. Weil der Arbeitsauftrag vom Lehrer kommt, könnte die Lehrerschaft des Oberstufenzentrums für Informations- und Medizintechnik als Zielgruppe und Benutzer betrachtet werden. Das Ergebnis muss abschließend Herrn Tenbusch vorgstellt werden.

In diesem Projekt haben wir klare Zuständigkeiten verteilt, um effizient zusammenzuarbeiten. **Denis Wollgramm** kümmert sich um das Backend, wobei seine Expertise und Fähigkeiten darauf ausgerichtet sind, die technische Grundlage unseres Projekts zu gestalten und zu optimieren. **Lucas Brie** hingegen fokussiert sich auf das Frontend, und seine Aufgaben umfassen die Gestaltung und Implementierung der Benutzeroberfläche, um eine ansprechende und benutzerfreundliche Erfahrung zu gewährleisten. Schließlich ist **Hanna Krusch** für die Dokumentation verantwortlich und sorgt dafür, dass alle Aspekte des Projekts transparent und nachvollziehbar dokumentiert sind, um eine reibungslose Kommunikation und Wartung zu ermöglichen.

> Technische Schnittstellen

In unserem Entwicklungsprojekt nimmt die Kooperation mit spezifischen technischen Schnittstellen eine Schlüsselrolle ein. Insbesondere handelt es sich um das Datenbank-Management-System (DBMS) und die Agora-Website.

**Datenbank-Management-System (DBMS):**
Das DBMS fungiert als essenzieller Verwalter unserer Daten. Es ermöglicht die effiziente Organisation, Speicherung und Retrieval von Informationen. Die sorgfältige Auswahl und Integration dieses Systems gewährleistet eine robuste Grundlage für die langfristige Datenspeicherung und -verwaltung.

**Agora-Website:**
Die technische Schnittstelle zur Agora-Website ist von zentraler Bedeutung für die automatisierte Datenerfassung. Hierbei werden die benötigten Energiedaten der Website [Agora](https://www.agora-energiewende.de/service/agorameter/) mit der Agora-API abgegriffen und verwendet. Diese Schnittstelle agiert als Bindeglied zwischen unserer Software und dem umfassenden Informationsangebot der Agora-Plattform. Kontinuierliche Überwachung und Anpassung sind erforderlich, um etwaige Veränderungen auf der Agora-Website zeitnah zu berücksichtigen.

### Projektabgrenzung
> Beschreibt was das Projekt nicht machen soll

Die analytische Funktionen zur Interpretation der Energiedaten, wie Trendanalysen oder Vorhersagen, sind nicht Teil dieses Projekts. Die Software konzentriert sich auf die Datenerfassung und -speicherung. Außerdem ist es nur für schulische Zwecke konzipiert und wird nicht für kommerzielle Zwecke oder breite öffentliche Nutzung entwickelt. Abschließend ist nicht vorgesehen, dass die Anwendung eine Schnittstelle mit anderen externen Systemen herstellt. Die Schnittstellen beschränken sich lediglich auf die Agora-Website und das DBMS.

## Projektplanung
### Meilensteine
> Detaillierte Zeitpläne, um den Fortschritt zu verfolgen.

| Woche  | Meilenstein
| --- | --- |
| 1  | Projektauswahl  |
| 1  | Organisation des Projekts  |
| 1  | Benötigtes Wissen aneignen  |
| 2  | Projektgrundlagen erstellen  |
| 2  | Agora-API sichten und den Key abgleichen  |
| 3  | Frontend-Navigationsleiste fertigstellen  |
| 3  | Backend-Datenbank fertigstellen und Abfragen ermöglichen  |

### Abweichungen von der Projektaufgabenstellung

### Ressourcenplan
> Planung der benötigten Ressourcen (Hard-/Software, Räumlichkeiten, ...)

### Entwicklungsprozess
> Welches Modell wurde bei der Bearbeitung verfolgt?

Scrum

## Analysephase
### Ist-Analyse

### Wirtschaftlichkeitsanalyse
#### Kosten:
**Personalkosten:** Die Hauptinvestition entfällt auf die Arbeitszeit der Teammitglieder (Backend-Entwicklung, Frontend-Entwicklung, Dokumentation) entsprechend ihrer Ausbildungszeit und Vergütung.

**Werkzeuge und Technologien:** Kosten für die Nutzung von Entwicklungswerkzeugen, Datenbank-Management-Systemen und anderen erforderlichen Software-Ressourcen.

**Schulressourcen:** Einbeziehung von Schulinfrastruktur wie Computerlabors und Netzwerkinfrastruktur für die Entwicklung und Umsetzung des Projekts.

#### Nutzen:
**Ausbildungsfortschritt:** Das Projekt bietet den Teammitgliedern, insbesondere im Kontext ihrer Fachinformatiker-Ausbildung, die Möglichkeit, praxisnahe Erfahrungen in der Softwareentwicklung zu sammeln.

**Projektergebnis:** Die entwickelte Software ermöglicht eine automatisierte Extraktion und Speicherung von Energiedaten, was nicht nur für schulische Projekte, sondern auch als Grundlage für zukünftige Anwendungen von Wert sein kann.

**Kompetenzerweiterung:** Die Teammitglieder erweitern ihre technischen Kompetenzen in den Bereichen Backend-Entwicklung, Frontend-Entwicklung und Dokumentation.

#### Return on Investment (ROI):
Die Investition in das Projekt zielt primär darauf ab, den Ausbildungsfortschritt und die Kompetenzerweiterung der Teammitglieder zu fördern. Der erwartete Return on Investment liegt vor allem in der verbesserten Qualifikation und den erworbenen praktischen Fähigkeiten.

#### Schlussfolgerung:
Die Wirtschaftlichkeitsanalyse zeigt, dass die Investitionen in erster Linie auf die Ausbildung der Teammitglieder abzielen. Die erwarteten Nutzen liegen in der Entwicklung von Fähigkeiten, dem pädagogischen Wert des Projekts und der potenziellen Anwendbarkeit der entwickelten Lösung für zukünftige Szenarien.
### Qualitätsanforderungen
### Anforderungen
| Nr  | Kriterium | Beschreibung | Punkte
| --- | --- | --- | --- |
| 1  | ER-Modell und Relationales Modell für DBS erstellen  | In einem Datensatz sollen jeweils die Energiedaten von einer Stunde gespeichert werden, gut auswertbares Datumsformat  | 10  |
| 2  | User Storys  | Erstellen Sie ein Anwendungsfalldiagramm und beschreiben Sie die daraus resultierenden User Stories.  | 10  |
| 3  | Auslesen der relevanten Daten aus der Webseite  | Ungefilterte Wiedergabe auf der Konsole  | 5  |
| 4  | Anzeigen der aktuellen Stromerzeugung  | z.B.: Strukturierte Ausgabe von Daten  | 10  |
| 5  | Anzeigen aller Datensätze des über die Webseite ausgewählten Zeitraums  | z.B.: Strukturierte Ausgabe von Daten  | 10  |
| 6  | Erstellen einer GUI mit Auswahl eines Zeitraums  | Kann auch erstmal ohne Funktion sein, das Datum soll über einen Kalender ausgewählt werden (Bibliothek nutzen)  | 5  |
| 7  | Datenbankanbindung  | Über eine Methode kann ein Datensatz in die Datenbank eingetragen werden. Testklasse mit Beispieldaten verwenden.  | 5  |
| 8  | Alle Daten in DBS speichern  | Es existiert ein Button, der die Daten des gesamten Zeitraums von der Webseite holt und in das DBS einspeichert.  | 10  |
| 9  | Installationsdokumentation  | Information an die Lehrkraft, wie das Programm installiert, konfiguriert und gestartet wird.  | 10  |
| 10  | _Content Cell_  | _Content Cell_  | _Content Cell_  |
| S1  | Nachträgliches Hinzufügen von Daten  | Es werden die neusten Daten von der Webseite geholt und mit den Daten im DBS vereint. Es soll nichts altes gelöscht werden.  | 10  |
| S2  | Die GUI ist optisch ansprechend  | _Content Cell_  | 5  |
| S3  | _Content Cell_  | _Content Cell_  | _Content Cell_  |
| K1  | Entwicklerdokumentation  | Sie beschreiben Ihr Vorgehen, Techniken, Bibliotheken, so dass jemand anderes ihre Software warten kann.  | 10  |
| K2  | _Content Cell_  | _Content Cell_  | _Content Cell_  |

## Anforderungsdokumentation
### Funktions- und technische Spezifikationen 
> Detaillierte Beschreibung der Anforderungen und Funktionalitäten des Systems.

## Designdokumente
### Technologiestack
> Dokumentation die für die Durchführung des Projekts benutzt worden sind.

### Datenbankdesign 
> Struktur der Datenbank, Tabellen, Beziehungen, etc.

### Benutzeroberflächendesign 
> Entwürfe und Spezifikationen für die Benutzeroberfläche.

Wir verwenden Normalize.css, um browserübergreifende Konsistenz in der Darstellung von HTML-Elementen zu gewährleisten. Es bietet den Vorteil, standardisierte und vorhersagbare Stilgrundlagen zu schaffen, ohne dabei unnötige oder problematische Zurücksetzungen vorzunehmen. Durch die gezielte Normalisierung werden konsistente Designs über verschiedene Browser hinweg ermöglicht, wodurch Entwickler weniger Zeit mit der Bewältigung von Browserinkonsistenzen verbringen und sich stattdessen auf die Gestaltung benutzerfreundlicher Benutzeroberflächen konzentrieren können.

## 4. Testdokumentation
### Testplan
> Übersicht über den Testprozess, Testarten und Testumfang.

### Testskripte
> Detaillierte Anweisungen für die Durchführung von Tests.

### Testergebnisse 
> Dokumentation der Ergebnisse, einschließlich Fehlerberichte und behobener Fehler.
