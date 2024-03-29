use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use time::Date;

use crate::templates::plotting::{to_data_sets, PlottingTemplateDataSet};

use super::{power_emission, power_generation, power_import_export, Average, Entity};

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
        use_average: &Average,
    ) -> Result<Vec<PlottingTemplateDataSet>, sqlx::Error> {
        let mut data_set = Vec::new();

        for entity in entities {
            match entity {
                AgoraEntities::PowerGeneration => {
                    let result = match use_average {
                        Average::None => {
                            power_generation::PowerGeneration::find_all_ordered_by_date(
                                connection, &from, &to,
                            )
                            .await?
                        }
                        Average::Daily => power_generation::PowerGeneration::find_all_ordered_by_date_average_daily(
                            connection, &from, &to,
                        )
                        .await?,
                        Average::Monthly => power_generation::PowerGeneration::find_all_ordered_by_date_average_monthly(
                            connection, &from, &to,
                        )
                        .await?,
                        Average::Yearly => power_generation::PowerGeneration::find_all_ordered_by_date_average_yearly(
                            connection, &from, &to,
                        )
                        .await?,
                    };

                    data_set.extend(to_data_sets(result));
                }
                AgoraEntities::PowerEmission => {
                    let result = match use_average {
                        Average::None => {
                            power_emission::PowerEmission::find_all_ordered_by_date(
                                connection, &from, &to,
                            )
                            .await?
                        }
                        Average::Daily => {
                            power_emission::PowerEmission::find_all_ordered_by_date_average_daily(
                                connection, &from, &to,
                            )
                            .await?
                        }
                        Average::Monthly => {
                            power_emission::PowerEmission::find_all_ordered_by_date_average_monthly(
                                connection, &from, &to,
                            )
                            .await?
                        }
                        Average::Yearly => {
                            power_emission::PowerEmission::find_all_ordered_by_date_average_yearly(
                                connection, &from, &to,
                            )
                            .await?
                        }
                    };

                    data_set.extend(to_data_sets(result));
                }
                AgoraEntities::PowerImportExport => {
                    let result = match use_average {
                        Average::None => power_import_export::PowerImportExport::find_all_ordered_by_date(
                            connection, &from, &to,
                        )
                        .await?,
                        Average::Daily => power_import_export::PowerImportExport::find_all_ordered_by_date_average_daily(
                            connection, &from, &to,
                        )
                        .await?,
                        Average::Monthly => power_import_export::PowerImportExport::find_all_ordered_by_date_average_monthly(
                            connection, &from, &to,
                        )
                        .await?,
                        Average::Yearly => power_import_export::PowerImportExport::find_all_ordered_by_date_average_yearly(
                            connection, &from, &to,
                        )
                        .await?,
                    };

                    data_set.extend(to_data_sets(result));
                }
            }
        }

        Ok(data_set)
    }
}
