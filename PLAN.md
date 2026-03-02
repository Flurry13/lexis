# txtstat - Technical Project Plan

## Overview

txtstat is a Rust CLI tool for fast, composable corpus-level text analysis. This document outlines the architecture, module design, dependencies, and implementation order.

---

## Architecture

```
txtstat
├── src/
│   ├── main.rs              # Entry point, CLI dispatch
│   ├── cli.rs               # Clap argument definitions
│   ├── input.rs             # Input handling (stdin, file, dir, glob)
│   ├── output.rs            # Output formatting (table, JSON, CSV)
│   ├── pipeline.rs          # Parallel processing pipeline
│   │
│   ├── commands/            # One module per subcommand
│   │   ├── mod.rs
│   │   ├── stats.rs         # Basic corpus statistics
│   │   ├── ngrams.rs        # N-gram frequency analysis
│   │   ├── tokens.rs        # Token counting
│   │   ├── readability.rs   # Readability metrics
│   │   ├── entropy.rs       # Information-theoretic stats
│   │   ├── perplexity.rs    # N-gram LM perplexity
│   │   ├── zipf.rs          # Zipf's law analysis
│   │   └── lang.rs          # Language detection
│   │
│   ├── analysis/            # Core computation modules
│   │   ├── mod.rs
│   │   ├── tokenizer.rs     # Word/sentence tokenization
│   │   ├── counter.rs       # Parallel frequency counting
│   │   ├── ngram.rs         # N-gram extraction iterator
│   │   ├── lm.rs            # N-gram language model (train/eval/smooth)
│   │   ├── readability.rs   # Readability formula implementations
│   │   ├── entropy.rs       # Entropy calculations
│   │   └── detect.rs        # Language detection
│   │
│   └── utils/
│       ├── mod.rs
│       ├── unicode.rs        # Unicode-aware text normalization
│       └── stopwords.rs      # Stopword list loading and filtering
│
├── benches/
│   └── benchmarks.rs         # Criterion benchmarks
│
├── tests/
│   ├── integration/          # End-to-end CLI tests
│   └── fixtures/             # Sample text files for testing
│
├── data/
│   └── stopwords/            # Built-in stopword lists
│
├── Cargo.toml
├── README.md
├── CONTRIBUTING.md
└── LICENSE-MIT / LICENSE-APACHE
```

---

## Core Dependencies

| Crate | Purpose | Why |
|-------|---------|-----|
| clap (v4, derive) | CLI parsing | Industry standard, great auto-generated help |
| rayon | Parallelism | Dead-simple data parallelism for chunk processing |
| serde + serde_json | Serialization | JSON output format |
| csv | CSV output | CSV output format |
| comfy-table | Table output | Pretty terminal tables |
| unicode-segmentation | Unicode tokenization | Proper word/sentence boundaries |
| indicatif | Progress bars | User feedback for large corpus processing |
| anyhow | Error handling | Ergonomic error propagation |
| memmap2 | Memory-mapped I/O | Fast large file reading |
| rustc-hash | Fast hashing | FxHashMap for ~2x faster string key hashing |

### Deferred Dependencies (v0.3+)

| Crate | Purpose |
|-------|---------|
| tokenizers (HuggingFace) | BPE/WordPiece token counting |
| pyo3 | Python bindings |
| whatlang | Language detection |

---

## Implementation Plan

### Phase 1: Foundation (v0.1.0) - ~2-3 weeks

**Goal:** Ship a working CLI that does word stats and n-grams fast.

**Week 1: Skeleton**
1. cargo init txtstat
2. Set up clap CLI with subcommand structure
3. Implement input.rs - read from file, stdin, or directory
4. Implement output.rs - table and JSON formatters
5. Write first integration test

**Week 2: Core Analysis**
1. analysis/tokenizer.rs - Unicode-aware word and sentence splitting
   - Use unicode-segmentation for word boundaries
   - Sentence splitting on .!? with abbreviation handling
2. analysis/counter.rs - Parallel frequency counting
   - Split input into chunks, count per-chunk with rayon, merge HashMaps
   - This is the performance-critical path - benchmark early
3. analysis/ngram.rs - N-gram extraction as an iterator
   - fn ngrams(tokens: &[&str], n: usize) -> impl Iterator

