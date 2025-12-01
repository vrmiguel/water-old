# Neon Postgres Database Tables

**Database**: ajudadora
**Host**: ep-withered-cloud-a4hgpu5o-pooler.us-east-1.aws.neon.tech
**Schema**: public

## Overview

This database contains **7 base tables** and **2 views** in the public schema.

## Base Tables

### 1. _sqlx_migrations
- **Size**: 32 kB
- **Columns**: 6
- **Description**: Migration tracking table for SQLx database migrations
- **Fields**: version, description, installed_on, success, checksum, execution_time

### 2. gifs
- **Size**: 456 kB
- **Columns**: 2
- **Description**: Stores GIF file references
- **Fields**: id (PK), file_id

### 3. memories
- **Size**: 55 MB (Largest table)
- **Columns**: 4
- **Description**: Stores memory content with embeddings (likely for AI/ML purposes)
- **Fields**: id (PK), memory_content, created_at, embedding (vector type)

### 4. messages
- **Size**: 2.9 MB
- **Columns**: 6
- **Description**: Stores message data with reply threading
- **Fields**: id (PK), sender_id (FK), text, sent_at, reply_to_id, created_at

### 5. senders
- **Size**: 64 kB
- **Columns**: 4
- **Description**: Stores sender/user information
- **Fields**: id (PK), username, created_at, first_name

### 6. socket_io_attachments
- **Size**: 16 kB
- **Columns**: 3
- **Description**: Stores Socket.IO attachment data
- **Fields**: id (PK), created_at, payload

### 7. stickers
- **Size**: 2.9 MB
- **Columns**: 2
- **Description**: Stores sticker file references
- **Fields**: id (PK), file_id

## Views

### 8. pg_stat_statements
- **Description**: PostgreSQL extension view for query statistics

### 9. pg_stat_statements_info
- **Description**: Metadata for pg_stat_statements

## Database Statistics

- **Total Base Tables**: 7
- **Total Views**: 2
- **Total Database Objects**: 9
- **Total Database Size**: ~61 MB (approximate sum of table sizes)
- **Largest Table**: memories (55 MB)

## Technical Notes

- The database uses PostgreSQL extensions including vector embeddings (USER-DEFINED type in memories.embedding)
- Connection requires SSL/TLS encryption
- Database appears to be a messaging/chat application with memory/AI features
- The `_sqlx_migrations` table indicates the use of SQLx for database migrations in a Rust application

---

*Generated on: 2025-12-01*
