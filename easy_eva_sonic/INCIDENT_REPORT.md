# Incident Report: False Positive PID 15006
**Datum:** 2025-12-31
**Vorfall:** Agent terminierte legitimen sleep Prozess.
**Ursache:** Zu aggressive Konfidenz-Logik (90%) und fehlende Whitelist.
**Lösung:** Update auf v10.8.11. Implementierung einer CRITICAL_WHITELIST und Erhöhung des Kill-Thresholds auf 95%.
**Status:** Gelöst & Sicher.
