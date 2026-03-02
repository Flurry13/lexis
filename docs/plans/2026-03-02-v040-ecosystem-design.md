# txtstat v0.4.0 — Ecosystem Design

**Date:** 2026-03-02
**Status:** Approved
**Scope:** Shell completions, streaming mode, Python bindings (PyO3), WASM npm package

---

## 1. Shell Completions

**Approach:** `clap_complete` crate with a hidden `completions` subcommand.

```
txtstat completions bash > ~/.bash_completions/txtstat
txtstat completions zsh > ~/.zsh/completions/_txtstat
txtstat completions fish > ~/.config/fish/completions/txtstat.fish
```

**Implementation:**
- Add `clap_complete` dependency
- Add hidden `Completions { shell: Shell }` CLI variant
- Generate and print completions to stdout
- Trivial — no architectural changes

---

## 2. Streaming Mode

**Approach:** Global `--stream` flag with windowed processing on stdin.

**Supported commands:** `stats`, `ngrams`, `entropy` (additive/incremental counters).
**Unsupported:** `readability`, `perplexity`, `lang`, `zipf`, `tokens` — print error if `--stream` used.

### Design

- Read stdin line-by-line, buffer into chunks (configurable via `--chunk-lines`, default 1000).
- For n-grams: maintain overlap window of `n-1` tokens between chunks to avoid boundary misses.
- Maintain cumulative state: running frequency maps, token counts, entropy accumulators.
- Emit results after each chunk.

### Output format interaction

| `--format` | Streaming behavior |
|---|---|
| `table` | Print table per chunk (with chunk number in title) |
| `json` | JSON Lines — one JSON object per chunk |
| `csv` | Header once, then append rows per chunk |

### State management

```rust
pub struct StreamingState {
    word_freqs: FxHashMap<String, usize>,
    total_tokens: usize,
    total_types: usize,
    total_chars: usize,
    total_sentences: usize,
    overlap_tokens: Vec<String>,  // last n-1 tokens for ngram continuity
    chunk_count: usize,
}
```

Each chunk merges into cumulative state. Output reflects cumulative totals.

---

## 3. Python Bindings (PyO3)

**Approach:** Separate `txtstat-python/` crate in a Cargo workspace. Main crate stays at root.

### Workspace structure

```
txtstat/                    (workspace root)
├── Cargo.toml              (workspace manifest + current CLI/lib crate)
├── src/                    (unchanged — CLI binary + core lib)
├── txtstat-python/
│   ├── Cargo.toml          (depends on txtstat lib)
│   ├── pyproject.toml      (maturin build config)
│   └── src/lib.rs          (PyO3 module)
└── txtstat-wasm/
    ├── Cargo.toml
    ├── package.json
    └── src/lib.rs
```

### Python API

Each command exposed as a function returning a dict with typed values (not ResultTable strings):

```python
import txtstat

result = txtstat.stats("corpus.txt")
# {"tokens": 215864, "types": 17143, "sentences": 9852, ...}

result = txtstat.ngrams("corpus.txt", n=2, top=10)
# [{"ngram": "of the", "frequency": 4521, "relative_pct": 2.09}, ...]

result = txtstat.entropy("corpus.txt")
# {"h1": 9.84, "h2": 7.21, "h3": 4.56, "entropy_rate": 3.2, ...}

result = txtstat.readability("corpus.txt")
# {"flesch_kincaid_grade": 12.1, "flesch_reading_ease": 48.2, ...}

result = txtstat.perplexity("corpus.txt", order=3, smoothing="laplace")
# {"order": 3, "vocab_size": 24301, "perplexity": 142.7, ...}

result = txtstat.lang("corpus.txt")
# {"language": "English", "code": "eng", "script": "Latin", "confidence": 1.0}

result = txtstat.tokens("corpus.txt", model="gpt4")
# {"whitespace": 12483, "sentences": 542, "characters": 71029, "bpe_gpt4": 14207}

result = txtstat.zipf("corpus.txt", top=20)
# [{"rank": 1, "word": "the", "frequency": 8}, ...]
```

Functions also accept `text=` keyword for direct string input (no file):
```python
result = txtstat.stats(text="hello world hello")
```

### Implementation notes

- Create typed result structs in the core lib (e.g., `StatsResult`, `EntropyResult`) that command functions can also use
- PyO3 wrappers convert these structs to Python dicts via `IntoPy`
- File I/O handled in Python wrapper (reads file, passes text to Rust)
- Build with `maturin develop` / `maturin build`
- Published to PyPI as `txtstat`

---

## 4. WASM npm Package

**Approach:** Separate `txtstat-wasm/` crate in the workspace with feature-gated dependencies.

### Compatibility constraints

| Dependency | WASM Status | Solution |
|---|---|---|
| `tiktoken-rs` | Incompatible (runtime I/O) | Exclude — no BPE in WASM |
| `memmap2` | Incompatible (no filesystem) | Exclude — text passed as string |
| `comfy-table` | Incompatible (dropped WASM) | JSON-only output |
| `rayon` | Needs nightly + special flags | Use sequential path only |
| `whatlang` | Compatible | Include |
| `unicode-segmentation` | Compatible | Include |
| `rustc-hash` | Compatible | Include |

### Feature flags on core crate

```toml
[features]
default = ["native"]
native = ["memmap2", "rayon", "tiktoken-rs", "comfy-table", "csv"]
```

WASM crate depends on txtstat with `default-features = false`.

### JS API

```javascript
import { stats, ngrams, entropy, readability, perplexity, lang, zipf } from 'txtstat';

const result = stats("The quick brown fox...");
// { tokens: 5, types: 5, sentences: 1, ... }

const detected = lang("Bonjour le monde");
// { language: "Français", code: "fra", script: "Latin", confidence: 0.99 }
```

- All functions accept text strings directly (no file paths)
- Returns plain JS objects (via serde-wasm-bindgen)
- No table/CSV output — JSON only
- No BPE tokens command (tiktoken excluded)
- Sequential processing only (no rayon)
- Built with `wasm-pack build --target bundler`
- Published to npm

---

## 5. Execution Order

1. **Shell completions** — trivial, independent
2. **Feature flags + typed results** — restructure core crate for multi-target support
3. **Workspace migration** — add workspace manifest, move nothing
4. **Streaming mode** — pure Rust, uses typed results
5. **Python bindings** — PyO3 crate using typed results
6. **WASM build** — wasm-bindgen crate, hardest due to compat constraints

---

## 6. What's Excluded (Deferred)

- Kneser-Ney smoothing
- Model save/load for LM
- Interactive web demo
- CI/CD for PyPI/npm publishing (document manual process)
- `--cost` flag for token pricing estimation
- Llama/BERT tokenizers
