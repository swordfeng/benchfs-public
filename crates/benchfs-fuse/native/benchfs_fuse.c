#define _GNU_SOURCE
#define _FILE_OFFSET_BITS 64
#include "benchfs_bridge.h"
#include <stdbool.h>
#include <errno.h>
#include <fcntl.h>
#include <fuse_lowlevel.h>
#include <fuse_opt.h>
#include <linux/fs.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/statvfs.h>
#include <sys/sysmacros.h>
#include <unistd.h>

struct benchfs_session {
    void *userdata;
    benchfs_dispatch_fn dispatch;
    benchfs_drain_fn drain;
    struct fuse_session *session;
    int init_error;
};

static struct benchfs_session *bridge(fuse_req_t req) {
    return (struct benchfs_session *)fuse_req_userdata(req);
}

static void initialize_request(fuse_req_t req, struct benchfs_request *request,
                               uint32_t opcode, fuse_ino_t inode) {
    const struct fuse_ctx *context = fuse_req_ctx(req);
    memset(request, 0, sizeof(*request));
    request->opcode = opcode;
    request->inode = inode;
    if (context != NULL) {
        request->uid = context->uid;
        request->gid = context->gid;
        request->pid = (uint32_t)context->pid;
        request->umask = context->umask;
    }
}

static void dispatch(fuse_req_t req, struct benchfs_request *request) {
    struct benchfs_session *session = bridge(req);
    session->dispatch(session->userdata, req, request);
}

static void lookup_cb(fuse_req_t req, fuse_ino_t parent, const char *name) {
    struct benchfs_request request;
    initialize_request(req, &request, BENCHFS_LOOKUP, parent);
    request.name = (const uint8_t *)name;
    request.name_length = strlen(name);
    dispatch(req, &request);
}

static void forget_cb(fuse_req_t req, fuse_ino_t inode, uint64_t count) {
    struct benchfs_request request;
    initialize_request(req, &request, BENCHFS_FORGET, inode);
    request.size = count;
    dispatch(req, &request);
}

static void forget_multi_cb(fuse_req_t req, size_t count,
                            struct fuse_forget_data *forgets) {
    struct benchfs_request request;
    struct benchfs_forget *portable = calloc(count, sizeof(*portable));
    if (portable == NULL && count != 0) {
        fuse_reply_none(req);
        return;
    }
    for (size_t index = 0; index < count; ++index) {
        portable[index].inode = forgets[index].ino;
        portable[index].count = forgets[index].nlookup;
    }
    initialize_request(req, &request, BENCHFS_FORGET_MULTI, 0);
    request.data = (const uint8_t *)portable;
    request.data_length = count * sizeof(*portable);
    dispatch(req, &request);
    free(portable);
}

static void getattr_cb(fuse_req_t req, fuse_ino_t inode,
                       struct fuse_file_info *file) {
    struct benchfs_request request;
    initialize_request(req, &request, BENCHFS_GETATTR, inode);
    if (file != NULL) {
        request.has_handle = 1;
        request.handle = file->fh;
    }
    dispatch(req, &request);
}

static void setattr_cb(fuse_req_t req, fuse_ino_t inode, struct stat *attribute,
                       int to_set, struct fuse_file_info *file) {
    struct benchfs_request request;
    initialize_request(req, &request, BENCHFS_SETATTR, inode);
    request.flags = (uint32_t)to_set;
    request.mode = attribute->st_mode;
    request.attribute_uid = attribute->st_uid;
    request.attribute_gid = attribute->st_gid;
    request.size = (uint64_t)attribute->st_size;
    request.atime_seconds = attribute->st_atim.tv_sec;
    request.atime_nanoseconds = (uint32_t)attribute->st_atim.tv_nsec;
    request.mtime_seconds = attribute->st_mtim.tv_sec;
    request.mtime_nanoseconds = (uint32_t)attribute->st_mtim.tv_nsec;
    if (file != NULL) {
        request.has_handle = 1;
        request.handle = file->fh;
    }
    dispatch(req, &request);
}

