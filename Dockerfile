FROM ubuntu:noble

# Install the conda environment
RUN apt-get update && apt-get install build-essential cmake git curl -y
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:$PATH"

RUN git clone https://github.com/jiangweiyao/fastqcount.git
RUN cd /fastqcount && cargo install --path /fastqcount --locked
