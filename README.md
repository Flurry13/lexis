# txtstat

**The ripgrep of text analysis.** A blazing-fast CLI tool for corpus-level NLP statistics, written in Rust.

`txtstat` replaces scattered Python scripts with a single binary that processes gigabytes of text in seconds. Pipe in files, directories, or stdin and get instant n-gram frequencies, vocabulary stats, readability scores, token counts, and more.

---

## Why txtstat?

NLP researchers and data scientists currently cobble together slow Python scripts for basic corpus analysis. There's no unified, fast CLI tool for the job.

- **Fast** — Built in Rust with parallel processing via `rayon`. Analyze multi-GB corpora in seconds, not minutes.
- **Composable** — Unix-friendly. Pipe text in, get structured output (JSON, CSV, or human-readable tables). Chain with `jq`, `awk`, or anything else.
- **Batteries included** — N-grams, perplexity, readability, token counts, entropy, language detection — all in one binary.
- **Zero setup** — `cargo install txtstat` and go. No Python environments, no dependency hell.
- **Multi-platform** — Available as a CLI binary, Python package (`pip install txtstat`), or npm/WASM module.

---

## Quick Start

```bash
# Install
cargo install txtstat

# Analyze a file
txtstat stats corpus.txt

# N-gram frequency (bigrams, top 20)
txtstat ngrams -n 2 --top 20 corpus.txt

# Readability scores
txtstat readability essay.txt

# Shannon entropy analysis
txtstat entropy corpus.txt

# N-gram language model perplexity
txtstat perplexity corpus.txt --smoothing laplace

# Detect language
txtstat lang mystery.txt

# BPE token counts for LLM cost estimation
txtstat tokens corpus.txt --model gpt4

# Zipf's law rank-frequency distribution
txtstat zipf corpus.txt --plot

# Pipe from stdin
cat *.txt | txtstat stats

# Process an entire directory recursively
txtstat stats ./documents/ --recursive

# Stream processing from stdin (emits results per chunk)
cat huge_corpus.txt | txtstat stats --stream --format json

# Generate shell completions
txtstat completions bash > ~/.bash_completions/txtstat

# Output as JSON for downstream processing
txtstat stats corpus.txt --format json | jq '.[0].value'
```

---

## Commands

### `txtstat stats`
Full corpus statistics at a glance.

```
$ txtstat stats prose.txt

  txtstat · prose.txt
┌─────────────────────┬────────────┐
│ Metric              ┆      Value │
╞═════════════════════╪════════════╡
│ Tokens (words)      ┆        175 │
│ Types (unique)      ┆         95 │
│ Characters          ┆        805 │
│ Sentences           ┆          6 │
│ Type-Token Ratio    ┆     0.5429 │
│ Hapax Legomena      ┆ 70 (73.7%) │
│ Avg Sentence Length ┆ 29.2 words │
└─────────────────────┴────────────┘
```

Options:
- `--stopwords <file|english>` — Filter stopwords (path to file, or `english` for built-in list)
- `--recursive` — Process directories recursively

### `txtstat ngrams`
N-gram frequency analysis with configurable N.

```
$ txtstat ngrams -n 2 --top 5 corpus.txt

  txtstat · corpus.txt
┌────────────┬──────┬───────┐
│ Bigram     ┆ Freq ┆ Rel % │
╞════════════╪══════╪═══════╡
│ "of the"   ┆    3 ┆ 1.72% │
│ "in the"   ┆    2 ┆ 1.15% │
│ "with a"   ┆    2 ┆ 1.15% │
│ "the air"  ┆    1 ┆ 0.57% │
│ "every"    ┆    1 ┆ 0.57% │
└────────────┴──────┴───────┘
```

Options:
- `-n <N>` — N-gram size (default: 1)
- `--top <K>` — Show top K results (default: 10)
- `--min-freq <F>` — Minimum frequency threshold
- `--case-insensitive` — Fold case before counting
- `--stopwords <file|english>` — Exclude stopwords

### `txtstat tokens`
Count tokens using various tokenization schemes, including BPE token counts for LLM cost estimation.

```
$ txtstat tokens prose.txt --model all

  txtstat · prose.txt
┌──────────────┬────────┐
│ Tokenizer    ┆ Tokens │
╞══════════════╪════════╡
│ Whitespace   ┆    126 │
│ Sentences    ┆      6 │
│ Characters   ┆    805 │
│ BPE (GPT-4)  ┆    150 │
│ BPE (GPT-4o) ┆    148 │
│ BPE (GPT-3)  ┆    151 │
└──────────────┴────────┘
```

