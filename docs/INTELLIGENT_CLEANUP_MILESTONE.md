# Intelligent cleanup milestone — 0.5.0

## Goal

Make optional AI cleanup reliably improve dictated text across local and cloud
providers without changing meaning, answering spoken questions, translating,
or inventing content. The pipeline remains provider-neutral and does not claim
to reproduce Fluid-1.

## Ordered phases

1. **Shared cleanup contract** — one language-aware prompt and validation path,
   with explicit conservative-editing rules and automatic-language behavior.
2. **Deterministic preprocessing** — handle spoken punctuation, paragraphs,
   self-correction markers, and safe whitespace locally before invoking AI.
3. **Multilingual fixtures** — cover English, Swedish, mixed-language speech,
   questions, technical text, false starts, and cases that must remain unchanged.
4. **Provider quality guidance** — show capability/speed/privacy guidance and
   warn when a small local model is unlikely to follow the editing contract.
5. **Observability and recovery** — record cleanup mode, latency, provider,
   fallback reason, and raw/final diffs without retaining extra private data.
6. **Release validation** — evaluate representative local and cloud providers,
   document expected limitations, and pass the complete release gate.

The first phase is complete when both streaming and non-streaming requests use
the same contract and tests prove fixed-language and automatic-language prompt
behavior.
