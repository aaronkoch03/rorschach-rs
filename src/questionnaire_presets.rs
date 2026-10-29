use crate::presets::asrm::asrm;
use crate::presets::gad7::gad7;
use crate::presets::hama::hama;
use crate::presets::madrs::madrs;
use crate::presets::phq9::phq9;
use crate::presets::ymrs::ymrs;
use crate::presets::hamd21::hamd21;
use crate::questionnaire::Questionnaire;

pub enum QuestionnairePresets {
    PHQ9,
    GAD7,
    ASRM,
    YMRS,
    MADRS,
    HAMA,
    HAMD21,
}

impl QuestionnairePresets {
    pub fn build(&self) -> Questionnaire {
        match self {
            QuestionnairePresets::PHQ9 => {
                phq9().expect("Should always work, because its hard coded")
            }
            QuestionnairePresets::GAD7 => {
                gad7().expect("Should always work, because its hard coded")
            }
            QuestionnairePresets::ASRM => {
                asrm().expect("Should always work, because its hard coded")
            }
            QuestionnairePresets::YMRS => {
                ymrs().expect("Should always work, because its hard coded")
            }
            QuestionnairePresets::MADRS => {
                madrs().expect("Should always work, because its hard coded")
            }
            QuestionnairePresets::HAMA => {
                hama().expect("Should always work, because its hard coded")
            }
            QuestionnairePresets::HAMD21 => {
                hamd21().expect("Should always work, because its hard coded")
            }
        }
    }
}
