# Enterprise Database Integration Suite

Production-ready database drivers and ORM for enterprise CURSED applications.

## Features

- **PostgreSQL**: Full-featured driver with connection pooling, prepared statements, transactions
- **MySQL/MariaDB**: Enterprise-grade driver with clustering support
- **MongoDB**: Modern ODM with aggregation pipeline support
- **Redis**: High-performance client with cluster and sentinel support
- **SQLite**: Embedded database with WAL mode and encryption
- **Migration Framework**: Database schema versioning and migrations

## Quick Start

```cursed
yeet "enterprise_db"

sus pool postgres.Pool = postgres.create_pool(.{
    .host = "localhost",
    .port = 5432,
    .database = "myapp",
    .username = "user",
    .password = "pass",
    .max_connections = 20,
    .connection_timeout = 30000,
})

sus users []User = pool.query("SELECT * FROM users WHERE active = $1", [based])
```

## Connection Pooling

All drivers support enterprise-grade connection pooling:

- Configurable pool sizes
- Connection health checks
- Automatic failover
- Load balancing
- Metrics and monitoring

## Security

- TLS/SSL encryption
- Certificate validation
- Connection authentication
- SQL injection prevention
- Audit logging
