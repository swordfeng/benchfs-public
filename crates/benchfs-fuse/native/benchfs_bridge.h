#ifndef BENCHFS_BRIDGE_H
#define BENCHFS_BRIDGE_H

#include <stddef.h>
#include <stdint.h>

enum benchfs_opcode {
    BENCHFS_LOOKUP = 1,
    BENCHFS_FORGET,
    BENCHFS_FORGET_MULTI,
    BENCHFS_GETATTR,
    BENCHFS_SETATTR,
    BENCHFS_READLINK,
    BENCHFS_MKNOD,
    BENCHFS_MKDIR,
    BENCHFS_UNLINK,
    BENCHFS_RMDIR,
    BENCHFS_SYMLINK,
    BENCHFS_RENAME,
    BENCHFS_LINK,
    BENCHFS_OPEN,
    BENCHFS_READ,
    BENCHFS_WRITE,
    BENCHFS_FLUSH,
    BENCHFS_RELEASE,
    BENCHFS_FSYNC,
    BENCHFS_OPENDIR,
    BENCHFS_READDIR,
    BENCHFS_RELEASEDIR,
    BENCHFS_FSYNCDIR,
    BENCHFS_STATFS,
    BENCHFS_SETXATTR,
    BENCHFS_GETXATTR,
    BENCHFS_LISTXATTR,
    BENCHFS_REMOVEXATTR,
    BENCHFS_CREATE,
    BENCHFS_FALLOCATE,
    BENCHFS_LSEEK
};

struct benchfs_forget {
    uint64_t inode;
    uint64_t count;
};

struct benchfs_request {
    uint32_t opcode;
    uint32_t uid;
    uint32_t gid;
    uint32_t pid;
    uint32_t umask;
    uint32_t attribute_uid;
    uint32_t attribute_gid;
    uint32_t flags;
    uint32_t flags2;
    uint32_t mode;
    uint32_t device_major;
    uint32_t device_minor;
    uint32_t has_handle;
    uint64_t inode;
    uint64_t inode2;
    uint64_t handle;
    int64_t offset;
    int64_t offset2;
    uint64_t size;
    uint64_t size2;
    int64_t atime_seconds;
    uint32_t atime_nanoseconds;
    int64_t mtime_seconds;
    uint32_t mtime_nanoseconds;
    const uint8_t *name;
    size_t name_length;
    const uint8_t *name2;
    size_t name2_length;
    const uint8_t *data;
    size_t data_length;
};

struct benchfs_attr {
    uint64_t inode;
    uint64_t generation;
    uint32_t mode;
    uint32_t uid;
    uint32_t gid;
    uint32_t device_major;
    uint32_t device_minor;
    uint64_t link_count;
    uint64_t size;
    uint64_t allocated_bytes;
    int64_t atime_seconds;
    uint32_t atime_nanoseconds;
    int64_t mtime_seconds;
    uint32_t mtime_nanoseconds;
    int64_t ctime_seconds;
    uint32_t ctime_nanoseconds;
};

struct benchfs_statfs {
    uint64_t blocks;
    uint64_t blocks_free;
    uint64_t blocks_available;
    uint64_t files;
    uint64_t files_free;
    uint32_t block_size;
    uint32_t fragment_size;
    uint32_t name_max;
    uint32_t read_only;
};

struct benchfs_dirent {
    const uint8_t *name;
    size_t name_length;
    uint64_t inode;
    uint32_t mode;
    uint64_t next_cookie;
};

typedef void (*benchfs_dispatch_fn)(void *userdata, void *request_handle,
                                    const struct benchfs_request *request);
typedef int (*benchfs_drain_fn)(void *userdata);

int benchfs_fuse_run(const char *mountpoint, void *userdata,
                     benchfs_dispatch_fn dispatch,
                     benchfs_drain_fn drain);
void benchfs_reply_error(void *request_handle, int error_number);
void benchfs_reply_none(void *request_handle);
void benchfs_reply_entry(void *request_handle, const struct benchfs_attr *attribute);
void benchfs_reply_attribute(void *request_handle, const struct benchfs_attr *attribute);
void benchfs_reply_open(void *request_handle, uint64_t handle, int directory);
void benchfs_reply_create(void *request_handle, const struct benchfs_attr *attribute,
                          uint64_t handle);
void benchfs_reply_data(void *request_handle, const uint8_t *data, size_t length);
void benchfs_reply_readlink_bytes(void *request_handle, const uint8_t *data, size_t length);
void benchfs_reply_write(void *request_handle, uint32_t count);
void benchfs_reply_statfs_value(void *request_handle, const struct benchfs_statfs *value);
void benchfs_reply_xattr_size(void *request_handle, size_t length);
void benchfs_reply_lseek_value(void *request_handle, int64_t offset);
void benchfs_reply_readdir_entries(void *request_handle, size_t buffer_size,
                                   const struct benchfs_dirent *entries, size_t count);
#endif
