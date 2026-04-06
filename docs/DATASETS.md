# FCLC — Clinical Datasets for Testing and Validation

**Status:** April 2026 · CONCEPT v6.0

---

## Рекомендованные датасеты (по приоритету)

### 1. Synthea — Synthetic Patient Generator ⭐ ЛУЧШИЙ СТАРТ

| Параметр | Значение |
|----------|----------|
| Тип | Синтетические данные (полная OMOP CDM совместимость) |
| Доступность | Open source, без ограничений, IRB не требуется |
| Размер | Настраивается: 100 до 10M пациентов |
| OMOP совместимость | Нативная (официальный OMOP converter входит в комплект) |
| Применение для FCLC | 3 виртуальные клиники по 1000 пациентов — идеально для пилота |

**Установка и генерация:**
```bash
# Java 11+ требуется
git clone https://github.com/synthetichealth/synthea.git
cd synthea
./run_synthea -p 1000 Georgia    # 1000 пациентов, локация Georgia
# Output: output/csv/ — формат совместимый с OMOP

# Для 3 виртуальных клиник:
./run_synthea -p 1000 --seed 1001 > clinic_node1.csv
./run_synthea -p 1000 --seed 1002 > clinic_node2.csv
./run_synthea -p 1000 --seed 1003 > clinic_node3.csv
```

**Конвертация в FCLC-формат:**
```bash
python3 scripts/synthea_to_fclc.py output/csv/ fclc_data/clinic1.csv
```
(скрипт: `scripts/synthea_to_fclc.py` — см. ниже)

**Маппинг Synthea → OMOP → FCLC OmopRecord:**
| Synthea поле | OMOP CDM | FCLC OmopRecord |
|--------------|----------|-----------------|
| birthdate | person.birth_datetime | age_group (bin) |
| gender | person.gender_concept_id | sex |
| conditions.DESCRIPTION | condition_occurrence | diabetes_diagnosis_year |
| observations[HbA1c] | measurement | hba1c_last |
| observations[BMI] | measurement | bmi |
| conditions[nephropathy] | condition_occurrence | has_nephropathy |
| conditions[retinopathy] | condition_occurrence | has_retinopathy |
| encounters count | visit_occurrence | hospitalized_last_12m |

---

### 2. MIMIC-IV Demo (OMOP CDM version) ⭐⭐ ЛУЧШИЙ ДЛЯ ПУБЛИКАЦИИ

| Параметр | Значение |
|----------|----------|
| Тип | Реальные данные ICU (MIT Beth Israel Deaconess) |
| Доступность | Free с PhysioNet аккаунтом (CITI training ~4 часа) |
| Размер | Demo: 100 пациентов · Full: 382,278 пациентов |
| OMOP совместимость | Официальный OMOP CDM mapping от OHDSI |
| IRB | PhysioNet Data Use Agreement (автоматически) |

**Ссылка:**
- Demo (100 пациентов, без аккаунта): https://physionet.org/content/mimic-iv-demo-omop/0.9/
- Full: https://physionet.org/content/mimiciv/3.0/

**Загрузка Demo:**
```bash
wget -r -N -c -np https://physionet.org/files/mimic-iv-demo-omop/0.9/
# или
pip install wfdb
python3 -c "import wfdb; wfdb.dl_database('mimic-iv-demo-omop', './mimic-demo')"
```

**OMOP tables нужные для FCLC:**
- `person.csv` (демография)
- `condition_occurrence.csv` (диагнозы ICD-10)
- `measurement.csv` (HbA1c, BMI — LOINC коды)
- `visit_occurrence.csv` (госпитализации)

**Маппинг для FCLC:**
```sql
-- Запрос для извлечения FCLC-совместимых записей из MIMIC OMOP
SELECT
  p.person_id,
  FLOOR((EXTRACT(YEAR FROM v.visit_start_date) - p.year_of_birth) / 5) * 5 AS age_bin,
  p.gender_concept_id,
  MIN(c.condition_start_date) AS first_diabetes_date,
  MAX(CASE WHEN m.measurement_concept_id = 3004410 THEN m.value_as_number END) AS hba1c,
  MAX(CASE WHEN m.measurement_concept_id = 3038553 THEN m.value_as_number END) AS bmi,
  MAX(CASE WHEN c2.condition_concept_id IN (443238, 4030518) THEN 1 ELSE 0 END) AS has_nephropathy,
  MAX(CASE WHEN c3.condition_concept_id IN (374873, 4174977) THEN 1 ELSE 0 END) AS has_retinopathy,
  MAX(CASE WHEN v2.visit_concept_id = 9201 
       AND v2.visit_start_date BETWEEN DATEADD(month,-12,CURRENT_DATE) AND CURRENT_DATE 
       THEN 1 ELSE 0 END) AS hospitalized_last_12m
FROM person p
JOIN visit_occurrence v ON p.person_id = v.person_id
-- ... (full query in scripts/mimic_to_fclc.sql)
```

---

### 3. NHANES (National Health and Nutrition Examination Survey) ⭐ ДЛЯ ДИАБЕТА