static void readlink_cb(fuse_req_t req, fuse_ino_t inode) {
    struct benchfs_request request;
    initialize_request(req, &request, BENCHFS_READLINK, inode);
    dispatch(req, &request);
}

static void mknod_cb(fuse_req_t req, fuse_ino_t parent, const char *name,
                     mode_t mode, dev_t device) {
    struct benchfs_request request;
    initialize_request(req, &request, BENCHFS_MKNOD, parent);
    request.name = (const uint8_t *)name;
    request.name_length = strlen(name);
    request.mode = mode;
    request.device_major = major(device);
    request.device_minor = minor(device);
    dispatch(req, &request);
}

static void mkdir_cb(fuse_req_t req, fuse_ino_t parent, const char *name,
                     mode_t mode) {
    struct benchfs_request request;
    initialize_request(req, &request, BENCHFS_MKDIR, parent);
    request.name = (const uint8_t *)name;
    request.name_length = strlen(name);
    request.mode = mode;
    dispatch(req, &request);
}

static void unlink_cb(fuse_req_t req, fuse_ino_t parent, const char *name) {
    struct benchfs_request request;
    initialize_request(req, &request, BENCHFS_UNLINK, parent);
    request.name = (const uint8_t *)name;
    request.name_length = strlen(name);
    dispatch(req, &request);
}

static void rmdir_cb(fuse_req_t req, fuse_ino_t parent, const char *name) {
    struct benchfs_request request;
    initialize_request(req, &request, BENCHFS_RMDIR, parent);
    request.name = (const uint8_t *)name;
    request.name_length = strlen(name);
    dispatch(req, &request);
}

static void symlink_cb(fuse_req_t req, const char *target, fuse_ino_t parent,
                       const char *name) {
    struct benchfs_request request;
    initialize_request(req, &request, BENCHFS_SYMLINK, parent);
    request.name = (const uint8_t *)name;
    request.name_length = strlen(name);
    request.data = (const uint8_t *)target;
    request.data_length = strlen(target);
    dispatch(req, &request);
}

static void rename_cb(fuse_req_t req, fuse_ino_t old_parent,
                      const char *old_name, fuse_ino_t new_parent,
                      const char *new_name, unsigned int flags) {
    struct benchfs_request request;
    initialize_request(req, &request, BENCHFS_RENAME, old_parent);
    request.inode2 = new_parent;
    request.name = (const uint8_t *)old_name;
    request.name_length = strlen(old_name);
    request.name2 = (const uint8_t *)new_name;
    request.name2_length = strlen(new_name);
    request.flags = flags;
    dispatch(req, &request);
}

static void link_cb(fuse_req_t req, fuse_ino_t inode, fuse_ino_t new_parent,
                    const char *new_name) {
    struct benchfs_request request;
    initialize_request(req, &request, BENCHFS_LINK, inode);
    request.inode2 = new_parent;
    request.name = (const uint8_t *)new_name;
    request.name_length = strlen(new_name);
    dispatch(req, &request);
}

static void open_cb(fuse_req_t req, fuse_ino_t inode,
                    struct fuse_file_info *file) {
    struct benchfs_request request;
    initialize_request(req, &request, BENCHFS_OPEN, inode);
    request.flags = (uint32_t)file->flags;
    dispatch(req, &request);
}

static void read_cb(fuse_req_t req, fuse_ino_t inode, size_t size, off_t offset,
                    struct fuse_file_info *file) {
    struct benchfs_request request;
    initialize_request(req, &request, BENCHFS_READ, inode);
    request.handle = file->fh;
    request.offset = offset;
    request.size = size;
    dispatch(req, &request);
}

static void write_cb(fuse_req_t req, fuse_ino_t inode, const char *buffer,
                     size_t size, off_t offset, struct fuse_file_info *file) {
    struct benchfs_request request;
    initialize_request(req, &request, BENCHFS_WRITE, inode);
    request.handle = file->fh;
    request.offset = offset;
    request.data = (const uint8_t *)buffer;
    request.data_length = size;
    request.flags = (uint32_t)file->flags;
    dispatch(req, &request);
}