Options:
- `--model <name>` — BPE tokenizer: `gpt4`, `gpt4o`, `gpt3`, `all` (omit for whitespace only)

### `txtstat readability`
Readability and complexity metrics.

```
$ txtstat readability prose.txt

  txtstat · prose.txt
┌──────────────────────┬───────┬─────────────┐
│ Metric               ┆ Score ┆       Grade │
╞══════════════════════╪═══════╪═════════════╡
│ Flesch-Kincaid Grade ┆ 12.73 ┆ High School │
│ Flesch Reading Ease  ┆ 41.16 ┆   Difficult │
│ Coleman-Liau Index   ┆ 13.82 ┆     College │
│ Gunning Fog Index    ┆ 16.97 ┆     College │
│ SMOG Index           ┆ 14.62 ┆     College │
└──────────────────────┴───────┴─────────────┘
```

### `txtstat entropy`
Information-theoretic analysis.

```
$ txtstat entropy prose.txt

  txtstat · prose.txt
┌──────────────────────┬─────────┐
│ Metric               ┆   Value │
╞══════════════════════╪═════════╡
│ H1 (Unigram Entropy) ┆  6.3184 │
│ H2 (Bigram Entropy)  ┆  6.9658 │
│ H3 (Trigram Entropy) ┆  6.9542 │
│ Entropy Rate         ┆ -0.0116 │
│ Vocabulary Size      ┆      95 │
│ Redundancy           ┆  1.0018 │
└──────────────────────┴─────────┘
```

### `txtstat zipf`
Zipf's law analysis — rank-frequency distribution with optional sparkline plot.

```
$ txtstat zipf corpus.txt --top 5

  txtstat · corpus.txt
┌──────┬──────┬───────────┐
│ Rank ┆ Word ┆ Frequency │
╞══════╪══════╪═══════════╡
│ 1    ┆  the ┆         8 │
│ 2    ┆   of ┆         6 │
│ 3    ┆   to ┆         4 │
│ 4    ┆ with ┆         4 │
│ 5    ┆every ┆         3 │
└──────┴──────┴───────────┘
```

Options:
- `--top <K>` — Show top K ranked words (default: 20)
- `--plot` — Show sparkline plot instead of rank table

### `txtstat perplexity`
N-gram language model perplexity with configurable smoothing.

```
$ txtstat perplexity prose.txt --smoothing laplace

  txtstat · prose.txt
┌─────────────────┬─────────────┐
│ Metric          ┆       Value │
╞═════════════════╪═════════════╡
│ Order           ┆           3 │
│ Vocabulary Size ┆          95 │
│ Unigrams        ┆          95 │
│ Bigrams         ┆         125 │
│ Trigrams        ┆         124 │
│ Smoothing       ┆ Add-k (k=1) │
│ Perplexity      ┆     48.1674 │
└─────────────────┴─────────────┘
```

Options:
- `-n, --order <N>` — N-gram order (default: 3)
- `--smoothing <method>` — `none`, `laplace`, `backoff` (default: laplace)
- `--k <K>` — Smoothing parameter for add-k (default: 1.0)

### `txtstat lang`
Language and script detection with confidence scoring.

```
$ txtstat lang prose.txt

  txtstat · prose.txt
┌────────────┬─────────┐
│ Metric     ┆   Value │
╞════════════╪═════════╡
│ Language   ┆ English │
│ Code       ┆     eng │
│ Script     ┆   Latin │
│ Confidence ┆  1.0000 │
│ Reliable   ┆     Yes │
└────────────┴─────────┘
```

### `txtstat completions`
Generate shell completions for bash, zsh, or fish.

```bash
# Bash
txtstat completions bash > ~/.bash_completions/txtstat

# Zsh
txtstat completions zsh > ~/.zsh/completions/_txtstat

# Fish
txtstat completions fish > ~/.config/fish/completions/txtstat.fish
```

---

## Streaming Mode

Use `--stream` to process stdin incrementally, emitting cumulative results after each chunk:

```bash
# Stream stats with 500-line chunks
cat huge_corpus.txt | txtstat stats --stream --chunk-lines 500 --format json
```

Supported commands: `stats`, `ngrams`, `entropy`. Output formats:
- **JSON** — JSON Lines (one object per chunk)
- **CSV** — Header once, rows per chunk
- **Table** — Table per chunk with chunk number

