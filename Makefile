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

ifndef DEBUG
TARGET = target/release
CARGO_RELEASE = --release
else
TARGET = target/debug
CARGO_RELEASE =
endif

.PHONY: all clean mrproper run

all: $(TARGET)/hook-open.so $(TARGET)/test-open

clean:
	rm src/hook-open.o || true
	rm -r ./gen || true

mrproper: clean
	rm $(TARGET)/hook-open.so || true
	rm $(TARGET)/test-open || true
	cargo clean

run: $(TARGET)/hook-open.so $(TARGET)/test-open
	LD_LIBRARY_PATH=./$(TARGET) LD_PRELOAD=./$(TARGET)/hook-open.so ./$(TARGET)/test-open

%.o: %.c
	$(CC) -c $(CFLAGS) -o $@ $<

$(TARGET)/libstemshim.so: src/lib.rs
	cargo build $(CARGO_RELEASE)

gen/wrapper.c: ./tools/gen-wrapper.py ./tools/libc.txt
	test -d ./gen || mkdir ./gen
	./$^ > $@

src/hook-open.o: gen/wrapper.c $(TARGET)/libstemshim.so

$(TARGET)/hook-open.so: src/hook-open.o $(TARGET)/libstemshim.so
	$(CC) -shared -ldl -L ./$(TARGET) -lstemshim $(LDFLAGS) -o $@ $^

$(TARGET)/test-open: tests/open.c
	test -d $(TARGET) || mkdir -p $(TARGET)
	$(CC) $^ $(CFLAGS) -o $@
