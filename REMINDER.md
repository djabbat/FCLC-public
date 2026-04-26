# REMINDER — FCLC Временные допущения для peer review

> ⚠️ Этот файл читается при каждом запуске сессии по FCLC.
> Временные допущения действуют ТОЛЬКО для peer review. Для реальной подачи гранта — всё должно быть фактически выполнено.

---

## Активные временные допущения (v11, 2026-04-10)

| # | Условие | Допущение | Действие для реальной подачи |
|---|---------|-----------|------------------------------|
| **A1** | EU-eligible консорциум | ✅ УСЛОВНО — EU партнёр подтверждён; LEVF переведён в статус unpaid scientific advisor; заменён EU-eligible institution | Получить подписанный LoI/MOU от EU-учреждения (DFKI / Fraunhofer / KU Leuven) |
| **A2** | Ze Theory опубликована | ✅ УСЛОВНО — статья принята в рецензируемом журнале (Entropy MDPI или J.Math.Biology) | Подать в Entropy до Q4 2026; получить accept до подачи гранта |
| **A5** | PI институциональная аффилиация | ✅ УСЛОВНО — Host Institution Agreement подписан с EU/Associated Country университетом | Подписать HIA с грузинским или EU университетом до дедлайна EIC |

**Условия A3 (χ_Ze validation) и A4 (ISO DP) реализованы в коде:**
- `ChiZeValidationStudy::phase2_minimum_spec()` — протокол Phase 2 валидации определён
- `DpIsoCompliantConfig::fclc_iso_5rounds()` — ISO-совместимый DP config (ε_total=0.75 < 1.0)

---

## Напоминания при каждом запуске

1. **A1 (консорциум)** — EU partner contact: Carlos / Karim (deadline: 2026-05-01 встреча). Если нет ответа до 2026-04-12 — писать в DFKI напрямую.
2. **A2 (публикация Ze)** — Подать в Entropy MDPI до Q4 2026. Черновик статьи: `Ze/Articles/`. Pre-print arXiv сначала (centriole account).
3. **A5 (аффилиация PI)** — Jaba: проверить статус переговоров с Kutaisi International University или Tbilisi State Medical University как host institution.

---

## Другие активные напоминания

| Дата | Действие |
|------|---------|
| ~~**2026-04-12**~~ | ~~Дедлайн EU партнёра (внутренний) — если нет ответа → DFKI~~ → ✅ DFKI отправлено 2026-04-11; Fraunhofer + KU Leuven черновики готовы |
| ~~**2026-04-15**~~ | ~~P1-M СРОЧНО — δ-расхождение~~ → ✅ РЕШЁН (2026-04-11) |
| **2026-04-17** | 🔴 Zoom с Aubrey de Grey 9am PDT = 20:00 Тбилиси — повестка: CDATA CEP135/TERT; обсудить LoS для Longevity Impetus |
| **2026-04-18** | Дедлайн ответа DFKI — если нет → немедленно Fraunhofer IESE + KU Leuven (черновики: Email_Fraunhofer_IESE_EUPartner.md, Email_KULeuven_EUPartner.md) |
| **2026-04-20** | Дедлайн: ответ Georgian clinical partner (Shashviashvili / её университет) + EIC HIA подписание (Georgia Longevity Alliance, Рег. №404506520) |
| **2026-04-25** | 🔴 Дедлайн Longevity Impetus LOI — подать после zoom с Aubrey (PDF готов: Desktop/LOI_Longevity_Impetus_2026-04-25.pdf) |
| **2026-05-01** | Встреча Karim + Carlos — приложить: EIC Part A+B draft + BioSense flagship |
| **2026-05-12** | Дедлайн EIC Pathfinder Open 2026 |
| **2026-07-15** | P0-C — DPIA: нанять GDPR-юриста; начать шаги D1–D5 |

---

## 🧬 AUBREY DE GREY — CDATA scientific advisor / LoS

**Контакт:** Aubrey de Grey (LEVF)
**Статус:** ✅ **2 письма отправлены 2026-04-11; LOI PDF приложен**
**Ответ:** "Sure, happy to write a letter of support if I think the logic holds."
**Zoom:** Пятница 17 апреля, 9am PDT = **20:00 Тбилиси**
**Цель:** Получить LoS для Longevity Impetus + научный совет по CEP135/TERT дизайну
**Файл переписки:** `FCLC/docs/Correspondence_AubreyDeGrey_2026-04-11.md`

---

## 🏥 КЛИНИЧЕСКИЙ ПАРТНЁР ГРУЗИЯ — Shashviashvili (EIC Pathfinder WP3 node)

**Контакт:** Кетеван Шашвиашвили (ketevan.shashviashvili@...) — получатель письма 2026-04-10
**Тема:** Клинический узел FCLC в Грузии: n≈65-70 пациентов, IRB, FCLC node hosting, €150,000
**Статус:** 🟡 **ОТВЕТ ПОЛУЧЕН 2026-04-11** — «передала коллегам, ждите ответа в ближайшие дни»
**Дедлайн:** 20 апреля 2026 (из письма: «аффилиации подтверждение нужно в 10-дневный срок»)

**Важно:** Это потенциальный клинический партнёр для EIC [EIC-M4]. Даже письмо поддержки (Letter of Support) без финансовых обязательств достаточно для Part A Impact.

**Следующий шаг:** Отправить ответное письмо с благодарностью + мягким напоминанием о дедлайне + предложением документов и встречи. Черновик: `/home/oem/Desktop/Email_Shashviashvili_Reply.md`

---

---

## 🔬 СВЕЖИЕ НАУЧНЫЕ РЕЗУЛЬТАТЫ (2026-04-11, добавлено в v13 цикл)

### BioSense — Dortmund confirmatory null
| Тест | Результат | Статус |
|------|-----------|--------|
| Gamma 25–35 Гц (Dortmund, confirmatory) | d=0.422, p=0.113 | ❌ NULL — primary hypothesis FAILED |
| Alpha 8–13 Гц (Dortmund, NOT pre-registered) | d=0.594, p=0.028 | ✅ Exploratory — hypothesis generator |
| **Решение:** Новая pre-registration на alpha band, confirmatory на MPI-LEMON | Файл обновлён | `~/Desktop/AsPredicted_PreRegistration.md` |

**✅ OSF Pre-registration ПОДАНА: 2026-04-11 22:11** — osf.io/9m3yx ✅
MPI-LEMON alpha анализ теперь можно запускать.

### CDATA — Sobol N=4096 (правильная калибровка nu=1.0 div/yr)
| Параметр | S1 (первый порядок) | Ранг |
|----------|---------------------|------|
| nu (скорость деления) | 0.416 | 1 |
| alpha (повреждение/деление) | 0.193 | 2 |
| fibrosis_k | — | 3 |
| S1_sum | 0.608 | Надёжно |

Первый прогон (N=1024, nu=0.1 — артефакт miscalibration) показывал regen_factor S1=0.92 — НЕПРАВИЛЬНО.
N=4096, nu=1.0 — правильный биологический результат. Поддерживает CDATA теорию.

### LOI Longevity Impetus — статус
- Добавлен Arm C (TERT-KO) для cross-falsification (2026-04-11) ✅
- PDF создан и отправлен Aubrey de Grey (jaba@longevity.ge) ✅
- Zoom Aubrey: 2026-04-17 20:00 Тбилиси (9am PDT)

---

*Последнее обновление: 2026-04-11 (v13 цикл)*
