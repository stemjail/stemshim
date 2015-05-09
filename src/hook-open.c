/*
 * Copyright (C) 2015 Mickaël Salaün
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3 of the License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details: <http://gnu.org/licenses/>.
 */

#define _GNU_SOURCE
#include <dlfcn.h>
#include <sys/types.h>

/* Do not include fcntl.h because of function redefinition. */
#include <asm-generic/fcntl.h>

#include "stemshim.h"

static void hook_read(const char *path)
{
	stemjail_request_access(path, false);
}

static void hook_write(const char *path)
{
	stemjail_request_access(path, true);
}

static void hook_dir(const char *path)
{
	stemjail_request_access(path, false);
}

static void hook_custom_open(const char *path, int flags)
{
	/* The O_RDONLY, O_WRONLY and O_RDWR are enums, not bitflags */
	stemjail_request_access(path, (flags & O_ACCMODE) == O_WRONLY || (flags & O_ACCMODE) == O_RDWR || flags & O_CREAT);
}

/* Check exported functions with "nm -D" */

#include "../gen/wrapper.c"
