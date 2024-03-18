use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use time::Date;

use crate::templates::plotting::{to_data_sets, PlottingTemplateDataSet};

use super::{power_emission, power_generation, power_import_export, Entity};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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

    pub async fn plotting_data(
        connection: &PgPool,
        entities: &[AgoraEntities],
        from: &Date,
        to: &Date,
    ) -> Vec<PlottingTemplateDataSet> {
        let mut data_set = Vec::new();

        for entity in entities {
            match entity {
                AgoraEntities::PowerGeneration => {
                    let result = power_generation::PowerGeneration::find_all_ordered_by_date(
                        connection, &from, &to,
                    )
                    .await;

                    data_set.extend(to_data_sets(result.unwrap()));
                }
                AgoraEntities::PowerEmission => {
                    let result = power_emission::PowerEmission::find_all_ordered_by_date(
                        connection, &from, &to,
                    )
                    .await;

                    data_set.extend(to_data_sets(result.unwrap()));
                }
                AgoraEntities::PowerImportExport => {
                    let result = power_import_export::PowerImportExport::find_all_ordered_by_date(
                        connection, &from, &to,
                    )
                    .await;

                    data_set.extend(to_data_sets(result.unwrap()));
                }
            }
        }

        data_set
    }
}