static void flush_cb(fuse_req_t req, fuse_ino_t inode,
                     struct fuse_file_info *file) {
    struct benchfs_request request;
    initialize_request(req, &request, BENCHFS_FLUSH, inode);
    request.handle = file->fh;
    dispatch(req, &request);
}

static void release_cb(fuse_req_t req, fuse_ino_t inode,
                       struct fuse_file_info *file) {
    struct benchfs_request request;
    initialize_request(req, &request, BENCHFS_RELEASE, inode);
    request.handle = file->fh;
    dispatch(req, &request);
}

static void fsync_cb(fuse_req_t req, fuse_ino_t inode, int data_only,
                     struct fuse_file_info *file) {
    struct benchfs_request request;
    initialize_request(req, &request, BENCHFS_FSYNC, inode);
    request.handle = file->fh;
    request.flags = data_only != 0;
    dispatch(req, &request);
}

static void opendir_cb(fuse_req_t req, fuse_ino_t inode,
                       struct fuse_file_info *file) {
    struct benchfs_request request;
    initialize_request(req, &request, BENCHFS_OPENDIR, inode);
    request.flags = (uint32_t)file->flags;
    dispatch(req, &request);
}

static void readdir_cb(fuse_req_t req, fuse_ino_t inode, size_t size,
                       off_t offset, struct fuse_file_info *file) {
    struct benchfs_request request;
    initialize_request(req, &request, BENCHFS_READDIR, inode);
    request.handle = file->fh;
    request.offset = offset;
    request.size = size;
    dispatch(req, &request);
}

static void releasedir_cb(fuse_req_t req, fuse_ino_t inode,
                          struct fuse_file_info *file) {
    struct benchfs_request request;
    initialize_request(req, &request, BENCHFS_RELEASEDIR, inode);
    request.handle = file->fh;
    dispatch(req, &request);
}

static void fsyncdir_cb(fuse_req_t req, fuse_ino_t inode, int data_only,
                        struct fuse_file_info *file) {
    struct benchfs_request request;
    initialize_request(req, &request, BENCHFS_FSYNCDIR, inode);
    request.handle = file->fh;
    /* data_only is intentionally ignored: the adapter normalizes
       directory fdatasync to directory fsync per SPEC §12.4. */
    (void)data_only;
    dispatch(req, &request);
}

static void statfs_cb(fuse_req_t req, fuse_ino_t inode) {
    struct benchfs_request request;
    initialize_request(req, &request, BENCHFS_STATFS, inode);
    dispatch(req, &request);
}

static void setxattr_cb(fuse_req_t req, fuse_ino_t inode, const char *name,
                        const char *value, size_t size, int flags) {
    struct benchfs_request request;
    initialize_request(req, &request, BENCHFS_SETXATTR, inode);
    request.name = (const uint8_t *)name;
    request.name_length = strlen(name);
    request.data = (const uint8_t *)value;
    request.data_length = size;
    request.flags = (uint32_t)flags;
    dispatch(req, &request);
}

static void getxattr_cb(fuse_req_t req, fuse_ino_t inode, const char *name,
                        size_t size) {
    struct benchfs_request request;
    initialize_request(req, &request, BENCHFS_GETXATTR, inode);
    request.name = (const uint8_t *)name;
    request.name_length = strlen(name);
    request.size = size;
    dispatch(req, &request);
}

static void listxattr_cb(fuse_req_t req, fuse_ino_t inode, size_t size) {
    struct benchfs_request request;
    initialize_request(req, &request, BENCHFS_LISTXATTR, inode);
    request.size = size;
    dispatch(req, &request);
}

static void removexattr_cb(fuse_req_t req, fuse_ino_t inode,
                           const char *name) {
    struct benchfs_request request;
    initialize_request(req, &request, BENCHFS_REMOVEXATTR, inode);
    request.name = (const uint8_t *)name;
    request.name_length = strlen(name);
    dispatch(req, &request);
}

