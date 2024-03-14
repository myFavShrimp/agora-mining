use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum AgoraEntities {
    PowerGeneration,
    PowerEmission,
}

impl AgoraEntities {
    pub fn get_title(&self) -> &str {
        return match self {
            AgoraEntities::PowerGeneration => { "Stromerzeugung" }
            AgoraEntities::PowerEmission => { "C0₂-Emissionen" }
        };
    }

    pub fn all() -> Vec<AgoraEntities> {
        return vec![
            AgoraEntities::PowerGeneration,
            AgoraEntities::PowerEmission,
        ];
    }
}