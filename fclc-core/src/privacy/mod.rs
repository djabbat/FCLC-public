use crate::schema::{OmopRecord, anonymize_record, suppress_rare_records};

/// Configuration for de-identification pipeline.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeidentConfig {
    /// Minimum group size for k-anonymity (k).
    pub k_anonymity: usize,
    /// Suppress quasi-identifier combos below this count.
    pub suppress_rare_threshold: usize,
}

impl Default for DeidentConfig {
    fn default() -> Self {
        Self {
            k_anonymity: 5,
            suppress_rare_threshold: 5, // must equal k_anonymity (CONCEPT.md §Privacy)
        }
    }
}

/// De-identify a batch of OMOP records:
/// 1. Apply field-level anonymization (rounding, generalisation) to each record.
/// 2. Suppress rare quasi-identifier combinations (k-anonymity enforcement).
///
/// Modifies `records` in-place.
pub fn deidentify_batch(records: &mut Vec<OmopRecord>, config: &DeidentConfig) {
    // Step 1: field-level anonymization (generalise values)
    for r in records.iter_mut() {
        anonymize_record(r);
    }

    // Step 2: suppress rare (age_group, sex) combos
    suppress_rare_records(records, config.suppress_rare_threshold);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{AgeGroup, Sex};

    fn make_batch(n: usize, age: u8, sex: Sex) -> Vec<OmopRecord> {
        (0..n)
            .map(|_| OmopRecord {
                age_group: AgeGroup::from_age(age),
                sex,
                diabetes_diagnosis_year: Some(2018),
                hba1c_last: Some(6.5),
                bmi: Some(24.0),
                has_nephropathy: false,
                has_retinopathy: false,
                hospitalized_last_12m: false,
                hospitalized_next_12m: false,
            })
            .collect()
    }

    #[test]
    fn test_deidentify_does_not_panic() {
        let mut records = make_batch(10, 45, Sex::Female);
        let config = DeidentConfig::default();
        deidentify_batch(&mut records, &config);
    }

    #[test]
    fn test_rare_combo_suppressed() {
        let mut records = Vec::new();
        // 1 record with rare combo (count < threshold=3)
        records.extend(make_batch(1, 32, Sex::Male));
        // 10 records with common combo
        records.extend(make_batch(10, 45, Sex::Female));

        let config = DeidentConfig {
            k_anonymity: 5,
            suppress_rare_threshold: 3,
        };
        deidentify_batch(&mut records, &config);

        // The rare record should have sex generalised to Unknown
        assert_eq!(records[0].sex, Sex::Unknown);
        // Common records should retain their sex
        assert_eq!(records[1].sex, Sex::Female);
    }
}