static void create_cb(fuse_req_t req, fuse_ino_t parent, const char *name,
                      mode_t mode, struct fuse_file_info *file) {
    struct benchfs_request request;
    initialize_request(req, &request, BENCHFS_CREATE, parent);
    request.name = (const uint8_t *)name;
    request.name_length = strlen(name);
    request.mode = mode;
    request.flags = (uint32_t)file->flags;
    dispatch(req, &request);
}

static void fallocate_cb(fuse_req_t req, fuse_ino_t inode, int mode,
                         off_t offset, off_t length,
                         struct fuse_file_info *file) {
    struct benchfs_request request;
    initialize_request(req, &request, BENCHFS_FALLOCATE, inode);
    request.handle = file->fh;
    request.flags = (uint32_t)mode;
    request.offset = offset;
    request.offset2 = length;
    dispatch(req, &request);
}

static void lseek_cb(fuse_req_t req, fuse_ino_t inode, off_t offset, int whence,
                     struct fuse_file_info *file) {
    struct benchfs_request request;
    initialize_request(req, &request, BENCHFS_LSEEK, inode);
    request.handle = file->fh;
    request.offset = offset;
    request.flags = (uint32_t)whence;
    dispatch(req, &request);
}

static void bmap_cb(fuse_req_t req, fuse_ino_t inode, size_t block_size,
                    uint64_t index) {
    (void)inode;
    (void)block_size;
    (void)index;
    fuse_reply_err(req, ENOSYS);
}

static void ioctl_cb(fuse_req_t req, fuse_ino_t inode, unsigned int command,
                     void *argument, struct fuse_file_info *file,
                     unsigned int flags, const void *input, size_t input_size,
                     size_t output_size) {
    (void)inode;
    (void)argument;
    (void)file;
    (void)flags;
    (void)input;
    (void)input_size;
    (void)output_size;
    // SPEC §14.2 distinguishes the known-but-unsupported dedupe operation
    // from unknown ioctls. FIDEDUPERANGE reaches this callback on FUSE and
    // must therefore not fall through to the generic ENOTTY response.
    fuse_reply_err(req, command == FIDEDUPERANGE ? EOPNOTSUPP : ENOTTY);
}

static void poll_cb(fuse_req_t req, fuse_ino_t inode,
                    struct fuse_file_info *file,
                    struct fuse_pollhandle *poll_handle) {
    (void)inode;
    (void)file;
    if (poll_handle != NULL) {
        fuse_pollhandle_destroy(poll_handle);
    }
    fuse_reply_err(req, ENOSYS);
}

static void copy_file_range_cb(fuse_req_t req, fuse_ino_t input_inode,
                               off_t input_offset,
                               struct fuse_file_info *input_file,
                               fuse_ino_t output_inode, off_t output_offset,
                               struct fuse_file_info *output_file, size_t length,
                               int flags) {
    (void)input_inode;
    (void)input_offset;
    (void)input_file;
    (void)output_inode;
    (void)output_offset;
    (void)output_file;
    (void)length;
    (void)flags;
    fuse_reply_err(req, ENOSYS);
}

static void init_cb(void *userdata, struct fuse_conn_info *connection) {
    struct benchfs_session *session = userdata;
    bool required = true;
    fuse_unset_feature_flag(connection, FUSE_CAP_WRITEBACK_CACHE);
    fuse_unset_feature_flag(connection, FUSE_CAP_POSIX_LOCKS);
    fuse_unset_feature_flag(connection, FUSE_CAP_FLOCK_LOCKS);
    required &= fuse_set_feature_flag(connection, FUSE_CAP_HANDLE_KILLPRIV);
    fuse_unset_feature_flag(connection, FUSE_CAP_HANDLE_KILLPRIV_V2);
    fuse_unset_feature_flag(connection, FUSE_CAP_READDIRPLUS);
    fuse_unset_feature_flag(connection, FUSE_CAP_READDIRPLUS_AUTO);
    required &= fuse_set_feature_flag(connection, FUSE_CAP_ASYNC_READ);
    required &= fuse_set_feature_flag(connection, FUSE_CAP_ATOMIC_O_TRUNC);
    required &= fuse_set_feature_flag(connection, FUSE_CAP_DONT_MASK);
    required &= fuse_set_feature_flag(connection, FUSE_CAP_PARALLEL_DIROPS);
    fuse_unset_feature_flag(connection, FUSE_CAP_EXPLICIT_INVAL_DATA);
    connection->time_gran = 1;
    if (!required) {
        session->init_error = EOPNOTSUPP;
        fuse_session_exit(session->session);
    }
}

