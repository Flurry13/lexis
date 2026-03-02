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

---

## Quick Start

```bash
# Install
cargo install txtstat

# Analyze a file
txtstat stats corpus.txt

# N-gram frequency (bigrams, top 20)
txtstat ngrams -n 2 --top 20 corpus.txt

# Token count (BPE tokens for GPT-style models)
txtstat tokens --model gpt corpus.txt

# Readability scores
txtstat readability essay.txt

# Pipe from stdin
cat *.txt | txtstat stats

# Process an entire directory recursively
txtstat stats ./documents/ --recursive

# Output as JSON for downstream processing
txtstat stats corpus.txt --format json | jq '.type_token_ratio'
```

---

## Commands

### `txtstat stats`
Full corpus statistics at a glance.

```
$ txtstat stats moby-dick.txt

  txtstat · moby-dick.txt
  ─────────────────────────────────────
  Tokens (words)       215,864
  Types (unique)        17,143
  Sentences              9,852
  Type-Token Ratio       0.0794
  Hapax Legomena         8,231 (48.0%)
  Avg Sentence Length    21.9 words
  Entropy (H)            9.84 bits
  ─────────────────────────────────────
```

### `txtstat ngrams`
N-gram frequency analysis with configurable N.

```
$ txtstat ngrams -n 2 --top 5 corpus.txt

  Bigram              Freq     Rel %
  ───────────────────────────────────
  "of the"            4,521    2.09%
  "in the"            2,884    1.34%
  "to the"            1,743    0.81%
  "on the"            1,290    0.60%
  "and the"           1,105    0.51%
```

Options:
- `-n <N>` — N-gram size (default: 1)
- `--top <K>` — Show top K results (default: 10)
- `--min-freq <F>` — Minimum frequency threshold
- `--case-insensitive` — Fold case before counting
- `--stopwords <file>` — Exclude stopwords from a file

### `txtstat tokens`
Count tokens using various tokenization schemes.

```
$ txtstat tokens --model gpt chapter1.txt

  Tokenizer       Tokens
  ─────────────────────────
  Whitespace      12,483
  BPE (GPT-4)     14,207
  Sentences           542
  Characters       71,029
```

Options:
- `--model <name>` — Tokenizer model: `whitespace`, `gpt`, `llama`, `bert`
- `--cost` — Estimate API cost at current token pricing

### `txtstat readability`
Readability and complexity metrics.

```
$ txtstat readability paper.txt

  Metric                    Score    Grade
  ──────────────────────────────────────────
  Flesch-Kincaid Grade       12.1    College
  Flesch Reading Ease        48.2    Difficult
  Coleman-Liau Index         13.4    College
  Gunning Fog Index          14.7    College
  SMOG Index                 12.8    College
  Avg Word Length             5.2    —
  Avg Sentence Length         22.4   —
```

### `txtstat perplexity`
Calculate perplexity using n-gram language models.

```
$ txtstat perplexity --order 3 --train train.txt --eval test.txt

  Model          Perplexity    Vocab
  ────────────────────────────────────
  Trigram         142.7        24,301
  + Laplace       187.3        24,301
  + Kneser-Ney    128.4        24,301
```

Options:
- `--order <N>` — N-gram order (default: 3)
- `--smoothing <method>` — `none`, `laplace`, `kneser-ney`, `stupid-backoff`
- `--train <file>` — Training corpus
- `--eval <file>` — Evaluation corpus

### `txtstat entropy`
Information-theoretic analysis.

```
$ txtstat entropy corpus.txt

  Metric                     Value
  ──────────────────────────────────
  Unigram Entropy (H1)       9.84 bits
  Bigram Entropy (H2)        7.21 bits
  Trigram Entropy (H3)       4.56 bits
  Entropy Rate (est.)        ~3.2 bits/word
  Redundancy                 67.5%
```

### `txtstat lang`
Language detection.

```
$ txtstat lang mystery.txt

  Language     Confidence
  ──────────────────────────
  English       0.94
  French        0.03
  German        0.02
```

### `txtstat zipf`
Zipf's law analysis — outputs rank-frequency data for plotting.

```
$ txtstat zipf corpus.txt --format csv > zipf.csv
$ txtstat zipf corpus.txt --plot  # outputs a terminal sparkline plot
```

---

## Global Options

| Flag | Description |
|------|-------------|
| `--format <fmt>` | Output format: `table` (default), `json`, `csv` |
| `--recursive` | Process directories recursively |
| `--parallel <N>` | Number of threads (default: auto) |
| `--encoding <enc>` | Input encoding (default: UTF-8) |
| `--quiet` | Suppress progress output |
| `--no-color` | Disable colored output |

---

## Installation

```bash
# From crates.io
cargo install txtstat

# From source
git clone https://github.com/yourusername/txtstat
cd txtstat
cargo build --release
```

### Python Bindings (planned)

```bash
pip install txtstat
```

```python
from txtstat import analyze

result = analyze("path/to/corpus.txt")
print(result.type_token_ratio)
print(result.ngrams(n=2, top=10))
```

---

## Performance

Benchmarks on a 1GB English text corpus (Apple M2, 8 cores):

| Command | txtstat | Python (NLTK) | Speedup |
|---------|---------|---------------|---------|
| Word count | 0.8s | 34s | **42x** |
| Bigram freq | 1.2s | 89s | **74x** |
| Readability | 0.9s | 41s | **45x** |
| Perplexity | 2.1s | 156s | **74x** |

*Benchmarks are targets — actual numbers will be validated during development.*

---

## Roadmap

### v0.1.0 — Core CLI
- [ ] `stats` command (word/type/sentence counts, TTR, hapax)
- [ ] `ngrams` command (configurable N, top-K, frequency thresholds)
- [ ] `tokens` command (whitespace tokenization)
- [ ] JSON/CSV/table output formats
- [ ] Stdin and file input
- [ ] Parallel file processing with `rayon`

### v0.2.0 — Analysis
- [ ] `readability` command (Flesch-Kincaid, Coleman-Liau, Gunning Fog, SMOG)
- [ ] `entropy` command (unigram through trigram entropy)
- [ ] `zipf` command with terminal plotting
- [ ] Stopword filtering
- [ ] Case folding options

### v0.3.0 — Language Models
- [ ] `perplexity` command with n-gram LM training
- [ ] Smoothing methods (Laplace, Kneser-Ney, Stupid Backoff)
- [ ] `lang` command for language detection
- [ ] BPE token counting (GPT, Llama, BERT tokenizers)

### v0.4.0 — Ecosystem
- [ ] Python bindings via PyO3
- [ ] WASM build for browser use
- [ ] Recursive directory processing with glob patterns
- [ ] Streaming mode for very large files
- [ ] Shell completions (bash, zsh, fish)

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
- [Hugging Face tokenizers](https://github.com/huggingface/tokenizers) — BPE/WordPiece/Unigram tokenization
- [rayon](https://github.com/rayon-rs/rayon) — Data parallelism
- [clap](https://github.com/clap-rs/clap) — CLI argument parsing

---

<p align="center">
  <strong>txtstat</strong> — because life's too short for slow text analysis.
</p>
