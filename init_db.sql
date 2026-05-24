DROP TABLE IF EXISTS metrics;
DROP TABLE IF EXISTS machine;

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
    memory_used BIGINT NOT NULL,
    memory_total BIGINT NOT NULL,
    memory_free BIGINT NOT NULL,
    total_swap BIGINT NOT NULL,
    used_swap BIGINT NOT NULL,
    nombre_processeurs REAL NOT NULL,
    uptime BIGINT NOT NULL,
    nombre_processus BIGINT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);