use crate::db::DbPool;
use axum::{extract::State, Json};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use sqlx::Row;
use uuid::Uuid;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2
};
use argon2::password_hash::PasswordHash;
use axum::http::StatusCode;
use argon2::PasswordVerifier;

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
    pub total: f64,
    pub used: f64,
    pub free: f64,
    pub total_swap: f64,
    pub used_swap: f64,
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


#[derive(Debug, Deserialize,Serialize)]
pub struct UserData {
    pub email: String,
    pub password: String,
    pub created_at: Option<String>,
}

pub async fn check_user(
    State(state): State<AppState>,
    Json(receive_users): Json<UserData>,
) -> Result<Json<Value>, (StatusCode, String)> {

    let row = sqlx::query(
        r#"
        SELECT id, email, password_hash
        FROM users
        WHERE email = $1
        "#
    )
    .bind(&receive_users.email)
    .fetch_optional(&state.db)
    .await
    .map_err(|err| (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("DB error: {}", err)
    ))?;

    // user not found
    let row = match row {
        Some(r) => r,
        None => return Err((StatusCode::UNAUTHORIZED, "User not found".to_string())),
    };

    let password_hash: String = row.get("password_hash");

    let parsed_hash = PasswordHash::new(&password_hash)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Invalid hash format".to_string()))?;

    Argon2::default()
        .verify_password(receive_users.password.as_bytes(), &parsed_hash)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid password".to_string()))?;

    let body = json!({
        "status": "success",
    });

    Ok(Json(body))
}

pub async fn insert_user(
    State(state): State<AppState>,
    Json(receive_users): Json<UserData>,
) -> Result<Json<Value>, (axum::http::StatusCode, String)> {
    let id = Uuid::new_v4();
    let mut tx = state.db.begin().await.map_err(|err| {
        let msg = format!("DB error: {}", err);
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg)
    })?;

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(receive_users.password.as_bytes(), &salt)
        .map_err(|err| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?
        .to_string();


    sqlx::query(
        r#"
        INSERT INTO users (
            id,
            email,
            password_hash,
            created_at
        )

        VALUES ($1,$2,$3,NOW())
        "#,
    )
    .bind(id)
    .bind(&receive_users.email)
    .bind(&password_hash)
    .execute(&mut *tx)
    .await
   .map_err(|err| {
    let msg = format!("DB error INSERT user: {}", err);
    println!("❌ {}", msg);
    (axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg)
})?;

    tx.commit().await.map_err(|err| {
        let msg = format!("DB error commit: {}", err);
        println!("❌ {}", msg);
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg)
    })?;

    let body = json!({
        "status": "success",
    });

    Ok(Json(body))
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
    let msg = format!("DB error INSERT metrics: {}", err);
    println!("❌ {}", msg);
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
    .bind(receive_metrics.memory.used)
    .bind(receive_metrics.memory.total)
    .bind(receive_metrics.memory.free)
    .bind(receive_metrics.memory.total_swap)
    .bind(receive_metrics.memory.used_swap)
    .bind(receive_metrics.global_data.nombre_processeurs as f32)
    .bind(receive_metrics.global_data.uptime as i64)
    .bind(receive_metrics.global_data.nombre_processus as i64)
    .execute(&mut *tx)
    .await
   .map_err(|err| {
    let msg = format!("DB error INSERT machine: {}", err);
    println!("❌ {}", msg);
    (axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg)
})?;

    tx.commit().await.map_err(|err| {
        let msg = format!("DB error commit: {}", err);
        println!("❌ {}", msg);
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg)
    })?;

    let body = json!({
        "status": "success",
    });

    Ok(Json(body))
}