static const struct fuse_lowlevel_ops operations = {
    .init = init_cb,
    .lookup = lookup_cb,
    .forget = forget_cb,
    .getattr = getattr_cb,
    .setattr = setattr_cb,
    .readlink = readlink_cb,
    .mknod = mknod_cb,
    .mkdir = mkdir_cb,
    .unlink = unlink_cb,
    .rmdir = rmdir_cb,
    .symlink = symlink_cb,
    .rename = rename_cb,
    .link = link_cb,
    .open = open_cb,
    .read = read_cb,
    .write = write_cb,
    .flush = flush_cb,
    .release = release_cb,
    .fsync = fsync_cb,
    .opendir = opendir_cb,
    .readdir = readdir_cb,
    .releasedir = releasedir_cb,
    .fsyncdir = fsyncdir_cb,
    .statfs = statfs_cb,
    .setxattr = setxattr_cb,
    .getxattr = getxattr_cb,
    .listxattr = listxattr_cb,
    .removexattr = removexattr_cb,
    .create = create_cb,
    .bmap = bmap_cb,
    .ioctl = ioctl_cb,
    .poll = poll_cb,
    .forget_multi = forget_multi_cb,
    .fallocate = fallocate_cb,
    .copy_file_range = copy_file_range_cb,
    .lseek = lseek_cb,
};

int benchfs_fuse_run(const char *mountpoint, void *userdata,
                     benchfs_dispatch_fn dispatch_function,
                     benchfs_drain_fn drain_function) {
    // callers and the harness applies per-request privilege drops. A non-root
    // daemon turns harness-identity failures into filesystem EPERM and poisons
    // qualification data, so refuse loudly and record the reason (the line
    // lands in the caller's stderr / daemon log).
    if (geteuid() != 0) {
        fprintf(stderr, "benchfs serve requires euid=0 (non-root daemon refused)\n");
        return -EPERM;
    }
    // `allow_other` lets non-root callers reach the daemon; `default_permissions`
    // then has the kernel enforce POSIX permissions from the cached inode
    // attributes. `nodev`/`noatime` match the fixed BenchFS mount policy.
    struct fuse_args args = FUSE_ARGS_INIT(0, NULL);
    // `fuse_opt_add_arg` asserts the vector is heap-owned (`allocated`), so
    // the fixed options go through it as well instead of FUSE_ARGS_INIT on a
    // static array; `fuse_opt_free_args` after `fuse_session_new` releases
    // the copies it made.
    if (fuse_opt_add_arg(&args, "benchfs") != 0 ||
        fuse_opt_add_arg(&args, "-o") != 0 ||
        fuse_opt_add_arg(&args, "default_permissions,allow_other,nodev,noatime") != 0) {
        fuse_opt_free_args(&args);
        return -ENOMEM;
    }
    // xfstests's `_check_if_dev_already_mounted` matches the mount source
    // against TEST_DEV/SCRATCH_DEV. The FUSE mount source is the daemon's
    // fsname, so preserve the actual backing path exported by
    // mount.fuse.benchfs / eval.rs; otherwise keep libfuse's fixed argv[0]
    // behavior for callers that do not provide BENCHFS_FSNAME.
    const char *fsname = getenv("BENCHFS_FSNAME");
    if (fsname != NULL && *fsname != '\0') {
        // `fsname=` is a mount option: it must arrive via `-o`, otherwise
        // fuse_session_new rejects it as an unknown option.
        size_t option_length = strlen(fsname) + sizeof("-ofsname=");
        char *fsname_option = malloc(option_length);
        if (fsname_option == NULL) {
            fuse_opt_free_args(&args);
            return -ENOMEM;
        }
        snprintf(fsname_option, option_length, "-ofsname=%s", fsname);
        if (fuse_opt_add_arg(&args, fsname_option) != 0) {
            free(fsname_option);
            fuse_opt_free_args(&args);
            return -EINVAL;
        }
        free(fsname_option);
    }
    struct benchfs_session bridge_state = {
        .userdata = userdata,
        .dispatch = dispatch_function,
        .drain = drain_function,
        .session = NULL,
        .init_error = 0,
    };
    int result = -EIO;

    bridge_state.session = fuse_session_new(&args, &operations,
                                              sizeof(operations), &bridge_state);
    fuse_opt_free_args(&args);
    if (bridge_state.session == NULL) {
        return -EINVAL;
    }
    if (fuse_set_signal_handlers(bridge_state.session) != 0) {
        goto destroy;
    }
    if (fuse_session_mount(bridge_state.session, mountpoint) != 0) {
        goto signals;
    }
    struct fuse_loop_config config = {
        .clone_fd = 1,
        .max_idle_threads = 16,
    };
    result = fuse_session_loop_mt(bridge_state.session, &config);
    if (bridge_state.init_error != 0) {
        result = -bridge_state.init_error;
    }
    if (bridge_state.drain != NULL) {
        int drain_error = bridge_state.drain(bridge_state.userdata);
        if (drain_error != 0 && result == 0) {
            result = -drain_error;
        }
    }
    fuse_session_unmount(bridge_state.session);
signals:
    fuse_remove_signal_handlers(bridge_state.session);
destroy:
    fuse_session_destroy(bridge_state.session);
    return result;
}


