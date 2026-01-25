# CMRust Save Format

## Overview

Saves are compressed JSON with integrity verification.

## File Structure

```
┌─────────────────────────────────┐
│ Header (32 bytes)               │
│ - Magic: "CMRS" (4 bytes)       │
│ - Version: u32 (4 bytes)        │
│ - Checksum: SHA256 (32 bytes)   │
├─────────────────────────────────┤
│ Payload (gzip compressed)       │
│ - World state                   │
│ - Game config                   │
│ - Game state                    │
└─────────────────────────────────┘
```

## Version Migration

Each save has a version number:
```rust
pub const SAVE_VERSION: u32 = 1;
```

When loading older saves, migrations are applied:
- v0 → v1: Add inbox field
- v1 → v2: (future)

## Payload Schema

```json
{
  "world": { ... },
  "game_config": {
    "difficulty": 2,
    "auto_save": true
  },
  "game_state": {
    "date": "2001-08-15",
    "manager_name": "Manager",
    "club_id": "LIV",
    "inbox": [ ... ]
  }
}
```

## Integrity Check

On load:
1. Read header checksum
2. Decompress payload
3. Calculate SHA256 of payload
4. Compare checksums
5. If mismatch, reject save

## File Extension

`.cmsave` - CMRust save file
