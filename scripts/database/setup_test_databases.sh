#!/bin/bash

# fr fr Database test environment setup script - getting databases ready for testing periodt
#
# This script sets up test databases for CI/CD and development environments:
# - SQLite test databases (file-based and in-memory)
# - PostgreSQL test database (with Docker)
# - MySQL test database (with Docker)
# - Test data fixtures and schema setup
# - Environment configuration for tests

set -euo pipefail

# Script configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
TEST_DATA_DIR="$PROJECT_ROOT/tests/fixtures/database"
DOCKER_COMPOSE_FILE="$SCRIPT_DIR/docker-compose.test-databases.yml"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Print usage information
usage() {
    cat << EOF
Usage: $0 [OPTIONS]

Setup test databases for CURSED database package testing.

OPTIONS:
    -h, --help          Show this help message
    -a, --all          Setup all database types (default)
    -s, --sqlite       Setup SQLite databases only
    -p, --postgres     Setup PostgreSQL database only
    -m, --mysql        Setup MySQL database only
    -c, --clean        Clean up existing test databases
    -d, --docker       Use Docker for PostgreSQL and MySQL
    --no-docker        Don't use Docker (requires local databases)
    --ci               CI mode - non-interactive setup
    --fixtures         Create test data fixtures
    --schema-only      Create schema without test data

EXAMPLES:
    $0 --all           # Setup all databases with Docker
    $0 --sqlite        # Setup SQLite databases only
    $0 --clean --all   # Clean and recreate all databases
    $0 --ci --all      # CI-friendly setup of all databases

EOF
}

# Parse command line arguments
SETUP_ALL=true
SETUP_SQLITE=false
SETUP_POSTGRES=false
SETUP_MYSQL=false
USE_DOCKER=true
CI_MODE=false
CLEAN_FIRST=false
CREATE_FIXTURES=true
SCHEMA_ONLY=false

while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            usage
            exit 0
            ;;
        -a|--all)
            SETUP_ALL=true
            shift
            ;;
        -s|--sqlite)
            SETUP_ALL=false
            SETUP_SQLITE=true
            shift
            ;;
        -p|--postgres)
            SETUP_ALL=false
            SETUP_POSTGRES=true
            shift
            ;;
        -m|--mysql)
            SETUP_ALL=false
            SETUP_MYSQL=true
            shift
            ;;
        -c|--clean)
            CLEAN_FIRST=true
            shift
            ;;
        -d|--docker)
            USE_DOCKER=true
            shift
            ;;
        --no-docker)
            USE_DOCKER=false
            shift
            ;;
        --ci)
            CI_MODE=true
            shift
            ;;
        --fixtures)
            CREATE_FIXTURES=true
            shift
            ;;
        --schema-only)
            SCHEMA_ONLY=true
            CREATE_FIXTURES=false
            shift
            ;;
        *)
            log_error "Unknown option: $1"
            usage
            exit 1
            ;;
    esac
done

# Set individual flags if --all is specified
if [[ "$SETUP_ALL" == "true" ]]; then
    SETUP_SQLITE=true
    SETUP_POSTGRES=true
    SETUP_MYSQL=true
fi

# Check dependencies
check_dependencies() {
    log_info "Checking dependencies..."
    
    # Check for sqlite3
    if [[ "$SETUP_SQLITE" == "true" ]] && ! command -v sqlite3 &> /dev/null; then
        log_error "sqlite3 is required but not installed"
        exit 1
    fi
    
    # Check for Docker if needed
    if [[ "$USE_DOCKER" == "true" ]] && [[ ("$SETUP_POSTGRES" == "true" || "$SETUP_MYSQL" == "true") ]]; then
        if ! command -v docker &> /dev/null; then
            log_error "Docker is required but not installed"
            exit 1
        fi
        
        if ! command -v docker-compose &> /dev/null; then
            log_error "docker-compose is required but not installed"
            exit 1
        fi
        
        # Check if Docker daemon is running
        if ! docker info &> /dev/null; then
            log_error "Docker daemon is not running"
            exit 1
        fi
    fi
    
    log_success "All dependencies satisfied"
}

