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

## Status

- Phase 1 complete: shared language-aware cleanup contract.
- Phase 2 complete: bounded, meaning-preserving whitespace and explicit spoken
  formatting preprocessing; no heuristic deletion or grammatical rewriting.
- Phase 3 complete: English, Swedish, mixed technical-language, automatic and
  fixed-language preservation fixtures.
- Phase 4 complete: local/cloud privacy, speed, memory, and expected-quality
  guidance is visible beside provider configuration.
- Phase 5 complete: backward-compatible History rows and exports identify the
  cleanup policy, language, provider, model, status, latency, raw text, final
  text, and fallback outcome.
- Phase 6 complete: protocol behavior is covered by isolated loopback fixtures,
  a real local Ollama/Qwen evaluation is recorded below, and the full release
  gate passes. Cloud quality is not claimed without user-supplied credentials.

## Provider evaluation — 2026-08-17

The installed local Ollama 0.32.14 server with `qwen2.5:7b` was evaluated using
synthetic text only. It correctly removed an English filler/self-correction,
punctuated a Swedish question, and added a question mark without answering the
question. It did not reliably preserve intentional Swedish/English
code-switching, even with an explicit fixed-language instruction. The GUI
therefore describes 7B as a baseline rather than a guarantee and warns that
less common and mixed languages may need a more capable model.

Cloud providers were validated through the existing isolated OpenAI-compatible
and Anthropic protocol fixtures, including streaming, bounded responses,
authentication, privacy locking, and failure fallback. No live cloud request
was made because release validation must not require or discover private API
credentials. Provider quality remains the provider's responsibility.
