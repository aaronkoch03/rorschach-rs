use crate::builders::condition_builder::ConditionBuilder;
use crate::builders::questionnaire_builder::QuestionnaireBuilder;
use crate::builders::questionnaire_item_builder::QuestionnaireItemBuilder;
use crate::condition::Condition;
use crate::error::RorschachError;
use crate::questionnaire::Questionnaire;
use crate::score_calculations::sum_score::SumScoreFirstN;
use crate::term::{PhenotypeTerms, SeverityTerms};
use chrono::Duration;
use maplit::btreemap;

/// HAM-D, 21 items (items 1-17 = classic 17-item core, 18-21 = additional items).
///
/// Item scales:
///   0-4: items 1, 2, 3, 7, 8, 9, 10, 11, 15, 19
///   0-2: items 4, 5, 6, 12, 13, 14, 16, 17, 18 (18b = severity), 21
///   0-3: item 20
///
/// Question 16, 17 reversed? Check!

const SCORE_ITEMS: usize = 17;
pub(crate) fn hamd21() -> Result<Questionnaire, RorschachError> {
    let name = "HAM-D-21";

    // 0-4 scale (items 1, 2, 3, 7, 8, 9, 10, 11, 15, 19)
    let sev4 = vec![
        (1, SeverityTerms::Borderline),
        (2, SeverityTerms::Moderate),
        (3, SeverityTerms::Moderate),
        (4, SeverityTerms::Profound),
    ];
    // 0-2 scale (items 4, 5, 6, 12, 13, 14, 16, 17, 18b, 21)
    let sev2 = vec![(1, SeverityTerms::Borderline), (2, SeverityTerms::Moderate)];
   
    // 0-3 scale (item 20)
    let sev3 = vec![
        (1, SeverityTerms::Borderline),
        (2, SeverityTerms::Moderate),
        (3, SeverityTerms::Profound),
    ];

    let builder = QuestionnaireBuilder::new(name, Box::new(SumScoreFirstN::new(SCORE_ITEMS)))
        .recall_period(Duration::weeks(1))
        //from Hamilton’s original work and the APA Handbook of Psychiatric Measures, while Zimmerman et al. (2013) suggest 
        //different scoring system (empirically derived bands), which is widely cited as an update
        .interpretations(btreemap! {
            0  => Some(Condition::new_excluded(PhenotypeTerms::Depression)),
            8  => Some(Condition::without_time(PhenotypeTerms::Depression, SeverityTerms::Mild)),
            14 => Some(Condition::without_time(PhenotypeTerms::Depression, SeverityTerms::Moderate)),
            19 => Some(Condition::without_time(PhenotypeTerms::Depression, SeverityTerms::Severe)),
            23 => Some(Condition::without_time(PhenotypeTerms::Depression, SeverityTerms::Profound)),
        })
        .items([
            // 1 Depressed mood (0-4)
            QuestionnaireItemBuilder::new(5, 4)
                .stem("Depressed mood")
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::PathologicalSadness)
                        .build_with_severities(sev4.as_slice(), true),
                ),
            // 2 Feelings of guilt (0-4)
            QuestionnaireItemBuilder::new(5, 4)
                .stem("Feelings of guilt")
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::Guilt)
                        .build_with_severities(sev4.as_slice(), true),
                ),
            // 3 Suicide (0-4). Alternative for score 4: Suicide behaviors HP:5200330
            QuestionnaireItemBuilder::new(5, 4)
                .stem("Suicide")
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::SuicidalIdeation)
                        .build_with_severities(sev4.as_slice(), true),
                ),
            // 4 Insomnia: initial (0-2)
            QuestionnaireItemBuilder::new(3, 2)
                .stem("Insomnia (initial)")
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::SleepOnsetInsomnia)
                        .build_with_severities(sev2.as_slice(), true),
                ),
            // 5 Insomnia: middle (0-2)
            QuestionnaireItemBuilder::new(3, 2)
                .stem("Insomnia (middle)")
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::MaintenanceInsomnia)
                        .build_with_severities(sev2.as_slice(), true),
                ),
            // 6 Insomnia: delayed / late (0-2)
            QuestionnaireItemBuilder::new(3, 2)
                .stem("Insomnia (delayed)")
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::TerminalInsomnia)
                        .build_with_severities(sev2.as_slice(), true),
                ),
            // 7 Work and activities (0-4). Alternative: Anhedonia
            QuestionnaireItemBuilder::new(5, 4)
                .stem("Work and activities")
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::ImpairmentOfActivitiesOfDailyLiving)
                        .build_with_severities(sev4.as_slice(), true),
                ),
            // 8 Retardation (0-4). No good HPO term (Bradykinesia is a motor sign), so no
            // phenotype; document the limitation in the output instead.
            QuestionnaireItemBuilder::new(5, 4)
                .stem("Retardation (psychomotor)")
                .empty_conditions(),
            
            // 9 Agitation (0-4)
            QuestionnaireItemBuilder::new(5, 4)
                .stem("Agitation")
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::Agitation)
                        .build_with_severities(sev4.as_slice(), true),
                ),

            // 10 Anxiety psychic (0-4)
            QuestionnaireItemBuilder::new(5, 4)
                .stem("Anxiety (psychic)")
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::Anxiety)
                        .build_with_severities(sev4.as_slice(), true),
                ),
            // 11 Anxiety somatic (0-4)
            QuestionnaireItemBuilder::new(5, 4)
                .stem("Anxiety (somatic)")
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::AbnormalAutonomicNervousSystemPhysiology)
                        .build_with_severities(sev4.as_slice(), true),
                ),

            // 12 Gastrointestinal somatic symptoms (0-2). Alternative: PoorAppetite?
            QuestionnaireItemBuilder::new(3, 2)
                .stem("Somatic symptoms (gastrointestinal)")
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::AbnormalityOfTheGastrointestinalTract)
                        .build_with_severities(sev2.as_slice(), true),
                ),
            
            // 13 General somatic symptoms (0-2) Term? Maybe fatigue (HP:0012378)?
            QuestionnaireItemBuilder::new(3, 2)
                .stem("Somatic symptoms (general)")
                .empty_conditions(),
            
            // 14 Genital symptoms (0-2)
            QuestionnaireItemBuilder::new(3, 2)
                .stem("Genital symptoms")
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::AbnormalityOfTheGenitourinarySystem)
                        .build_with_severities(sev2.as_slice(), true),
                ),
            
            // 15 Hypochondriasis (0-4). only really the highest score justifies this term, but we can include it for completeness
            QuestionnaireItemBuilder::new(5, 4)
                .stem("Hypochondriasis")
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::HypochondriacDelusion)
                        .build_with_severities(sev4.as_slice(), true),
                ),
            
            // 16 Weight loss (0-2)
            QuestionnaireItemBuilder::new(3, 2)
                .stem("Weight loss")
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::WeightLoss)
                        .build_with_severities(sev2.as_slice(), true),
                ),
            
            // 17 Insight (0-2). Inverse scale: 0 = insight present, 2 = denies being ill at all.
            QuestionnaireItemBuilder::new(3, 2)
                .stem("Insight")
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::LackOfInsight)
                        .build_with_severities(sev2.as_slice(), true),
                ),
            
            // ---- items 18-21: additional items, not part of the 17-item score ----

            // 18 Diurnal variation (use column hamd_18b = severity 0-2)
            QuestionnaireItemBuilder::new(3, 2)
                .stem("Diurnal variation")
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::DiurnalVariation)
                        .build_with_severities(sev2.as_slice(), true),
                ),
            
            // 19 Depersonalization and derealization (0-4)
            QuestionnaireItemBuilder::new(5, 4)
                .stem("Depersonalization and derealization")
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::Depersonalization)
                        .build_with_severities(sev4.as_slice(), true),
                ),
            
            // 20 Paranoid symptoms (0-3)
            QuestionnaireItemBuilder::new(4, 3)
                .stem("Paranoid symptoms")
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::Paranoia)
                        .build_with_severities(sev3.as_slice(), true),
                ),
            
            // 21 Obsessional symptoms (0-2)
            QuestionnaireItemBuilder::new(3, 2)
                .stem("Obsessional symptoms")
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::ObsessiveCompulsiveTrait)
                        .build_with_severities(sev2.as_slice(), true),
                ),
        ]);

    let builder = builder.map_err(|err| {
        RorschachError::BuildingError(format!("Error when building {name}: {}", err))
    })?;
    Ok(builder.build())
}
