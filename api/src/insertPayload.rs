use crate::db::DbPool;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::Row;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub db: DbPool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MetricsPayload {
    pub global_data: GlobalData,
    pub memory: MemoryData,
    pub cpus: Vec<CpuData>,
    pub disks: Vec<DiskData>,
    pub networks: Vec<NetworkInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GlobalData {
    pub nom_machine: String,
    pub systeme_exploitation: String,
    pub version_systeme: String,
    pub uptime: u64,
    pub nombre_processus: usize,
    pub nombre_processeurs: usize,
}

#[derive(Debug, Deserialize,Serialize)]
pub struct MemoryData {
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub total_swap: u64,
    pub used_swap: u64,
}

#[derive(Debug, Deserialize,Serialize)]
pub struct CpuData {
    pub nom_cpu: String,
    pub usage: f32,
    pub utilisation: f32,
    pub frequence: u64,
    pub brand: String,
    pub id: usize,
    pub total_usage: f32,
}

#[derive(Debug, Deserialize,Serialize)]
pub struct DiskData {
    pub nom_disque: String,
    pub taille_totale: u64,
    pub espace_libre: u64,
    pub espace_utilise: u64,
    pub file_system: String,
    pub pourcentage_utilisation: f32,
}

#[derive(Debug, Deserialize,Serialize)]
pub struct NetworkInfo {
    pub interface: String,
    pub received: u64,
    pub transmitted: u64,
}


pub async fn insert_to_db(
    State(state): State<AppState>,
    Json(receive_metrics): Json<MetricsPayload>,
) -> Result<Json<Value>, (axum::http::StatusCode, String)> {
    let id = Uuid::new_v4();
    let mut tx = state.db.begin().await.map_err(|err| {
        let msg = format!("DB error: {}", err);
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg)
    })?;

    let row = sqlx::query(
        r#"
        INSERT INTO machine (
            id,
            hostname,
            os,
            os_version,
            created_at
        )

        VALUES ($1,$2,$3,$4,NOW())

        ON CONFLICT (hostname)

        DO UPDATE SET
            os = EXCLUDED.os,
            os_version = EXCLUDED.os_version

        RETURNING id, hostname, os, os_version, created_at::text AS created_at
        "#,
    )
    .bind(id)
    .bind(&receive_metrics.global_data.nom_machine)
    .bind(&receive_metrics.global_data.systeme_exploitation)
    .bind(&receive_metrics.global_data.version_systeme)
    .fetch_one(&mut *tx)
    .await
    .map_err(|err| {
        let msg = format!("DB error: {}", err);
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg)
    })?;

    let machine_id = row.get::<Uuid, _>("id");

    let cpus = serde_json::to_value(&receive_metrics.cpus).map_err(|err| {
        let msg = format!("Serialize error: {}", err);
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg)
    })?;
    let disks = serde_json::to_value(&receive_metrics.disks).map_err(|err| {
        let msg = format!("Serialize error: {}", err);
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg)
    })?;
    let networks = serde_json::to_value(&receive_metrics.networks).map_err(|err| {
        let msg = format!("Serialize error: {}", err);
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg)
    })?;

    sqlx::query(
        r#"
        INSERT INTO metrics (
            id,
            machine_id,
            cpus,
            disks,
            networks,
            memory_used,
            memory_total,
            memory_free,
            total_swap,
            used_swap,
            nombre_processeurs,
            uptime,
            nombre_processus,
            created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, NOW())
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(machine_id)
    .bind(cpus)
    .bind(disks)
    .bind(networks)
    .bind(receive_metrics.memory.used as i64)
    .bind(receive_metrics.memory.total as i64)
    .bind(receive_metrics.memory.free as i64)
    .bind(receive_metrics.memory.total_swap as i64)
    .bind(receive_metrics.memory.used_swap as i64)
    .bind(receive_metrics.global_data.nombre_processeurs as f32)
    .bind(receive_metrics.global_data.uptime as i64)
    .bind(receive_metrics.global_data.nombre_processus as i64)
    .execute(&mut *tx)
    .await
    .map_err(|err| {
        let msg = format!("DB error: {}", err);
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg)
    })?;

    tx.commit().await.map_err(|err| {
        let msg = format!("DB error: {}", err);
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg)
    })?;


    let body = json!({
        "status": "success",
    });

    Ok(Json(body))
}