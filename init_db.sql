DROP TABLE IF EXISTS metrics;
DROP TABLE IF EXISTS machine;

CREATE TABLE machine(
    id UUID PRIMARY KEY,
    hostname VARCHAR(255) NOT NULL,
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
    cpu_global REAL NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);