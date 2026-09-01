# BenchFS Feature Overview

[中文](features.zh-CN.md)

BenchFS has three variants:

| Variant | Features |
|---|---|
| Core | Files, directories, links, metadata, file I/O, synchronization, and clean-remount persistence |
| Journal | Core plus metadata journaling and crash recovery |
| COW | Core plus copy-on-write, snapshots, reflinks, and space reclamation |

## Shared features

### Capacity

BenchFS supports large files and directories, reports filesystem capacity, handles space exhaustion, and reuses storage after deletion.

### Extents

Like ext4 and XFS, BenchFS maps file data with extents.

### Sparse files and persistent preallocation

Files may contain holes and preallocated unwritten ranges. BenchFS supports sparse-file navigation, truncation, growth, zeroing, and hole punching.

### B+ tree indexes

Like XFS, BenchFS uses B+ trees for scalable metadata indexes.

### Extended attributes

Files and directories support user extended attributes.

### Metadata checksums

Persistent metadata is checksummed so corruption can be detected.

## Journal

### Journaling

Like ext3, ext4, and XFS, Journal records metadata updates in a write-ahead log and recovers committed state after a crash.

## COW

### Copy-on-write

Like Btrfs, COW preserves existing state while writing changed data and metadata to new storage. Storage no longer reachable after an update can be reclaimed.

### Snapshots

COW can create, list, mount, and delete snapshots. Mounted snapshot views are read-only.

### Reflinks

Whole-file and range reflinks clone files by sharing extents. A later write separates the changed data while leaving the other file unchanged.
