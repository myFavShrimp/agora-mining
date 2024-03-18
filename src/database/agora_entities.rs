use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum AgoraEntities {
    PowerGeneration,
    PowerEmission,
    PowerImportExport,
}

impl AgoraEntities {
    pub fn get_title(&self) -> &str {
        return match self {
            AgoraEntities::PowerGeneration => "Stromerzeugung",
            AgoraEntities::PowerEmission => "C0₂-Emissionen",
            AgoraEntities::PowerImportExport => "Stromimport, -export & -preis",
        };
    }

    pub fn all() -> Vec<AgoraEntities> {
        return vec![
            AgoraEntities::PowerGeneration,
            AgoraEntities::PowerEmission,
            AgoraEntities::PowerImportExport,
        ];
    }
}
