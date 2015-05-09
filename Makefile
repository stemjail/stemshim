# Copyright (C) 2015 Mickaël Salaün
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Lesser General Public License as published by
# the Free Software Foundation, version 3 of the License.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Lesser General Public License for more details.
#
# You should have received a copy of the GNU Lesser General Public License
# along with this program. If not, see <http://www.gnu.org/licenses/>.

CC = gcc
CFLAGS = -Werror -Wall -Wextra -Wformat=2 -ansi -fPIC
TARGET = target/debug
LDFLAGS = -ldl -L ./$(TARGET) -l$(STEMSHIM_NAME)

STEMSHIM_NAME = $(shell find $(TARGET) -maxdepth 1 -name 'libstemshim-*.so' | sed -r 's,.*/lib(.*)\.so$$,\1,')

.PHONY: all clean mrproper run

all: $(TARGET)/hook-open.so $(TARGET)/test-open

clean:
	rm src/hook-open.o || true
	rm -r ./gen || true

mrproper: clean
	rm $(TARGET)/hook-open.so || true
	rm $(TARGET)/test-open || true
	cargo clean

run: $(TARGET)/build $(TARGET)/hook-open.so $(TARGET)/test-open
	LD_LIBRARY_PATH=./$(TARGET) LD_PRELOAD=./$(TARGET)/hook-open.so ./$(TARGET)/test-open

%.o: %.c
	$(CC) -c $(CFLAGS) -o $@ $<

$(TARGET)/build: src/lib.rs
	cargo build

gen/wrapper.c: ./tools/gen-wrapper.py ./tools/libc.txt
	test -d ./gen || mkdir ./gen
	./$^ > $@

src/hook-open.o: gen/wrapper.c $(TARGET)/build

$(TARGET)/hook-open.so: $(TARGET)/build src/hook-open.o
	$(CC) -shared $(LDFLAGS) -o $@ src/hook-open.o

$(TARGET)/test-open: $(TARGET)/build tests/open.c
	$(CC) tests/open.c -o $@