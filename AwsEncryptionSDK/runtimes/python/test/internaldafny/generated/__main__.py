# Dafny program the_program compiled into Python
import sys
from typing import Callable, Any, TypeVar, NamedTuple
from math import floor
from itertools import count

import module_ as module_
import _dafny as _dafny

try:
    dafnyArgs = [_dafny.Seq(a) for a in sys.argv]
    module_.default__.Test____Main____(dafnyArgs)
except _dafny.HaltException as e:
    _dafny.print("[Program halted] " + e.message + "\n")
    sys.exit(1)