# Create test data directories
create_directories() {
    log_info "Creating test data directories..."
    
    mkdir -p "$TEST_DATA_DIR"
    mkdir -p "$TEST_DATA_DIR/sqlite"
    mkdir -p "$TEST_DATA_DIR/postgres"
    mkdir -p "$TEST_DATA_DIR/mysql"
    mkdir -p "$TEST_DATA_DIR/fixtures"
    mkdir -p "$TEST_DATA_DIR/schemas"
    
    log_success "Test directories created"
}

# Clean existing test databases
clean_databases() {
    if [[ "$CLEAN_FIRST" != "true" ]]; then
        return
    fi
    
    log_info "Cleaning existing test databases..."
    
    # Clean SQLite databases
    if [[ "$SETUP_SQLITE" == "true" ]]; then
        rm -f "$TEST_DATA_DIR/sqlite"/*.db
        log_info "Cleaned SQLite test databases"
    fi
    
    # Clean Docker containers
    if [[ "$USE_DOCKER" == "true" ]]; then
        if [[ -f "$DOCKER_COMPOSE_FILE" ]]; then
            docker-compose -f "$DOCKER_COMPOSE_FILE" down -v &> /dev/null || true
            log_info "Cleaned Docker test containers"
        fi
    fi
    
    log_success "Database cleanup completed"
}

# Setup SQLite test databases
setup_sqlite() {
    if [[ "$SETUP_SQLITE" != "true" ]]; then
        return
    fi
    
    log_info "Setting up SQLite test databases..."
    
    # Create test database files
    local sqlite_dir="$TEST_DATA_DIR/sqlite"
    
    # Main test database
    local main_db="$sqlite_dir/cursed_test.db"
    sqlite3 "$main_db" "SELECT 1;" &> /dev/null
    
    # Performance test database
    local perf_db="$sqlite_dir/cursed_performance_test.db"
    sqlite3 "$perf_db" "SELECT 1;" &> /dev/null
    
    # Migration test database
    local migration_db="$sqlite_dir/cursed_migration_test.db"
    sqlite3 "$migration_db" "SELECT 1;" &> /dev/null
    
    log_success "SQLite databases created: $main_db, $perf_db, $migration_db"
    
    # Create schema
    create_sqlite_schema "$main_db"
    create_sqlite_schema "$perf_db"
    
    # Create test data if requested
    if [[ "$CREATE_FIXTURES" == "true" ]]; then
        create_sqlite_fixtures "$main_db"
    fi
}

# Create SQLite schema
create_sqlite_schema() {
    local db_file="$1"
    
    log_info "Creating SQLite schema in $(basename "$db_file")..."
    
    sqlite3 "$db_file" << 'EOF'
-- Users table
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    age INTEGER CHECK(age >= 0),
    is_active BOOLEAN DEFAULT true,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Posts table
CREATE TABLE IF NOT EXISTS posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    content TEXT,
    is_published BOOLEAN DEFAULT false,
    view_count INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Comments table
CREATE TABLE IF NOT EXISTS comments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    post_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    content TEXT NOT NULL,
    is_approved BOOLEAN DEFAULT false,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Categories table
CREATE TABLE IF NOT EXISTS categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,
    description TEXT,
    parent_id INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (parent_id) REFERENCES categories(id)
);

-- Post categories junction table
CREATE TABLE IF NOT EXISTS post_categories (
    post_id INTEGER NOT NULL,
    category_id INTEGER NOT NULL,
    PRIMARY KEY (post_id, category_id),
    FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE,
    FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE CASCADE
);

-- Tags table
CREATE TABLE IF NOT EXISTS tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,
    color TEXT DEFAULT '#666666',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Post tags junction table
CREATE TABLE IF NOT EXISTS post_tags (
    post_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY (post_id, tag_id),
    FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_posts_user_id ON posts(user_id);
CREATE INDEX IF NOT EXISTS idx_posts_created_at ON posts(created_at);
CREATE INDEX IF NOT EXISTS idx_comments_post_id ON comments(post_id);
CREATE INDEX IF NOT EXISTS idx_comments_user_id ON comments(user_id);

-- Views for common queries
CREATE VIEW IF NOT EXISTS user_post_counts AS
SELECT 
    u.id,
    u.username,
    u.email,
    COUNT(p.id) as post_count,
    COUNT(CASE WHEN p.is_published THEN 1 END) as published_count
FROM users u
LEFT JOIN posts p ON u.id = p.user_id
GROUP BY u.id, u.username, u.email;

CREATE VIEW IF NOT EXISTS post_with_stats AS
SELECT 
    p.*,
    u.username as author_username,
    COUNT(c.id) as comment_count
FROM posts p
JOIN users u ON p.user_id = u.id
LEFT JOIN comments c ON p.id = c.post_id
GROUP BY p.id;
EOF

    log_success "SQLite schema created"
}

# Create SQLite test fixtures
create_sqlite_fixtures() {
    local db_file="$1"
    
    log_info "Creating SQLite test fixtures in $(basename "$db_file")..."
    
    sqlite3 "$db_file" << 'EOF'
-- Insert test users
INSERT OR IGNORE INTO users (username, email, first_name, last_name, age) VALUES
('alice_dev', 'alice@example.com', 'Alice', 'Johnson', 28),
('bob_writer', 'bob@example.com', 'Bob', 'Smith', 34),
('charlie_admin', 'charlie@example.com', 'Charlie', 'Brown', 42),
('diana_user', 'diana@example.com', 'Diana', 'Prince', 29),
('eve_blogger', 'eve@example.com', 'Eve', 'Wilson', 31);

-- Insert test categories
INSERT OR IGNORE INTO categories (name, description) VALUES
('Technology', 'Technology and programming posts'),
('Tutorial', 'How-to guides and tutorials'),
('News', 'Latest news and updates'),
('Opinion', 'Personal opinions and thoughts'),
('Review', 'Product and service reviews');

-- Insert test tags
INSERT OR IGNORE INTO tags (name, color) VALUES
('cursed', '#ff6b6b'),
('database', '#4ecdc4'),
('tutorial', '#45b7d1'),
('beginner', '#96ceb4'),
('advanced', '#feca57'),
('testing', '#ff9ff3'),
('performance', '#54a0ff');

-- Insert test posts
INSERT OR IGNORE INTO posts (user_id, title, content, is_published, view_count) VALUES
(1, 'Getting Started with CURSED Database', 'This post explains how to use the CURSED database package...', true, 150),
(1, 'Advanced Transaction Management', 'Learn about complex transaction scenarios...', true, 89),
(2, 'Connection Pooling Best Practices', 'Optimize your database connections with pooling...', true, 234),
(2, 'Migration Strategies', 'How to handle database schema changes...', false, 0),
(3, 'Performance Benchmarking Guide', 'Comprehensive guide to database performance testing...', true, 567),
(3, 'Error Handling Patterns', 'Best practices for database error handling...', true, 123),
(4, 'CRUD Operations Tutorial', 'Basic database operations explained...', true, 89),
(5, 'NoSQL vs SQL Comparison', 'When to use different database types...', true, 201);

-- Insert test comments
INSERT OR IGNORE INTO comments (post_id, user_id, content, is_approved) VALUES
(1, 2, 'Great introduction! Very helpful for beginners.', true),
(1, 3, 'Could you add more examples?', true),
(1, 4, 'Thanks for sharing this!', true),
(2, 4, 'Advanced stuff, but well explained.', true),
(3, 1, 'Connection pooling is crucial for performance.', true),
(3, 5, 'Any recommendations for pool sizing?', true),
(5, 2, 'Excellent benchmarking methodology.', true),
(5, 4, 'Would love to see more database comparisons.', true);

-- Link posts to categories
INSERT OR IGNORE INTO post_categories (post_id, category_id) VALUES
(1, 1), (1, 2),  -- Technology, Tutorial
(2, 1), (2, 2),  -- Technology, Tutorial
(3, 1), (3, 2),  -- Technology, Tutorial
(4, 1), (4, 2),  -- Technology, Tutorial
(5, 1), (5, 5),  -- Technology, Review
(6, 1), (6, 2),  -- Technology, Tutorial
(7, 2),          -- Tutorial
(8, 1), (8, 4);  -- Technology, Opinion

-- Link posts to tags
INSERT OR IGNORE INTO post_tags (post_id, tag_id) VALUES
(1, 1), (1, 2), (1, 4),  -- cursed, database, beginner
(2, 1), (2, 2), (2, 5),  -- cursed, database, advanced
(3, 1), (3, 2), (3, 7),  -- cursed, database, performance
(4, 1), (4, 2),          -- cursed, database
(5, 1), (5, 2), (5, 6), (5, 7),  -- cursed, database, testing, performance
(6, 1), (6, 2), (6, 5),  -- cursed, database, advanced
(7, 1), (7, 2), (7, 3), (7, 4),  -- cursed, database, tutorial, beginner
(8, 2), (8, 4);          -- database, opinion
EOF

    log_success "SQLite test fixtures created"
}

# Create Docker Compose file for test databases
create_docker_compose() {
    if [[ "$USE_DOCKER" != "true" ]]; then
        return
    fi
    
    log_info "Creating Docker Compose configuration..."
    
    cat > "$DOCKER_COMPOSE_FILE" << 'EOF'
version: '3.8'

services:
  postgres-test:
    image: postgres:15
    environment:
      POSTGRES_DB: cursed_test
      POSTGRES_USER: cursed_test
      POSTGRES_PASSWORD: cursed_test
      POSTGRES_INITDB_ARGS: "--encoding=UTF8 --locale=C"
    ports:
      - "5433:5432"  # Use non-standard port to avoid conflicts
    volumes:
      - postgres_test_data:/var/lib/postgresql/data
      - ./postgres/init.sql:/docker-entrypoint-initdb.d/init.sql
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U cursed_test -d cursed_test"]
      interval: 10s
      timeout: 5s
      retries: 5
    networks:
      - cursed_test_network

  mysql-test:
    image: mysql:8.0
    environment:
      MYSQL_DATABASE: cursed_test
      MYSQL_USER: cursed_test
      MYSQL_PASSWORD: cursed_test
      MYSQL_ROOT_PASSWORD: cursed_root_test
    ports:
      - "3307:3306"  # Use non-standard port to avoid conflicts
    volumes:
      - mysql_test_data:/var/lib/mysql
      - ./mysql/init.sql:/docker-entrypoint-initdb.d/init.sql
    healthcheck:
      test: ["CMD", "mysqladmin", "ping", "-h", "localhost", "-u", "cursed_test", "-pcursed_test"]
      interval: 10s
      timeout: 5s
      retries: 5
    networks:
      - cursed_test_network

volumes:
  postgres_test_data:
  mysql_test_data:

networks:
  cursed_test_network:
    driver: bridge
EOF

    log_success "Docker Compose configuration created"
}

# Setup PostgreSQL test database
setup_postgres() {
    if [[ "$SETUP_POSTGRES" != "true" ]]; then
        return
    fi
    
    log_info "Setting up PostgreSQL test database..."
    
    if [[ "$USE_DOCKER" == "true" ]]; then
        setup_postgres_docker
    else
        setup_postgres_local
    fi
}

# Setup PostgreSQL with Docker
setup_postgres_docker() {
    # Create initialization script
    mkdir -p "$SCRIPT_DIR/postgres"
    
    cat > "$SCRIPT_DIR/postgres/init.sql" << 'EOF'
-- Create test schema
CREATE SCHEMA IF NOT EXISTS cursed_test;
SET search_path TO cursed_test;

-- Users table
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    age INTEGER CHECK(age >= 0),
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Posts table
CREATE TABLE IF NOT EXISTS posts (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    content TEXT,
    is_published BOOLEAN DEFAULT false,
    view_count INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Comments table
CREATE TABLE IF NOT EXISTS comments (
    id SERIAL PRIMARY KEY,
    post_id INTEGER NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    is_approved BOOLEAN DEFAULT false,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_posts_user_id ON posts(user_id);
CREATE INDEX IF NOT EXISTS idx_posts_created_at ON posts(created_at);
CREATE INDEX IF NOT EXISTS idx_comments_post_id ON comments(post_id);
CREATE INDEX IF NOT EXISTS idx_comments_user_id ON comments(user_id);

-- Insert test data
INSERT INTO users (username, email, first_name, last_name, age) VALUES
('alice_dev', 'alice@example.com', 'Alice', 'Johnson', 28),
('bob_writer', 'bob@example.com', 'Bob', 'Smith', 34),
('charlie_admin', 'charlie@example.com', 'Charlie', 'Brown', 42)
ON CONFLICT (username) DO NOTHING;

INSERT INTO posts (user_id, title, content, is_published, view_count) VALUES
(1, 'PostgreSQL with CURSED', 'How to use PostgreSQL with the CURSED database package', true, 120),
(2, 'Advanced PostgreSQL Features', 'Exploring advanced PostgreSQL capabilities', true, 89),
(3, 'Database Administration', 'PostgreSQL administration best practices', false, 0)
ON CONFLICT DO NOTHING;
EOF

    # Start PostgreSQL container
    docker-compose -f "$DOCKER_COMPOSE_FILE" up -d postgres-test
    
    # Wait for PostgreSQL to be ready
    log_info "Waiting for PostgreSQL to be ready..."
    timeout=60
    while ! docker-compose -f "$DOCKER_COMPOSE_FILE" exec -T postgres-test pg_isready -U cursed_test -d cursed_test &> /dev/null; do
        sleep 2
        timeout=$((timeout - 2))
        if [[ $timeout -le 0 ]]; then
            log_error "PostgreSQL failed to start within 60 seconds"
            exit 1
        fi
    done
    
    log_success "PostgreSQL test database is ready (port 5433)"
}

# Setup PostgreSQL locally
setup_postgres_local() {
    log_warning "Local PostgreSQL setup requires manual configuration"
    log_info "Please ensure PostgreSQL is running and create:"
    log_info "  Database: cursed_test"
    log_info "  User: cursed_test"
    log_info "  Password: cursed_test"
}

# Setup MySQL test database
setup_mysql() {
    if [[ "$SETUP_MYSQL" != "true" ]]; then
        return
    fi
    
    log_info "Setting up MySQL test database..."
    
    if [[ "$USE_DOCKER" == "true" ]]; then
        setup_mysql_docker
    else
        setup_mysql_local
    fi
}

# Setup MySQL with Docker
setup_mysql_docker() {
    # Create initialization script
    mkdir -p "$SCRIPT_DIR/mysql"
    
    cat > "$SCRIPT_DIR/mysql/init.sql" << 'EOF'
USE cursed_test;

-- Users table
CREATE TABLE IF NOT EXISTS users (
    id INT PRIMARY KEY AUTO_INCREMENT,
    username VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    age INT CHECK(age >= 0),
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Posts table
CREATE TABLE IF NOT EXISTS posts (
    id INT PRIMARY KEY AUTO_INCREMENT,
    user_id INT NOT NULL,
    title TEXT NOT NULL,
    content TEXT,
    is_published BOOLEAN DEFAULT false,
    view_count INT DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Comments table
CREATE TABLE IF NOT EXISTS comments (
    id INT PRIMARY KEY AUTO_INCREMENT,
    post_id INT NOT NULL,
    user_id INT NOT NULL,
    content TEXT NOT NULL,
    is_approved BOOLEAN DEFAULT false,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Create indexes
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_posts_user_id ON posts(user_id);
CREATE INDEX idx_posts_created_at ON posts(created_at);
CREATE INDEX idx_comments_post_id ON comments(post_id);
CREATE INDEX idx_comments_user_id ON comments(user_id);

-- Insert test data
INSERT IGNORE INTO users (username, email, first_name, last_name, age) VALUES
('alice_dev', 'alice@example.com', 'Alice', 'Johnson', 28),
('bob_writer', 'bob@example.com', 'Bob', 'Smith', 34),
('charlie_admin', 'charlie@example.com', 'Charlie', 'Brown', 42);

INSERT IGNORE INTO posts (user_id, title, content, is_published, view_count) VALUES
(1, 'MySQL with CURSED', 'How to use MySQL with the CURSED database package', true, 95),
(2, 'MySQL Performance Tuning', 'Optimizing MySQL for better performance', true, 156),
(3, 'Backup and Recovery', 'MySQL backup and recovery strategies', false, 0);
EOF

    # Start MySQL container
    docker-compose -f "$DOCKER_COMPOSE_FILE" up -d mysql-test
    
    # Wait for MySQL to be ready
    log_info "Waiting for MySQL to be ready..."
    timeout=60
    while ! docker-compose -f "$DOCKER_COMPOSE_FILE" exec -T mysql-test mysqladmin ping -h localhost -u cursed_test -pcursed_test &> /dev/null; do
        sleep 2
        timeout=$((timeout - 2))
        if [[ $timeout -le 0 ]]; then
            log_error "MySQL failed to start within 60 seconds"
            exit 1
        fi
    done
    
    log_success "MySQL test database is ready (port 3307)"
}

# Setup MySQL locally
setup_mysql_local() {
    log_warning "Local MySQL setup requires manual configuration"
    log_info "Please ensure MySQL is running and create:"
    log_info "  Database: cursed_test"
    log_info "  User: cursed_test"
    log_info "  Password: cursed_test"
}

# Create environment configuration file
create_env_config() {
    log_info "Creating environment configuration..."
    
    local env_file="$PROJECT_ROOT/.env.test"
    
    cat > "$env_file" << EOF
# CURSED Database Test Environment Configuration
# Generated by setup_test_databases.sh on $(date)

# SQLite Configuration
CURSED_SQLITE_TEST_DB="$TEST_DATA_DIR/sqlite/cursed_test.db"
CURSED_SQLITE_PERF_DB="$TEST_DATA_DIR/sqlite/cursed_performance_test.db"
CURSED_SQLITE_MIGRATION_DB="$TEST_DATA_DIR/sqlite/cursed_migration_test.db"

# PostgreSQL Configuration
CURSED_POSTGRES_TEST_URL="postgresql://cursed_test:cursed_test@localhost:5433/cursed_test"
CURSED_POSTGRES_HOST="localhost"
CURSED_POSTGRES_PORT="5433"
CURSED_POSTGRES_USER="cursed_test"
CURSED_POSTGRES_PASSWORD="cursed_test"
CURSED_POSTGRES_DATABASE="cursed_test"

# MySQL Configuration
CURSED_MYSQL_TEST_URL="mysql://cursed_test:cursed_test@localhost:3307/cursed_test"
CURSED_MYSQL_HOST="localhost"
CURSED_MYSQL_PORT="3307"
CURSED_MYSQL_USER="cursed_test"
CURSED_MYSQL_PASSWORD="cursed_test"
CURSED_MYSQL_DATABASE="cursed_test"

# Test Configuration
CURSED_TEST_MODE="true"
CURSED_TEST_DATA_DIR="$TEST_DATA_DIR"
CURSED_LOG_LEVEL="debug"
EOF

    log_success "Environment configuration created: $env_file"
}

# Verify database setup
verify_setup() {
    log_info "Verifying database setup..."
    
    local all_good=true
    
    # Verify SQLite
    if [[ "$SETUP_SQLITE" == "true" ]]; then
        local main_db="$TEST_DATA_DIR/sqlite/cursed_test.db"
        if [[ -f "$main_db" ]]; then
            local user_count=$(sqlite3 "$main_db" "SELECT COUNT(*) FROM users;" 2>/dev/null || echo "0")
            if [[ "$user_count" -gt "0" ]]; then
                log_success "SQLite: $user_count test users found"
            else
                log_warning "SQLite: No test users found"
                all_good=false
            fi
        else
            log_error "SQLite: Test database not found"
            all_good=false
        fi
    fi
    
    # Verify PostgreSQL
    if [[ "$SETUP_POSTGRES" == "true" && "$USE_DOCKER" == "true" ]]; then
        if docker-compose -f "$DOCKER_COMPOSE_FILE" exec -T postgres-test psql -U cursed_test -d cursed_test -c "SELECT COUNT(*) FROM cursed_test.users;" &> /dev/null; then
            local pg_count=$(docker-compose -f "$DOCKER_COMPOSE_FILE" exec -T postgres-test psql -U cursed_test -d cursed_test -t -c "SELECT COUNT(*) FROM cursed_test.users;" | tr -d ' \n')
            log_success "PostgreSQL: $pg_count test users found"
        else
            log_error "PostgreSQL: Failed to query test database"
            all_good=false
        fi
    fi
    
    # Verify MySQL
    if [[ "$SETUP_MYSQL" == "true" && "$USE_DOCKER" == "true" ]]; then
        if docker-compose -f "$DOCKER_COMPOSE_FILE" exec -T mysql-test mysql -u cursed_test -pcursed_test cursed_test -e "SELECT COUNT(*) FROM users;" &> /dev/null; then
            local mysql_count=$(docker-compose -f "$DOCKER_COMPOSE_FILE" exec -T mysql-test mysql -u cursed_test -pcursed_test cursed_test -se "SELECT COUNT(*) FROM users;")
            log_success "MySQL: $mysql_count test users found"
        else
            log_error "MySQL: Failed to query test database"
            all_good=false
        fi
    fi
    
    if [[ "$all_good" == "true" ]]; then
        log_success "All database setups verified successfully!"
    else
        log_warning "Some database setups may have issues"
    fi
}

# Print setup summary
print_summary() {
    log_info "Setup Summary"
    echo "─────────────────────────────────────────"
    
    if [[ "$SETUP_SQLITE" == "true" ]]; then
        echo "✅ SQLite databases created in $TEST_DATA_DIR/sqlite/"
    fi
    
    if [[ "$SETUP_POSTGRES" == "true" && "$USE_DOCKER" == "true" ]]; then
        echo "✅ PostgreSQL test database running on port 5433"
        echo "   Connection: postgresql://cursed_test:cursed_test@localhost:5433/cursed_test"
    fi
    
    if [[ "$SETUP_MYSQL" == "true" && "$USE_DOCKER" == "true" ]]; then
        echo "✅ MySQL test database running on port 3307"
        echo "   Connection: mysql://cursed_test:cursed_test@localhost:3307/cursed_test"
    fi
    
    echo "📋 Environment config: $PROJECT_ROOT/.env.test"
    echo "📁 Test data directory: $TEST_DATA_DIR"
    
    echo ""
    echo "🧪 To run tests:"
    echo "   cd $PROJECT_ROOT"
    echo "   source .env.test"
    echo "   cargo test --test database_integration_tests"
    echo ""
    
    if [[ "$USE_DOCKER" == "true" ]]; then
        echo "🐳 To stop Docker containers:"
        echo "   docker-compose -f $DOCKER_COMPOSE_FILE down"
        echo ""
    fi
}

# Main execution
main() {
    log_info "Starting CURSED database test environment setup"
    echo "═══════════════════════════════════════════════════"
    
    check_dependencies
    create_directories
    clean_databases
    
    if [[ "$USE_DOCKER" == "true" ]]; then
        create_docker_compose
    fi
    
    setup_sqlite
    setup_postgres
    setup_mysql
    
    create_env_config
    verify_setup
    print_summary
    
    log_success "Database test environment setup completed! 🎉"
}

# Run main function
main "$@"
