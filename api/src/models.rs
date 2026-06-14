use crate::insertPayload::AppState;
use axum::{extract::State, http::StatusCode, Json};
use serde::Serialize;
use serde_json::Value;
use sqlx::Row;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct MachineInfo {
    pub id: String,
    pub hostname: String,
    pub os: String,
    pub os_version: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct LatestMetricsInfo {
    pub machine: MachineInfo,
    pub metrics_id: String,
    pub cpus: Value,
    pub disks: Value,
    pub networks: Value,
    pub memory_used: f64,
    pub memory_total: f64,
    pub memory_free: f64,
    pub total_swap: f64,
    pub used_swap: f64,
    pub nombre_processeurs: f32,
    pub uptime: i64,
    pub nombre_processus: i64,
    pub created_at: String,
}

pub async fn get_latest_metrics(
    State(state): State<AppState>,
) -> Result<Json<Vec<LatestMetricsInfo>>, (StatusCode, String)> {
    let rows = sqlx::query(
        r#"
        SELECT
            m.id AS machine_id,
            m.hostname,
            m.os,
            m.os_version,
            m.created_at::text AS machine_created_at,
            met.id AS metrics_id,
            met.cpus,
            met.disks,
            met.networks,
            met.memory_used,
            met.memory_total,
            met.memory_free,
            met.total_swap,
            met.used_swap,
            met.nombre_processeurs,
            met.uptime,
            met.nombre_processus,
            met.created_at::text AS metrics_created_at
        FROM machine m
        JOIN LATERAL (
            SELECT *
            FROM metrics
            WHERE machine_id = m.id
            ORDER BY created_at DESC
            LIMIT 1
        ) met ON true
        ORDER BY m.hostname
        "#,
    )
    .fetch_all(&state.db)
    .await
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {}", err)))?;

    let latest_metrics = rows
        .into_iter()
        .map(|row| LatestMetricsInfo {
            machine: MachineInfo {
                id: row.get::<Uuid, _>("machine_id").to_string(),
                hostname: row.get::<String, _>("hostname"),
                os: row.get::<String, _>("os"),
                os_version: row.get::<String, _>("os_version"),
                created_at: row.get::<String, _>("machine_created_at"),
            },
            metrics_id: row.get::<Uuid, _>("metrics_id").to_string(),
            cpus: row.get::<Value, _>("cpus"),
            disks: row.get::<Value, _>("disks"),
            networks: row.get::<Value, _>("networks"),
            memory_used: row.get::<f64, _>("memory_used"),
            memory_total: row.get::<f64, _>("memory_total"),
            memory_free: row.get::<f64, _>("memory_free"),
            total_swap: row.get::<f64, _>("total_swap"),
            used_swap: row.get::<f64, _>("used_swap"),
            nombre_processeurs: row.get::<f32, _>("nombre_processeurs"),
            uptime: row.get::<i64, _>("uptime"),
            nombre_processus: row.get::<i64, _>("nombre_processus"),
            created_at: row.get::<String, _>("metrics_created_at"),
        })
        .collect::<Vec<LatestMetricsInfo>>();

    Ok(Json(latest_metrics))
}

pub async fn get_machines(
    State(state): State<AppState>,
) -> Result<Json<Vec<MachineInfo>>, (StatusCode, String)> {
    let rows = sqlx::query(
        r#"
        SELECT id::text AS id, hostname, os, os_version, created_at::text AS created_at
        FROM machine
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(&state.db)
    .await
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {}", err)))?;

    let machines = rows
        .into_iter()
        .map(|row| MachineInfo {
            id: row.get::<String, _>("id"),
            hostname: row.get::<String, _>("hostname"),
            os: row.get::<String, _>("os"),
            os_version: row.get::<String, _>("os_version"),
            created_at: row.get::<String, _>("created_at"),
        })
        .collect::<Vec<MachineInfo>>();

    Ok(Json(machines))
}
