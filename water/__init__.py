"""Python bindings for the water WebAssembly Text Format parser."""

try:
    # Import from the Rust extension module
    from _water import parse_module, parse_function

    __all__ = ["parse_module", "parse_function"]
except ImportError:
    # If the extension module hasn't been built yet
    __all__ = []