**Week 3: Commands**
1. commands/stats.rs - word count, type count, TTR, hapax legomena, avg sentence length
2. commands/ngrams.rs - top-K n-grams with frequency and relative %
3. commands/tokens.rs - whitespace tokenization counts
4. Memory-mapped file reading for large inputs
5. Benchmarks with Criterion against a ~500MB text file

**Ship v0.1.0** - publish to crates.io

---

### Phase 2: Analysis (v0.2.0) - ~2 weeks

**Goal:** Add readability, entropy, and Zipf analysis.

1. analysis/readability.rs - implement formulas:
   - Flesch-Kincaid Grade Level
   - Flesch Reading Ease
   - Coleman-Liau Index
   - Gunning Fog Index
   - SMOG Index
   - Key sub-problem: Syllable counting (rule-based vowel group approach)

2. analysis/entropy.rs:
   - H(X) = -sum p(x) * log2(p(x))
   - Compute for unigram, bigram, trigram distributions
   - Entropy rate estimation via sliding window

3. commands/zipf.rs:
   - Output rank-frequency pairs as CSV
   - Optional terminal sparkline using unicode block characters
   - Calculate Zipf exponent via least-squares fit on log-log data

4. Stopword filtering support

---

### Phase 3: Language Models (v0.3.0) - ~3 weeks

**Goal:** Perplexity, BPE token counting, and language detection.
This is where your CS 424 knowledge directly applies.

1. analysis/lm.rs - N-gram language model:
   - Training: build count tables from corpus
   - Smoothing implementations:
     - Laplace (add-1)
     - Kneser-Ney (interpolated) - the gold standard
     - Stupid Backoff - fast approximation for large corpora
   - Perplexity: PP(W) = P(w1...wN)^(-1/N)
   - Model serialization (save/load trained models as binary)

2. BPE token counting:
   - Integrate HuggingFace tokenizers crate
   - Support loading tokenizer configs for GPT-4, Llama, BERT
   - Report token counts and estimated API cost

3. analysis/detect.rs - Language detection:
   - Character n-gram profile comparison (simple and fast)
   - Or integrate whatlang crate for production quality

---

### Phase 4: Ecosystem (v0.4.0) - ~2-3 weeks

1. Python bindings via PyO3
2. Shell completions (bash, zsh, fish)
3. WASM build for browser use
4. Streaming mode for files larger than available RAM
5. Glob pattern support

---

## Performance Strategy

The main performance wins come from:

1. **Memory-mapped I/O** (memmap2) - avoid copying file contents into userspace
2. **Parallel chunk processing** (rayon) - split input, process in parallel, merge
3. **Efficient counting** - FxHashMap instead of std HashMap (~2x faster on string keys)
4. **Iterator-based n-grams** - zero-allocation n-gram windows using slice refs
5. **Avoid unnecessary allocations** - work with &str slices, only allocate for output

### Benchmark targets (1GB English text, 8 cores):
- Word count: < 1s
- Bigram frequencies: < 2s
- Full stats: < 2s
- Perplexity (trigram): < 3s

---

## Testing Strategy

- **Unit tests:** Each analysis module gets tests with known inputs/outputs
- **Integration tests:** Run the CLI binary against fixture files, assert output
- **Benchmark tests:** Criterion benchmarks for core paths, track regressions
- **Property tests:** Use proptest for unicode edge cases in tokenization

### Key test fixtures:
- fixtures/small.txt - 100 words, manually verified stats
- fixtures/moby-dick.txt - Real-world novel (~215K words)
- fixtures/multilingual.txt - Mixed-language text for unicode handling
- fixtures/empty.txt - Edge case
- fixtures/single-word.txt - Edge case

---

## Launch Strategy

1. README with benchmarks - the #1 driver of GitHub stars for CLI tools
2. Post on r/rust - the Rust community loves well-built CLI tools
3. Post on r/LanguageTechnology and r/NLP - target audience
4. Hacker News "Show HN" - performance-focused tools do well here
5. Blog post - "I replaced my Python NLP scripts with a single Rust binary"

---

## Open Questions

- Should txtstat support streaming input (line-by-line) or require full file reads?
  Recommendation: Both. Default to mmap for files, streaming for stdin.

- Should we bundle tokenizer model files or download on first use?
  Recommendation: Download on first use with txtstat init, cache in ~/.txtstat/

- How to handle very large n-gram tables that dont fit in memory?
  Recommendation: For v0.1, require they fit in RAM. For v0.4+, consider
  disk-backed counting (RocksDB or sorted temp files with merge).
