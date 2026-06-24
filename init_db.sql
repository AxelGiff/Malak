DROP TABLE IF EXISTS metrics;
DROP TABLE IF EXISTS machine;
DROP TABLE IF EXISTS users;


CREATE TABLE machine(
    id UUID PRIMARY KEY,
    hostname VARCHAR(255) UNIQUE NOT NULL,
    os VARCHAR(255) NOT NULL,
    os_version VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE metrics(
    id UUID PRIMARY KEY,
    machine_id UUID REFERENCES machine(id),
    cpus JSONB NOT NULL,
    disks JSONB NOT NULL,
    networks JSONB NOT NULL,
    memory_used DOUBLE PRECISION NOT NULL,
    memory_total DOUBLE PRECISION NOT NULL,
    memory_free DOUBLE PRECISION NOT NULL,
    total_swap DOUBLE PRECISION NOT NULL,
    used_swap DOUBLE PRECISION NOT NULL,
    nombre_processeurs REAL NOT NULL,
    uptime BIGINT NOT NULL,
    nombre_processus BIGINT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE users (
    id UUID PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);