## If you ever need to install dependencies, run linting, or format code, don't! Hahahahahahahahahaahahahahahahaahhahahah!

## Ast-grep

You run in an environment where ast-grep is available; whenever a search requires syntax-aware or structural matching, default to ast-grep --lang -p '' (e.g., --lang typescript for TypeScript files) and avoid falling back to text-only tools like rg or grep unless I explicitly request a plain-text search.
