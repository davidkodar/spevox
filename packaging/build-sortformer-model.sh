#!/usr/bin/env bash
set -euo pipefail

# Maintainer-only reproducible model build. End users download the resulting
# verified GGUF and do not need Python, PyTorch, uv, or CUDA.
runtime_revision="9bc876635af36df537d9bc6d3f57ad1b76e4f74a"
model_revision="5240a64075176943f677d30fa2171c780229f341"
checkpoint_sha256="b371afce2c4958186469df33d939936b9746c89f38b10a69cfd2c61254e83329"
output_sha256="0679cfeb1ce356d0dea9470b31274f4bfc7eb927497d82005483770666da998a"
checkpoint_name="diar_streaming_sortformer_4spk-v2.nemo"
output_name="sortformer-v2-q8_0.gguf"

for tool in git curl uv sha256sum; do
    command -v "${tool}" >/dev/null || {
        echo "${tool} is required" >&2
        exit 1
    }
done

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
build_root="$(mktemp -d -t spevox-sortformer-build.XXXXXX)"
trap 'rm -rf -- "${build_root}"' EXIT

git clone --filter=blob:none --no-checkout https://github.com/NVIDIA/NeMo-Speech.cpp.git "${build_root}/nemo-speech"
git -C "${build_root}/nemo-speech" fetch --depth 1 origin "${runtime_revision}"
git -C "${build_root}/nemo-speech" checkout --detach "${runtime_revision}"

curl --fail --location --output "${build_root}/${checkpoint_name}" \
    "https://huggingface.co/nvidia/diar_streaming_sortformer_4spk-v2/resolve/${model_revision}/${checkpoint_name}"
printf '%s  %s\n' "${checkpoint_sha256}" "${build_root}/${checkpoint_name}" | sha256sum --check

uv venv "${build_root}/venv"
uv pip install --python "${build_root}/venv/bin/python" \
    --extra-index-url https://download.pytorch.org/whl/cpu \
    'torch==2.6.0+cpu' 'numpy==1.26.4' 'PyYAML==6.0.3' 'gguf==0.19.0'
"${build_root}/venv/bin/python" "${build_root}/nemo-speech/convert_model.py" \
    "${build_root}/${checkpoint_name}" \
    --outfile "${build_root}/${output_name}" \
    --outtype q8_0
printf '%s  %s\n' "${output_sha256}" "${build_root}/${output_name}" | sha256sum --check

mkdir -p "${repo_root}/target/package"
cp "${build_root}/${output_name}" "${repo_root}/target/package/${output_name}"
echo "Built ${repo_root}/target/package/${output_name}"