---

## Global Options

| Flag | Description |
|------|-------------|
| `--format <fmt>` | Output format: `table` (default), `json`, `csv` |
| `--recursive` | Process directories recursively |
| `--stream` | Process stdin as a continuous stream, emitting results per chunk |
| `--chunk-lines <N>` | Lines per chunk in streaming mode (default: 1000) |

---

## Installation

```bash
# CLI (from crates.io)
cargo install txtstat

# CLI (from source)
git clone https://github.com/Flurry13/txtstat
cd txtstat
cargo build --release

# Python
pip install txtstat

# JavaScript/WASM
npm install txtstat
```

### Python Usage

```python
import txtstat

result = txtstat.stats(text="hello world hello")
# {"tokens": 3, "types": 2, "sentences": 1, ...}

result = txtstat.ngrams("corpus.txt", n=2, top=10)
# [{"ngram": "of the", "frequency": 4521, "relative_pct": 2.09}, ...]

result = txtstat.lang(text="Bonjour le monde")
# {"language": "Français", "code": "fra", "script": "Latin", "confidence": 0.99}
```

### JavaScript/WASM Usage

```javascript
import { stats, ngrams, lang } from 'txtstat';

const result = stats("The quick brown fox...");
// { tokens: 5, types: 5, sentences: 1, ... }

const detected = lang("Bonjour le monde");
// { language: "Français", code: "fra", script: "Latin", confidence: 0.99 }
```

---

## Performance

Benchmarks on a 1GB English text corpus (Apple M2, 8 cores):

| Command | txtstat | Python (NLTK) | Speedup |
|---------|---------|---------------|---------|
| Word count | 0.8s | 34s | **42x** |
| Bigram freq | 1.2s | 89s | **74x** |
| Readability | 0.9s | 41s | **45x** |

*Benchmarks are targets — actual numbers will be validated during development.*

---

## Roadmap

### v0.1.0 — Core CLI
- [x] `stats` command (word/type/sentence counts, TTR, hapax)
- [x] `ngrams` command (configurable N, top-K, frequency thresholds)
- [x] `tokens` command (whitespace tokenization)
- [x] JSON/CSV/table output formats
- [x] Stdin and file input
- [x] Recursive directory processing

### v0.2.0 — Analysis
- [x] `readability` command (Flesch-Kincaid, Coleman-Liau, Gunning Fog, SMOG)
- [x] `entropy` command (unigram through trigram entropy)
- [x] `zipf` command with terminal plotting
- [x] Stopword filtering (`--stopwords english` or `--stopwords path/to/file`)
- [x] Case folding options
- [x] Parallel processing with `rayon`

### v0.3.0 — Language Models
- [x] `perplexity` command with n-gram LM training
- [x] Smoothing methods (Laplace, Stupid Backoff)
- [x] `lang` command for language detection
- [x] BPE token counting (GPT-3/GPT-4/GPT-4o tokenizers)

### v0.4.0 — Ecosystem
- [x] Python bindings via PyO3
- [x] WASM build for browser use
- [x] Streaming mode for very large files
- [x] Shell completions (bash, zsh, fish)

### Future
- [ ] Custom vocabulary / dictionary support
- [ ] Concordance / KWIC (keyword in context) search
- [ ] Colocation analysis (PMI, chi-squared)
- [ ] Sentiment lexicon scoring
- [ ] Diff mode: compare two corpora

---

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

```bash
# Run tests
cargo test

# Run benchmarks
cargo bench

# Lint
cargo clippy -- -D warnings
```

---

## License

MIT OR Apache-2.0 — your choice.

---

## Acknowledgments

Built on the shoulders of giants:
- [rayon](https://github.com/rayon-rs/rayon) — Data parallelism
- [clap](https://github.com/clap-rs/clap) — CLI argument parsing
- [comfy-table](https://github.com/nuber-io/comfy-table) — Beautiful terminal tables
- [unicode-segmentation](https://github.com/unicode-rs/unicode-segmentation) — Unicode text segmentation
- [whatlang](https://github.com/grstreten/whatlang-rs) — Language detection
- [tiktoken-rs](https://github.com/zurawiki/tiktoken-rs) — BPE tokenization for GPT models
- [PyO3](https://github.com/PyO3/pyo3) — Rust bindings for Python
- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) — Rust to WebAssembly interop

---

<p align="center">
  <strong>txtstat</strong> — because life's too short for slow text analysis.
</p>
