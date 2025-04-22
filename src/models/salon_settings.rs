use chrono::{NaiveDateTime, NaiveTime};
use diesel::{Queryable, Insertable, AsChangeset, Identifiable};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::schema::salon_settings;

/// Representação do banco de dados
#[derive(Debug, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = salon_settings)]
pub struct SalonSetting {
    pub id: Uuid,
    pub professional_id: Uuid,
    pub opening_hour: NaiveTime,
    pub closing_hour: NaiveTime,
    pub working_days: String,  // Armazenado como JSON string no banco
    pub created_at: NaiveDateTime,
}

/// Estrutura para inserção (usando tipos Diesel-compatíveis)
#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = salon_settings)]
pub struct NewSalonSetting {
    pub professional_id: Uuid,
    pub opening_hour: NaiveTime,
    pub closing_hour: NaiveTime,
    #[serde(deserialize_with = "deserialize_working_days")]
    pub working_days: String,  // String JSON serializada
}

/// Estrutura para atualização
#[derive(Debug, AsChangeset, Deserialize, Default)]
#[diesel(table_name = salon_settings)]
pub struct UpdateSalonSetting {
    #[serde(default)]
    pub opening_hour: Option<NaiveTime>,
    #[serde(default)]
    pub closing_hour: Option<NaiveTime>,
    #[serde(default, deserialize_with = "deserialize_working_days_option")]
    pub working_days: Option<String>,  // Optional String JSON
}

// Modelo de domínio com tipos convenientes
#[derive(Debug, Serialize, Deserialize)]
pub struct SalonSettings {
    pub id: Uuid,
    pub professional_id: Uuid,
    pub opening_hour: NaiveTime,
    pub closing_hour: NaiveTime,
    pub working_days: Vec<String>,  // Tipo conveniente para a aplicação
    pub created_at: NaiveDateTime,
}

// Conversão entre modelos
impl From<SalonSetting> for SalonSettings {
    fn from(db_model: SalonSetting) -> Self {
        let days: Vec<String> = serde_json::from_str(&db_model.working_days)
            .unwrap_or_default();
            
        Self {
            id: db_model.id,
            professional_id: db_model.professional_id,
            opening_hour: db_model.opening_hour,
            closing_hour: db_model.closing_hour,
            working_days: days,
            created_at: db_model.created_at,
        }
    }
}

impl From<SalonSettings> for NewSalonSetting {
    fn from(domain_model: SalonSettings) -> Self {
        let days_json = serde_json::to_string(&domain_model.working_days)
            .unwrap_or_default();
            
        Self {
            professional_id: domain_model.professional_id,
            opening_hour: domain_model.opening_hour,
            closing_hour: domain_model.closing_hour,
            working_days: days_json,
        }
    }
}

// Funções auxiliares de deserialização
fn deserialize_working_days<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let days: Vec<String> = Vec::deserialize(deserializer)?;
    serde_json::to_string(&days).map_err(serde::de::Error::custom)
}

fn deserialize_working_days_option<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt: Option<Vec<String>> = Option::deserialize(deserializer)?;
    match opt {
        Some(days) => {
            let json = serde_json::to_string(&days).map_err(serde::de::Error::custom)?;
            Ok(Some(json))
        }
        None => Ok(None),
    }
}