| Параметр | Значение |
|----------|----------|
| Тип | US nationally representative population survey |
| Доступность | Public domain (CDC), без ограничений |
| Размер | ~5,000 участников на волну, 2001–2022 |
| Диабет-специфичность | HbA1c, BMI, диагнозы — все есть |
| IRB | Не требуется (публичные данные) |

**Загрузка:**
```bash
# Python пакет для NHANES
pip install nhanes

python3 -c "
from nhanes.load import load_NHANES_data
df = load_NHANES_data(year='2017-2018', components=['DEMO', 'GHB', 'BMX', 'DIQ'])
df.to_csv('nhanes_2017_2018.csv', index=False)
"
```

**NHANES переменные → FCLC OmopRecord:**
| NHANES | Описание | FCLC поле |
|--------|----------|-----------|
| RIDAGEYR | Возраст (лет) | age_group |
| RIAGENDR | Пол (1=M, 2=F) | sex |
| LBXGH | HbA1c (%) | hba1c_last |
| BMXBMI | ИМТ | bmi |
| DIQ010 | Диагноз диабета врачом | diabetes_dx |
| DIQ080 | Ретинопатия | has_retinopathy |
| KIQ022 | Болезнь почек | has_nephropathy |

**Скрипт конвертации:**
```bash
python3 scripts/nhanes_to_fclc.py nhanes_2017_2018.csv fclc_nhanes.csv
```

---

### 4. OHDSI Eunomia — Официальные тестовые данные OMOP

| Параметр | Значение |
|----------|----------|
| Тип | Синтетические OMOP CDM данные (официальные от OHDSI) |
| Доступность | R пакет, GitHub, без ограничений |
| Размер | ~26,000 пациентов (GiBleed study) |
| OMOP совместимость | 100% (это официальный CDM тест-датасет) |

```r
install.packages("Eunomia")
library(Eunomia)
connectionDetails <- getEunomiaConnectionDetails()
# Экспорт в CSV для FCLC
```

---

### 5. UK Biobank (для будущей валидации)

| Параметр | Значение |
|----------|----------|
| Тип | Проспективная когорта, 500,000 участников |
| Доступность | Требует application + Data Access Agreement |
| Срок получения | 3–6 месяцев |
| Стоимость | £ 200–2,000 зависит от проекта |
| Применение | Валидация FCLC после пилота, публикация в высокорейтинговых журналах |

**Action item:** Подать заявку на UK Biobank (target: September 2026).
Форма: https://www.ukbiobank.ac.uk/enable-your-research/apply-for-access

---

## Стратегия тестирования FCLC

### Этап 1 — Demo (сейчас): Synthea
```
3 виртуальные клиники × 1000 синтетических пациентов
→ 5 раундов федерального обучения
→ Проверка: AUC > 0.70, Shapley sum = 1.0, DP budget tracking
→ Никакой IRB не нужен
```

### Этап 2 — Pilot (апрель–июнь 2026): MIMIC-IV Demo + NHANES
```
MIMIC Demo (реальные данные, 100 пациентов) → 1 виртуальная клиника
NHANES (2 волны → 2 виртуальные клиники)
→ Тест де-идентификации на реальных данных
→ PhysioNet DUA подписан автоматически
→ Публикуемые результаты
```

### Этап 3 — Real Pilot (июль–декабрь 2026): Реальные клиники
```
3 клиники в Грузии (Tbilisi) или EU партнёр
→ DUA + IRB из этих шаблонов
→ EIC Pathfinder отчёт
```

---

## Скрипт генерации синтетических данных для FCLC

`scripts/generate_demo_data.py` — встроен в FCLC, не требует внешних зависимостей:

```bash
# Запуск демо без реальных данных
cd /home/oem/Desktop/FCLC
python3 scripts/generate_demo_data.py --nodes 3 --records 500 --seed 42

# Генерирует:
# data/clinic_node1_demo.csv
# data/clinic_node2_demo.csv
# data/clinic_node3_demo.csv
# Каждый файл: 500 записей в формате FCLC OmopRecord CSV
```

**Формат CSV для FCLC Node (импорт):**
```
age_years,sex,diabetes_year,hba1c,bmi,has_nephropathy,has_retinopathy,hospitalized_last_12m,hospitalized_next_12m
45,M,2015,7.2,28,0,0,0,1
62,F,2008,8.1,32,1,0,1,1
...
```

---

## Итог по датасетам

| Датасет | IRB нужен | Реальные данные | OMOP | Размер | Рекомендация |
|---------|-----------|-----------------|------|--------|-------------|
| Synthea | ❌ | ❌ (синт.) | ✅ | любой | ✅ **Старт сейчас** |
| MIMIC-IV Demo | ❌ (PhysioNet DUA) | ✅ | ✅ | 100 пац. | ✅ **Пилот** |
| NHANES | ❌ | ✅ | частично | 5K+ | ✅ **Пилот** |
| Eunomia | ❌ | ❌ (синт.) | ✅ | 26K | 🟡 Альтернатива |
| MIMIC-IV Full | ✅ (CITI) | ✅ | ✅ | 382K | 🔵 Публикация |
| UK Biobank | ✅ | ✅ | частично | 500K | 🔵 Валидация 2027 |

---

*Документ: FCLC/docs/DATASETS.md · v1.0 · 2026-04-06*
