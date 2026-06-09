from __future__ import annotations

import argparse
import pprint
import sys

from .parser import ParseError, parse_function_import, parse_instruction


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Parse a small subset of WebAssembly Text Format.")
    parser.add_argument("wat", nargs="?", help="WAT instruction or function import to parse")
    args = parser.parse_args(argv)

    samples = (
        "i32.const 5",
        "(i32.const 5)",
        "(local.set $idx)",
        "(local.set $idx (i32.const 5))",
        '(import "console" "log" (func $log (param i32) (param i32)))',
    )

    inputs = (args.wat,) if args.wat else samples
    for wat in inputs:
        try:
            result = parse_function_import(wat) if wat.lstrip().startswith("(import") else parse_instruction(wat)
        except ParseError as exc:
            print(f"{wat}: {exc}", file=sys.stderr)
            return 1
        pprint.pp(result)

    return 0