static void fill_stat(struct stat *output, const struct benchfs_attr *input) {
    memset(output, 0, sizeof(*output));
    output->st_ino = input->inode;
    output->st_mode = input->mode;
    output->st_nlink = input->link_count;
    output->st_uid = input->uid;
    output->st_gid = input->gid;
    output->st_rdev = makedev(input->device_major, input->device_minor);
    output->st_size = (off_t)input->size;
    output->st_blksize = 4096;
    output->st_blocks = (blkcnt_t)(input->allocated_bytes / 512);
    output->st_atim.tv_sec = input->atime_seconds;
    output->st_atim.tv_nsec = input->atime_nanoseconds;
    output->st_mtim.tv_sec = input->mtime_seconds;
    output->st_mtim.tv_nsec = input->mtime_nanoseconds;
    output->st_ctim.tv_sec = input->ctime_seconds;
    output->st_ctim.tv_nsec = input->ctime_nanoseconds;
}

void benchfs_reply_error(void *request_handle, int error_number) {
    fuse_reply_err((fuse_req_t)request_handle, error_number);
}

void benchfs_reply_none(void *request_handle) {
    fuse_reply_none((fuse_req_t)request_handle);
}

void benchfs_reply_entry(void *request_handle,
                         const struct benchfs_attr *attribute) {
    struct fuse_entry_param entry;
    memset(&entry, 0, sizeof(entry));
    entry.ino = attribute->inode;
    entry.generation = attribute->generation;
    entry.attr_timeout = 1.0;
    entry.entry_timeout = 1.0;
    fill_stat(&entry.attr, attribute);
    fuse_reply_entry((fuse_req_t)request_handle, &entry);
}

void benchfs_reply_attribute(void *request_handle,
                             const struct benchfs_attr *attribute) {
    struct stat value;
    fill_stat(&value, attribute);
    fuse_reply_attr((fuse_req_t)request_handle, &value, 1.0);
}

