use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use time::{Date, Duration};

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

        let from_plus_90_days = from
            .checked_add(Duration::days(90))
            .expect("We have not yet reached end of time");

        let use_daily_average = &from_plus_90_days < to;

        for entity in entities {
            match entity {
                AgoraEntities::PowerGeneration => {
                    let result = if use_daily_average {
                        power_generation::PowerGeneration::find_all_ordered_by_date_average_daily(
                            connection, &from, &to,
                        )
                        .await
                    } else {
                        power_generation::PowerGeneration::find_all_ordered_by_date(
                            connection, &from, &to,
                        )
                        .await
                    };

                    data_set.extend(to_data_sets(result.unwrap()));
                }
                AgoraEntities::PowerEmission => {
                    let result = if use_daily_average {
                        power_emission::PowerEmission::find_all_ordered_by_date_average_daily(
                            connection, &from, &to,
                        )
                        .await
                    } else {
                        power_emission::PowerEmission::find_all_ordered_by_date(
                            connection, &from, &to,
                        )
                        .await
                    };

                    data_set.extend(to_data_sets(result.unwrap()));
                }
                AgoraEntities::PowerImportExport => {
                    let result = if use_daily_average {
                        power_import_export::PowerImportExport::find_all_ordered_by_date_average_daily(
                            connection, &from, &to,
                        )
                        .await
                    } else {
                        power_import_export::PowerImportExport::find_all_ordered_by_date(
                            connection, &from, &to,
                        )
                        .await
                    };

                    data_set.extend(to_data_sets(result.unwrap()));
                }
            }
        }

        data_set
    }
}
