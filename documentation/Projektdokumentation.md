# Konzept
1. [Einleitung](#einleitung)
   - [Projektbeschreibung](#projektbeschreibung)
   - [Projektziel](#projektziel)
   - [Projektschnittstellen](#projektschnittstellen)
   - [Projektabgrenzung](#projektabgrenzung)
2. [Projektplanung](#projektplanung)
   - [Meilensteine](#meilensteine)
   - [Abweichungen von der Projektaufgabenstellung](#abweichungen-von-der-projektaufgabenstellung)
   - [Ressourcenplan](#ressourcenplan)
   - [Entwicklungsprozess](#entwicklungsprozess)
3. [Analysephase](#analysephase)
   - [Ist-Analyse](#ist-analyse)
   - [Wirtschaftlichkeitsanalyse](#wirtschaftlichkeitsanalyse)
   - [Qualitätsanforderungen](#qualitätsanforderungen)
   - [Anforderungen](#anforderungen)
4. [Entwurfsphase](#entwurfsphase)
   - [Zielplattform](#zielplattform)
   - [Benutzeroberflächendesign](#benutzeroberflächendesign)
   - [Datenmodell](#datenmodell)
   - [Maßnahmen zur Qualitätssicherung](#maßnahmen-zur-qualitätssicherung)
5. [Implementierungsphase](#implementierungsphase)
   - [Datenstruktur](#datenstruktur)
   - [Benutzeroberfläche](#benutzeroberfläche)
6. [Testdokumentation](#testdokumentation)
7. [Fazit](#fazit)
   - [Soll-/Ist-Vergleich](#soll-ist-vergleich)
   - [Lessons Learned](#lessons-learned)

## Einleitung
### Projektbeschreibung
Dieses Projekt entsteht im Rahmen der Ausbildung zum Fachinformatiker Anwendungsentwicklung am Oberstufenzentrum Informations- und Medizintechnik in Berlin. Es soll in einer Gruppenarbeit über 5 Wochen eine Software entwickelt werden. Die Aufgabe ist die Entwicklung einer Software zum Extrahieren von Energiedaten der Website [Agora](https://www.agora-energiewende.de/service/agorameter/) und Überführung in eine Datenbank.

### Projektziel
Die herausfordernde Aufgabe beinhaltet die Automatisierung der Datenerfassung, -verarbeitung und -speicherung mit einem klaren Fokus auf die Auswahl und Verarbeitung von Energiedaten für einen bestimmten Zeitraum.

### Projektschnittstellen

Da dieses Projekt im Rahmen der Ausbildung in der Berufsschule bearbeitet und abgegeben wird, gibt es keine konkreten Benutzer und Zielgruppen. Weil der Arbeitsauftrag vom Lehrer kommt, könnte die Lehrerschaft des Oberstufenzentrums für Informations- und Medizintechnik als Zielgruppe und Benutzer betrachtet werden. Das Ergebnis muss abschließend Herrn Tenbusch vorgstellt werden.

In diesem Projekt haben wir klare Zuständigkeiten verteilt, um effizient zusammenzuarbeiten. **Denis Wollgramm** kümmert sich um das Backend, wobei seine Expertise und Fähigkeiten darauf ausgerichtet sind, die technische Grundlage unseres Projekts zu gestalten und zu optimieren. **Lucas Brie** hingegen fokussiert sich auf das Frontend, und seine Aufgaben umfassen die Gestaltung und Implementierung der Benutzeroberfläche, um eine ansprechende und benutzerfreundliche Erfahrung zu gewährleisten. Schließlich ist **Hanna Krusch** für die Dokumentation verantwortlich und sorgt dafür, dass alle Aspekte des Projekts transparent und nachvollziehbar dokumentiert sind, um eine reibungslose Kommunikation und Wartung zu ermöglichen.

In unserem Entwicklungsprojekt nimmt die Kooperation mit spezifischen technischen Schnittstellen eine Schlüsselrolle ein. Insbesondere handelt es sich um das Datenbank-Management-System (DBMS) und die Agora-Website.

**Datenbank-Management-System (DBMS):**
Das DBMS fungiert als essenzieller Verwalter unserer Daten. Es ermöglicht die effiziente Organisation, Speicherung und Retrieval von Informationen. Die sorgfältige Auswahl und Integration dieses Systems gewährleistet eine robuste Grundlage für die langfristige Datenspeicherung und -verwaltung.

**Agora-Website:**
Die technische Schnittstelle zur Agora-Website ist von zentraler Bedeutung für die automatisierte Datenerfassung. Hierbei werden die benötigten Energiedaten der Website [Agora](https://www.agora-energiewende.de/service/agorameter/) mit der Agora-API abgegriffen und verwendet. Diese Schnittstelle agiert als Bindeglied zwischen unserer Software und dem umfassenden Informationsangebot der Agora-Plattform. Kontinuierliche Überwachung und Anpassung sind erforderlich, um etwaige Veränderungen auf der Agora-Website zeitnah zu berücksichtigen.

### Projektabgrenzung

Die analytische Funktionen zur Interpretation der Energiedaten, wie Trendanalysen oder Vorhersagen, sind nicht Teil dieses Projekts. Die Software konzentriert sich auf die Datenerfassung und -speicherung. Außerdem ist es nur für schulische Zwecke konzipiert und wird nicht für kommerzielle Zwecke oder breite öffentliche Nutzung entwickelt. Abschließend ist nicht vorgesehen, dass die Anwendung eine Schnittstelle mit anderen externen Systemen herstellt. Die Schnittstellen beschränken sich lediglich auf die Agora-Website und das DBMS.

---

## Projektplanung
### Meilensteine

| Woche  | Meilenstein
| --- | --- |
| 1  | Projektauswahl  |
| 1  | Organisation des Projekts  |
| 1  | Benötigtes Wissen aneignen  |
| 2  | Projektgrundlagen erstellen  |
| 2  | Agora-API sichten und den Key abgleichen  |
| 3  | Frontend-Navigationsleiste fertigstellen  |
| 3  | Backend-Datenbank fertigstellen und Abfragen ermöglichen  |
| 4  | Plotting Library auswählen und erste Daten darstellen  |
| 4  | Implementierung des Front-Ends  |
| 5  | Entwicklerdokumentation schreiben  |
| 5  | Installationsdokumentation schreiben  |
| 5  | Plot umstylen, auslagern und erweitern  |
| 5  | Favicon einbinden  |

### Abweichungen von der Projektaufgabenstellung
Da wir während er Bearbeitungszeit dieses Projektes keine erweiterte Aufgabenstellung bekommen haben können wir keine Abweichungen feststellen und dokumentieren.

### Ressourcenplan

Für die erfolgreiche Umsetzung unseres Projekts stehen verschiedene Personalressourcen zur Verfügung. Denis Wollgramm übernimmt die Rolle des Backend-Entwicklers und wird sich vollzeitlich auf die Implementierung der Datenbankanbindung und die Extraktion von Agora-Daten konzentrieren. Lucas Brie agiert als Frontend-Entwickler, ebenfalls in Vollzeit, und ist verantwortlich für das Design und die Umsetzung der Benutzeroberfläche. Hanna Krusch übernimmt die Position der Dokumentationsverantwortlichen und sorgt für die Erstellung von der Projektdokumentation.

Die Anlagenressourcen für unser Projekt umfassen primär die persönlichen Laptops der Teammitglieder, die für die individuelle Entwicklung genutzt werden. Diese Laptops bieten die notwendige Flexibilität und Unabhängigkeit für die Entwicklung von Backend und Frontend. Die Schulklassenräume dienen als Ort für gemeinsame Besprechungen und Präsentationen. Wir haben uns für die Nutzung der PostgreSQL-Datenbank entschieden, wobei unsere Daten in diesem System gespeichert werden. Des Weiteren erfolgt die Sicherung und Versionierung unseres Projektfortschritts über die Plattform GitHub.

Für die Umsetzung des Projekts stehen keine externen Finanzressourcen zur Verfügung. Die Finanzierung erfolgt ausschließlich durch schuleigene Mittel. Die Kosten für die Nutzung der Schulcomputerlabore, den Stromverbrauch und das WLAN sind bereits durch diese Mittel abgedeckt.

Die zeitlichen Ressourcen werden entsprechend der Arbeitszeiten der Teammitglieder verteilt. Vollzeitliche Engagement während der 5-wöchigen Projektlaufzeit ist für Backend- und Frontend-Entwickler vorgesehen, während die Dokumentationsverantwortliche ihre Aufgaben in Teilzeit über die gesamte Projektlaufzeit hinweg durchführt. Die Projektlaufzeit von 5 Wochen ist durch klare Meilensteine strukturiert, um den Fortschritt zu überwachen.

### Entwicklungsprozess

Unser Entwicklungsprozess basiert auf dem Scrum-Framework, einem agilen Ansatz, der sich besonders gut für die Zusammenarbeit in kleinen, flexiblen Teams eignet. Als Product Owner und Scrum Master fungiert Hanna Krusch, die maßgeblich für die erfolgreiche Umsetzung des Projekts verantwortlich ist. In ihrer Rolle als Product Owner definiert sie die Anforderungen und Prioritäten, während sie als Scrum Master sicherstellt, dass der Entwicklungsprozess reibungslos verläuft und Hindernisse beseitigt werden.

Die Entwicklungsaufgaben sind klar auf die Teammitglieder verteilt. Denis Wollgramm konzentriert sich als Backend-Entwickler darauf, die technische Grundlage zu schaffen, insbesondere die Anbindung an die Datenbank und die Extraktion von Agora-Daten. Lucas Brie übernimmt die Frontend-Entwicklung, wobei er für das Design und die Benutzeroberfläche verantwortlich ist. Durch diese klare Aufgabenverteilung wird eine effiziente Zusammenarbeit und Spezialisierung ermöglicht.

Die regelmäßigen Sprint-Meetings dienen dazu, den Fortschritt zu überprüfen, Herausforderungen zu besprechen und Anpassungen vorzunehmen. Diese iterative Vorgehensweise ermöglicht es uns, flexibel auf Veränderungen oder Anpassungen der Anforderungen zu reagieren. Die Kommunikation im Team wird durch die klaren Zuständigkeiten und die regelmäßigen Meetings erleichtert.

---

## Analysephase
### Ist-Analyse
Es wurde der Prozess der Datenerfassung auf der Agora-Website untersucht, um zu verstehen, wie die Daten bisher manuell gesammelt und verarbeitet wurden. Es wurde ermittelt, dass dieser manuelle Prozess zeitaufwändig und fehleranfällig ist, was die Notwendigkeit einer Automatisierung deutlich machte. 
Die Anforderungen und Erwartungen der Stakeholder ergaben, dass die Lehrer des Oberstufenzentrums und potenzielle Benutzer der Software, eine effiziente Automatisierung der Datenerfassung und -verarbeitung sowie eine benutzerfreundliche Benutzeroberfläche benötigen, um die Daten und Ergebnisse selbst noch weiter interpretieren und verwenden.

### Wirtschaftlichkeitsanalyse
#### Kosten:
**Personalkosten:** Die Hauptinvestition entfällt auf die Arbeitszeit der Teammitglieder (Backend-Entwicklung, Frontend-Entwicklung, Dokumentation) entsprechend ihrer Ausbildungszeit und Vergütung.

**Werkzeuge und Technologien:** Kosten für die Nutzung von Entwicklungswerkzeugen, Datenbank-Management-Systemen und anderen erforderlichen Software-Ressourcen.

**Schulressourcen:** Einbeziehung von Schulinfrastruktur wie Schulräume und W-LAN für die Entwicklung und Umsetzung des Projekts.

#### Nutzen:
**Ausbildungsfortschritt:** Das Projekt bietet den Teammitgliedern, insbesondere im Kontext ihrer Fachinformatiker-Ausbildung, die Möglichkeit, praxisnahe Erfahrungen in der Softwareentwicklung zu sammeln.

**Projektergebnis:** Die entwickelte Software ermöglicht eine automatisierte Extraktion und Speicherung von Energiedaten, was nicht nur für schulische Projekte, sondern auch als Grundlage für zukünftige Anwendungen von Wert sein kann.

**Kompetenzerweiterung:** Die Teammitglieder erweitern ihre technischen Kompetenzen in den Bereichen Backend-Entwicklung, Frontend-Entwicklung und Dokumentation.

#### Return on Investment (ROI):
Die Investition in das Projekt zielt primär darauf ab, den Ausbildungsfortschritt und die Kompetenzerweiterung der Teammitglieder zu fördern. Der erwartete Return on Investment liegt vor allem in der verbesserten Qualifikation und den erworbenen praktischen Fähigkeiten.

#### Schlussfolgerung:
Die Wirtschaftlichkeitsanalyse zeigt, dass die Investitionen in erster Linie auf die Ausbildung der Teammitglieder abzielen. Die erwarteten Nutzen liegen in der Entwicklung von Fähigkeiten, dem pädagogischen Wert des Projekts und der potenziellen Anwendbarkeit der entwickelten Lösung für zukünftige Szenarien.

### Qualitätsanforderungen

Unsere Software strebt hohe Qualitätsstandards an, insbesondere in den Bereichen Performance, Usability, Effizienz und Sicherheit. Die Anwendung soll reaktionsschnell sein, eine benutzerfreundliche Oberfläche bieten und effizient mit der Datenbank interagieren. Stabilität und Sicherheit haben höchste Priorität, einschließlich der sicheren Verwahrung sensibler Daten. Die technische Umsetzung folgt Best Practices, um konsistente Datenhaltung zu gewährleisten. Klare Dokumentation unterstützt die Wartbarkeit und Weiterentwicklung der Software. Insgesamt streben wir nach einer qualitativ hochwertigen Lösung, die den Anforderungen unserer Ausbildung und darüber hinaus gerecht wird.

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
| 10  | Mehrere Datensätzen sind auswählbar  | Es werden mehr als nur ein Datensatz zur Auswahl zur Verfügung stehen  | 0  |
| S1  | Nachträgliches Hinzufügen von Daten  | Es werden die neusten Daten von der Webseite geholt und mit den Daten im DBS vereint. Es soll nichts altes gelöscht werden.  | 10  |
| S2  | Die GUI ist optisch ansprechend  | DIe GUI soll optisch ansprechend sein für die User  | 5  |
| S3  | Logarithmische Skala zum Anzeigen der Daten  | Damit die Anzeige nicht zu groß wird, soll die Skala logarithmisch dargestellt werden  | 0  |
| K1  | Entwicklerdokumentation  | Sie beschreiben Ihr Vorgehen, Techniken, Bibliotheken, so dass jemand anderes ihre Software warten kann.  | 10  |
| K2  | about Page  | EIne Seite in der kurz beschrieben wird worum es auf der Seite geht und wieso sie existiert  | 0  |

---

## Entwurfsphase
### Zielplattform 
Rust als Typensichere, kompilierte und sehr schnelle Sprachen, bietet von Haus aus viele Vorteile und war für uns spannend zur Verwendung in solch einem Projekt um Erfahrung zu sammeln

### Benutzeroberflächendesign 
Wir haben eine lokal gehostete Webanwendung gewählt, in der Annahme dass dies die größte Menge an möglichen Usecases abdeckt ohne dass wir verschiedene Umgebungen, Betriebssysteme etc. selbst handhaben müssen.

Ein simples Webseitendesign mit einer Navigationsleiste oben und dem Inhalt der jeweiligen Seite darunter, aus Gewohnheitsgründen sollte dieser Aufbau für einen generischen Nutzer einfach verständlich und verwendbar sein.
Die Farbwahl erfolgte anhand eigener Präferenzen und ist ganz modern in einem ewigen Dark Mode um weniger Anstrengend für die Augen des Nutzers zu sein 
Die Farbwahl wurde unter Zuhilfenahme von Googles Material3 Farbpicker getroffen.
Simplizität der Oberflächen mit runden Ecken sollte modernen Standards entsprechen

### Datenmodell
> Entwurf/Beschreibung der Datenstrukturen (z.B. ERM und/oder Tabellenmodell, XML-Schemas) mit kurzer Beschreibung der wichtigsten (!) verwendeten Entitäten.
### Maßnahmen zur Qualitätssicherung
>  Welche Maßnahmen werden ergriffen, um die Qualität des Projektergebnisses (siehe Kapitel 3.5) zu sichern (z.B. automatische Tests, Anwendertests)?

> Ggfs. Definition von Testfällen und deren Durchführung (durch Programme/Benutzer).

---

## Implementierungsphase
### Datenstruktur
> Beschreibung der angelegten Datenbank (z.B. Generierung von SQL aus Modellierungswerkzeug oder händisches Anlegen), XML-Schemas usw.

### Benutzeroberfläche
> Beschreibung der Implementierung der Benutzeroberfläche, falls dies separat zur Implementierung der Geschäftslogik erfolgt (z.B. bei HTML-Oberflächen und Stylesheets).
> Ggfs. Beschreibung des Corporate Designs und dessen Umsetzung in der Anwendung.
> Screenshots der Anwendung (TODO: Montag)

Allgemein haben wir versucht uns bei dem Design an der aktuellen Version Material3 von Googles Material Design zu orientieren. Dementsprechend verwenden wir viele Rundungen und flach wirkende Oberflächen. Auch unsere Farbpalette wurde mit Hilfe des m3 Material Theme Builders aufgebaut, als Grundlage wurde Aufgrund persönlicher Präferenz ein Lilaton gewählt.

Wir verwenden Normalize.css, um browserübergreifende Konsistenz in der Darstellung von HTML-Elementen zu gewährleisten. Es bietet den Vorteil, standardisierte und vorhersagbare Stilgrundlagen zu schaffen, ohne dabei unnötige oder problematische Zurücksetzungen vorzunehmen. Durch die gezielte Normalisierung werden konsistente Designs über verschiedene Browser hinweg ermöglicht, wodurch Entwickler weniger Zeit mit der Bewältigung von Browserinkonsistenzen verbringen und sich stattdessen auf die Gestaltung benutzerfreundlicher Benutzeroberflächen konzentrieren können.

Architektonisch gibt es einen Router welcher anhand der angefragten Routen verschiedene HTML-Templates rendert. An die HTML-Templates können Datensätze übergeben werden, welche dann inline mit Rustcode verarbeitet werden können, dabei bietet die Bibliothek askama auch die Möglichkeit dynamisch aus Daten HTML generieren zu lassen. 
Herz dieses Projektes ist der Graph auf welchem die vom Backend zur Verfügung gestellten Daten dargestellt werden. 
Da Rust, als relativ junge Programmiersprache, aktuell noch keine Bibliotheken hat welche es an Umfang und Funktionalität mit chart.js aufnehmen können, haben wir uns dafür entschieden chart.js per Script in unser Template für die Darstellung des Graphen einzubinden.
Dabei werden unsere Daten über die serde-Bibliothek zu JSON serialisiert, welches dann an das Script weitergereicht wird. 
Der Graph selbst wird auch über JSON gesteuert, es wurde beispielsweise konfiguriert dass die y-Achse logarithmisch dargestellt werden soll da durch große Diskrepanzen in den Werten der verschiedenen Daten sonst ein extrem unleserlicher Graph entstehen würde.
Mit etwas JavaScript-Stringformatierung können auch eigene Labels für die Datensätze generiert werden. Dies ist notwendig da wir beispielsweise zeitgleich die Menge der ausgestoßenen Emissionen in Tonnen CO2 und die produzierte Strommenge in MW/h auf dem selben Graphen darstellen können und bei einer einfachen Achsenbeschriftung dementsprechend Verwirrung aufkommen könnte.

---

## Testdokumentation
SQLx ist eine Rust-Bibliothek, die das Ausführen von SQL-Abfragen erleichtert und dabei Compile-Zeit-Typensicherheit bietet. Durch die Integration von SQL-Abfragen in den Rust-Code ermöglicht SQLx eine nahtlose Interaktion mit der Datenbank, wodurch viele Fehler, die in anderen Umgebungen auftreten könnten, vermieden werden. SQLx überprüft zur Compile-Zeit, ob die Daten in der Datenbank stehen können. Falls das nicht der Fall ist, wird ein Compile-Fehler geworfen.

Aufgrund dieser Eigenschaften von Rust und SQLx können Entwickler:innen in einigen Fällen davon ausgehen, dass ihr Code bereits eine gewisse Robustheit aufweist, ohne dass umfangreiche Testfälle geschrieben werden müssen. Insbesondere bei einfachen Anwendungen oder in Situationen, in denen die Funktionalität sehr klar und begrenzt ist, kann die Verwendung von Rust und SQLx dazu beitragen, den Testaufwand zu reduzieren.

Wir haben uns entschieden, auf umfangreiche Tests zu verzichten, da wir direkt im Anschluss an die Programmierung die Funktionalitäten unserer Anwendung getestet haben und festgestellt haben, dass sie alle Anforderungen erfüllen. Durch diese praxisnahe Herangehensweise konnten wir sicherstellen, dass unser Code den gewünschten Spezifikationen entspricht, ohne zusätzlichen Testaufwand zu betreiben. Unsere direkte Überprüfung nach dem Programmieren hat sich als effizienter Weg erwiesen, um sicherzustellen, dass unsere Anwendung zuverlässig und fehlerfrei funktioniert.

---

## Fazit
### Soll-/Ist-Vergleich
Der Soll-/Ist-Vergleich zeigt, dass die Ziele des Projekts erfolgreich erreicht wurden. Wir konnten ein vollständiges ER-Modell und relationales Modell für die Datenbank erstellen, User Stories definieren und die relevanten Daten von der Webseite extrahieren und anzeigen. Die GUI wurde mit einer Auswahlmöglichkeit für den Zeitraum erfolgreich implementiert, ebenso wie die Datenbankanbindung und die Speicherung aller Daten. Auch das nachträgliche Hinzufügen von Daten und die optisch ansprechende Gestaltung der GUI wurden erreicht.

### Lessons Learned
Durch das Projekt haben wir wichtige Erkenntnisse aus spezifischen Herausforderungen gewonnen. Die Nutzung einer fehlerhaften Library für das Graphendarstellen führte zu Komplikationen und Compilierungsfehlern in Woche 4. Dies verdeutlicht die Bedeutung der sorgfältigen Auswahl und Überprüfung von externen Bibliotheken, um potenzielle Probleme frühzeitig zu erkennen und zu vermeiden.

Des Weiteren haben wir festgestellt, dass das Rendern von Graphen über den Server zwar eine anfängliche Lösung war, aber nicht optimal in Bezug auf Dynamik und Geschwindigkeit. Durch die Umstellung auf eine clientseitige Darstellung der Graphen konnten wir eine deutlich verbesserte Leistung erzielen. Diese Erfahrung zeigt, wie wichtig es ist, verschiedene Ansätze zu prüfen und die Architektur der Anwendung kontinuierlich zu optimieren, um eine optimale Benutzererfahrung zu gewährleisten.

Außerdem haben wir gemerkt wie wichtig es ist, Funktionen nach ihrer Relevanz zu priorisieren und flexibel auf unvorhergesehene Änderungen zu reagieren. Klare Kommunikation, effektive Zusammenarbeit und gründliche Dokumentation sind entscheidend für den Erfolg eines Projekts. Zudem ist eine effektive Zeitplanung und -verwaltung sowie die Berücksichtigung der Benutzerbedürfnisse von großer Bedeutung für eine erfolgreiche Entwicklung.
