#!/usr/bin/env python
# -*- coding: utf-8 -*-

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

import re
import sys

func_re = re.compile(r"^(?P<ctype>.+[\s\*]+)(?P<name>[^\s]+)\s*\((?P<args>[^)]*)\)\s*(?P<hook>[^;]+;)")
args_re = re.compile(r"^(?P<ctype>.+[\s\*]+)(?P<name>.+)")

wrap_tmpl = """
typedef {0.func.ctype} (*{0.func.name}_t)({1});

{0.func.ctype} {0.func.name}({1})
{{
\t{0.func.name}_t {0.func.name}_next;
\tconst char *function_name = "{0.func.name}";
\t{0.hook}
\t{0.func.name}_next = ({0.func.name}_t)dlsym(RTLD_NEXT, function_name);
\treturn {0.func.name}_next({2});
}}
"""

class Entry(object):
    def __init__(self, name, ctype):
        self.name = name.strip()
        self.ctype = ctype.strip()

    def __str__(self):
        return self.ctype + " " + self.name

class Func(object):
    def __init__(self, line):
        m = func_re.match(line)
        if m:
            self.func = Entry(m.group("name"), m.group("ctype"))
            self.hook = m.group("hook")
            args_split = m.group("args").split(",")
            self.args = []
            for arg in args_split:
                m = args_re.match(arg)
                if m:
                    self.args += [Entry(m.group("name"), m.group("ctype"))]

    def _str_args_full(self):
        return ", ".join(str(a) for a in self.args)

    def _str_args_names(self):
        return ", ".join(str(a.name) for a in self.args)

    def __str__(self):
        return "{0}({1});".format(self.func, self._str_args_full())

    def get_wrapper(self):
        return wrap_tmpl.format(self, self._str_args_full(), self._str_args_names())

def main(func_list):
    funcs = []
    with open(func_list) as f:
        for line in f.readlines():
            if line != "\n":
                funcs += [Func(line)]
    print("\n".join(f.get_wrapper() for f in funcs))

if __name__ == '__main__':
    main(sys.argv[1])
