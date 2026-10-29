use crate::condition::Condition;
use crate::error::RorschachError;
use crate::traits::AsTerm;
use std::fmt;
use strum::{EnumCount, EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Term {
    id: String,
    label: String,
}

impl Term {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn label(&self) -> &str {
        &self.label
    }
    pub fn as_severity(&self) -> Option<SeverityTerms> {
        if self.id == SeverityTerms::Borderline.as_term().id {
            Some(SeverityTerms::Borderline)
        } else if self.id == SeverityTerms::Mild.as_term().id {
            Some(SeverityTerms::Mild)
        } else if self.id == SeverityTerms::Moderate.as_term().id {
            Some(SeverityTerms::Moderate)
        } else if self.id == SeverityTerms::Severe.as_term().id {
            Some(SeverityTerms::Severe)
        } else if self.id == SeverityTerms::Profound.as_term().id {
            Some(SeverityTerms::Profound)
        } else {
            None
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} [{}]", self.label, self.id)
    }
}

pub enum DiagnosisTerms {
    DepressiveDisorder,
    AnxietyDisorder,
}

impl DiagnosisTerms {
    pub fn as_term(&self) -> Term {
        match self {
            DiagnosisTerms::DepressiveDisorder => Term::new("MONDO:0002050", "depressive disorder"),
            DiagnosisTerms::AnxietyDisorder => Term::new("MONDO:0005618", "anxiety disorder"),
        }
    }
}
impl From<DiagnosisTerms> for Term {
    fn from(value: DiagnosisTerms) -> Self {
        value.as_term()
    }
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum PhenotypeTerms {
    Anhedonia,
    Depression,
    SleepDisturbance,
    Fatigue,
    AbnormalEatingBehavior,
    LowSelfEsteem,
    Guilt,
    PsychomotorDeterioration,
    SuicidalIdeation,
    DiminishedAbilityToConcentrate,
    Anxiety,
    SenseOfImpendingDoom,
    Ruminations,
    Agitation,
    Restlessness,
    Irritability,
    AnticipatoryAnxiety,
    Euphoria,
    Grandiosity,
    DecreasedNeedForSleep,
    PressuredSpeech,
    PathologicalSadness,
    Mania,
    AmplificationOfSexualBehavior,
    AggressiveBehavior,
    LackOfInsight,
    Psychosis,
    FlightOfIdeas,
    DisorderOfThoughtContent,
    AbnormallyRapidThoughtProcess,
    Delusion,
    SelfNeglect,
    PoorAppetite,
    Apathy,
    Hopelessness,
    Phobia,
    Insomnia,
    AbnormalityOfTheCardiovascularSystem,
    AbnormalityOfTheRespiratorySystem,
    AbnormalityOfTheGenitourinarySystem,
    AbnormalityOfTheGastrointestinalTract,
    CognitiveImpairment,
    AbnormalAutonomicNervousSystemPhysiology,
    AtypicalBehavior,
    SleepOnsetInsomnia,
    MaintenanceInsomnia,
    TerminalInsomnia,
    ImpairmentOfActivitiesOfDailyLiving,
    HypochondriacDelusion,
    WeightLoss,
    DiurnalVariation,
    Depersonalization,
    Paranoia,
    ObsessiveCompulsiveTrait,
}

impl AsTerm for PhenotypeTerms {
    fn as_term(&self) -> Term {
        match self {
            PhenotypeTerms::Anhedonia => Term::new("HP:0012154", "Anhedonia"),
            PhenotypeTerms::Depression => Term::new("HP:0000716", "Depression"),
            PhenotypeTerms::SleepDisturbance => Term::new("HP:0002360", "Sleep disturbance"),
            PhenotypeTerms::Fatigue => Term::new("HP:0012378", "Fatigue"),
            PhenotypeTerms::AbnormalEatingBehavior => {
                Term::new("HP:0100738", "Abnormal eating behavior")
            }
            PhenotypeTerms::LowSelfEsteem => Term::new("HP:0031469", "Low self-esteem"),
            PhenotypeTerms::Guilt => Term::new("HP:6000011", "Guilt"),
            PhenotypeTerms::PsychomotorDeterioration => {
                Term::new("HP:0002361", "Psychomotor deterioration")
            }
            PhenotypeTerms::SuicidalIdeation => Term::new("HP:0031589", "Suicidal ideation"),
            PhenotypeTerms::DiminishedAbilityToConcentrate => {
                Term::new("HP:0031987", "Diminished ability to concentrate")
            }
            PhenotypeTerms::Anxiety => Term::new("HP:0000739", "Anxiety"),
            PhenotypeTerms::Ruminations => Term::new("HP:0025771", "Ruminations"),
            PhenotypeTerms::Agitation => Term::new("HP:0000713", "Agitation"),
            PhenotypeTerms::Restlessness => Term::new("HP:0000711", "Restlessness"),
            PhenotypeTerms::Irritability => Term::new("HP:0000737", "Irritability"),
            PhenotypeTerms::AnticipatoryAnxiety => Term::new("HP:5200233", "Anticipatory anxiety"),
            PhenotypeTerms::Euphoria => Term::new("HP:0031844", "Euphoria"),
            PhenotypeTerms::Grandiosity => Term::new("HP:5200270", "Grandiosity"),
            PhenotypeTerms::DecreasedNeedForSleep => {
                Term::new("HP:5200276", "Decreased need for sleep")
            }
            PhenotypeTerms::PressuredSpeech => Term::new("HP:5200265", "Pressured speech"),
            PhenotypeTerms::PathologicalSadness => Term::new("HP:5200273", "Pathological sadness"),
            PhenotypeTerms::Mania => Term::new("HP:0100754", "Mania"),
            PhenotypeTerms::AmplificationOfSexualBehavior => {
                Term::new("HP:5200321", "Amplification of sexual behavior")
            }
            PhenotypeTerms::AggressiveBehavior => Term::new("HP:0000718", "Aggressive behavior"),
            PhenotypeTerms::LackOfInsight => Term::new("HP:0000757", "Lack of insight"),
            PhenotypeTerms::Psychosis => Term::new("HP:0000709", "Psychosis"),
            PhenotypeTerms::FlightOfIdeas => Term::new("HP:5200234", "Flight of ideas"),
            PhenotypeTerms::DisorderOfThoughtContent => {
                Term::new("HP:0025779", "Disorder of thought content")
            }
            PhenotypeTerms::AbnormallyRapidThoughtProcess => {
                Term::new("HP:0025781", "Abnormally rapid thought process")
            }
            PhenotypeTerms::Delusion => Term::new("HP:0000746", "Delusion"),
            PhenotypeTerms::SelfNeglect => Term::new("HP:0025479", "Self-neglect"),
            PhenotypeTerms::PoorAppetite => Term::new("HP:0004396", "Poor appetite"),
            PhenotypeTerms::Apathy => Term::new("HP:0000741", "Apathy"),
            PhenotypeTerms::Hopelessness => Term::new("HP:5200271", "Hopelessness"),
            PhenotypeTerms::Phobia => Term::new("HP:5200232", "Phobia"),
            PhenotypeTerms::Insomnia => Term::new("HP:0100785", "Insomnia"),
            PhenotypeTerms::AbnormalityOfTheCardiovascularSystem => {
                Term::new("HP:0001626", "Abnormality of the cardiovascular system")
            }
            PhenotypeTerms::AbnormalityOfTheRespiratorySystem => {
                Term::new("HP:0002086", "Abnormality of the respiratory system")
            }
            PhenotypeTerms::AbnormalityOfTheGastrointestinalTract => {
                Term::new("HP:0011024", "Abnormality of the gastrointestinal tract")
            }
            PhenotypeTerms::AbnormalityOfTheGenitourinarySystem => {
                Term::new("HP:0000119", "Abnormality of the genitourinary system")
            }
            PhenotypeTerms::CognitiveImpairment => Term::new("HP:0100543", "Cognitive impairment"),
            PhenotypeTerms::AbnormalAutonomicNervousSystemPhysiology => {
                Term::new("HP:0012332", "Abnormal autonomic nervous system physiology")
            }
            PhenotypeTerms::AtypicalBehavior => Term::new("HP:0000708", "Atypical behavior"),
            PhenotypeTerms::SenseOfImpendingDoom => {
                Term::new("HP:0033845", "Sense of impending doom")
            }
            PhenotypeTerms::SleepOnsetInsomnia => Term::new("HP:0031354", "Sleep onset insomnia"),
            PhenotypeTerms::MaintenanceInsomnia => Term::new("HP:0031355", "Maintenance insomnia"),
            PhenotypeTerms::TerminalInsomnia => Term::new("HP:0031356", "Terminal insomnia"),
            PhenotypeTerms::ImpairmentOfActivitiesOfDailyLiving => {
                Term::new("HP:0031058", "Impairment of activities of daily living")
            }
            PhenotypeTerms::HypochondriacDelusion => {
                Term::new("HP:5200425", "Hypochondriac delusion")
            }
            PhenotypeTerms::WeightLoss => Term::new("HP:0001824", "Weight loss"),
            PhenotypeTerms::DiurnalVariation => Term::new("HP:0025302", "Diurnal"),
            PhenotypeTerms::Depersonalization => Term::new("HP:5200217", "Depersonalization"),
            PhenotypeTerms::Paranoia => Term::new("HP:0011999", "Paranoia"),
            PhenotypeTerms::ObsessiveCompulsiveTrait => {
                Term::new("HP:0008770", "Obsessive-compulsive trait")
            }
        }
    }
}

impl From<PhenotypeTerms> for Term {
    fn from(value: PhenotypeTerms) -> Self {
        value.as_term()
    }
}

impl From<PhenotypeTerms> for Condition {
    fn from(value: PhenotypeTerms) -> Self {
        Condition::from_type(value)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumIter, EnumCount)]
pub enum SeverityTerms {
    // The severities need to be sorted descending for from_category to work
    Borderline,
    Mild,
    Moderate,
    Severe,
    Profound,
}
impl AsTerm for SeverityTerms {
    fn as_term(&self) -> Term {
        match self {
            SeverityTerms::Borderline => Term::new("HP:0012827", "Borderline"),
            SeverityTerms::Mild => Term::new("HP:0012825", "Mild"),
            SeverityTerms::Moderate => Term::new("HP:0012826", "Moderate"),
            SeverityTerms::Severe => Term::new("HP:0012828", "Severe"),
            SeverityTerms::Profound => Term::new("HP:0012829", "Profound"),
        }
    }
}
impl SeverityTerms {
    pub fn from_category(severity: i16) -> Result<SeverityTerms, RorschachError> {
        if let Some(s) = SeverityTerms::iter().get(severity as usize) {
            Ok(s)
        } else {
            Err(RorschachError::CantMapSeverity(severity as f32))
        }
    }
}

impl From<SeverityTerms> for Term {
    fn from(value: SeverityTerms) -> Self {
        value.as_term()
    }
}

impl From<&SeverityTerms> for Term {
    fn from(value: &SeverityTerms) -> Self {
        value.as_term()
    }
}