void benchfs_reply_open(void *request_handle, uint64_t handle, int directory) {
    struct fuse_file_info file;
    memset(&file, 0, sizeof(file));
    file.fh = handle;
    file.keep_cache = directory == 0;
    file.cache_readdir = 0;
    fuse_reply_open((fuse_req_t)request_handle, &file);
}

void benchfs_reply_create(void *request_handle,
                          const struct benchfs_attr *attribute,
                          uint64_t handle) {
    struct fuse_entry_param entry;
    struct fuse_file_info file;
    memset(&entry, 0, sizeof(entry));
    memset(&file, 0, sizeof(file));
    entry.ino = attribute->inode;
    entry.generation = attribute->generation;
    entry.attr_timeout = 1.0;
    entry.entry_timeout = 1.0;
    fill_stat(&entry.attr, attribute);
    file.fh = handle;
    file.keep_cache = 1;
    fuse_reply_create((fuse_req_t)request_handle, &entry, &file);
}

void benchfs_reply_data(void *request_handle, const uint8_t *data,
                        size_t length) {
    fuse_reply_buf((fuse_req_t)request_handle, (const char *)data, length);
}

void benchfs_reply_readlink_bytes(void *request_handle, const uint8_t *data,
                                  size_t length) {
    char *target = malloc(length + 1);
    if (target == NULL) {
        fuse_reply_err((fuse_req_t)request_handle, ENOMEM);
        return;
    }
    memcpy(target, data, length);
    target[length] = '\0';
    fuse_reply_readlink((fuse_req_t)request_handle, target);
    free(target);
}

void benchfs_reply_write(void *request_handle, uint32_t count) {
    fuse_reply_write((fuse_req_t)request_handle, count);
}

void benchfs_reply_statfs_value(void *request_handle,
                                const struct benchfs_statfs *input) {
    struct statvfs output;
    memset(&output, 0, sizeof(output));
    output.f_bsize = input->block_size;
    output.f_frsize = input->fragment_size;
    output.f_blocks = input->blocks;
    output.f_bfree = input->blocks_free;
    output.f_bavail = input->blocks_available;
    output.f_files = input->files;
    output.f_ffree = input->files_free;
    output.f_favail = input->files_free;
    output.f_flag = input->read_only != 0 ? ST_RDONLY : 0;
    output.f_namemax = input->name_max;
    fuse_reply_statfs((fuse_req_t)request_handle, &output);
}

void benchfs_reply_xattr_size(void *request_handle, size_t length) {
    fuse_reply_xattr((fuse_req_t)request_handle, length);
}

void benchfs_reply_lseek_value(void *request_handle, int64_t offset) {
    fuse_reply_lseek((fuse_req_t)request_handle, (off_t)offset);
}

void benchfs_reply_readdir_entries(void *request_handle, size_t buffer_size,
                                   const struct benchfs_dirent *entries,
                                   size_t count) {
    fuse_req_t req = (fuse_req_t)request_handle;
    char *buffer = malloc(buffer_size == 0 ? 1 : buffer_size);
    size_t used = 0;
    if (buffer == NULL) {
        fuse_reply_err(req, ENOMEM);
        return;
    }
    for (size_t index = 0; index < count; ++index) {
        char *name = malloc(entries[index].name_length + 1);
        struct stat attribute;
        size_t needed;
        if (name == NULL) {
            free(buffer);
            fuse_reply_err(req, ENOMEM);
            return;
        }
        memcpy(name, entries[index].name, entries[index].name_length);
        name[entries[index].name_length] = '\0';
        memset(&attribute, 0, sizeof(attribute));
        attribute.st_ino = entries[index].inode;
        attribute.st_mode = entries[index].mode;
        needed = fuse_add_direntry(req, NULL, 0, name, &attribute,
                                   (off_t)entries[index].next_cookie);
        if (needed > buffer_size - used) {
            free(name);
            break;
        }
        fuse_add_direntry(req, buffer + used, buffer_size - used, name,
                          &attribute, (off_t)entries[index].next_cookie);
        used += needed;
        free(name);
    }
    fuse_reply_buf(req, buffer, used);
    free(buffer);
}
